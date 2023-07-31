use crate::binding::UnunifiMsg;
use crate::helpers::send_ica_tx;
use crate::state::{Config, IcaAmounts, CONFIG, HOST_LP_RATE_MULTIPLIER};
use cosmwasm_std::{Env, Response, StdError, Storage, Uint128};
use osmosis_std::shim::Duration;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::gamm::v1beta1::{MsgExitPool, MsgJoinPool, MsgSwapExactAmountIn};
use osmosis_std::types::osmosis::lockup::{MsgBeginUnlocking, MsgLockTokens};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
// use prost::EncodeError;
use prost_types::Any;
use proto::cosmos::base::v1beta1::Coin as ProtoCoin;
use proto::ibc::applications::transfer::v1::MsgTransfer;
use proto::traits::MessageExt;
use strategy::error::ContractError;
use strategy_osmosis::msg::{join_pool_to_any, lock_tokens_msg_to_any, exit_pool_to_any, swap_msg_to_any, begin_unlocking_msg_to_any};
use strategy_osmosis::strategy::Phase;

// Regular epoch operation (once per day)
// - icq balance of ica account when `Deposit` phase
// Unbonding epoch operation
// - begin lp unbonding on host through ica tx per unbonding epoch - per day probably - (if to unbond lp is not enough, wait for icq to update bonded lp correctly)
// `Deposit` phase operations
// - This phase starts when `WithdrawToUser` phase ends
// - ibc transfer to host for newly incoming atoms
// - ibc transfer to host for stacked atoms during withdraw phases
// - swap half atom to osmo & half osmo to atom in a single ica tx
// - initiate and wait for icq to update latest balances
// - add liquidity & bond in a single ica tx
// - repeat the flow
// `DepositEnding` phase operations
// - This phase starts from `Deposit` phase, when ica free lp balance is positive
// - ibc transfers are disabled
// - swap half atom to osmo & half osmo to atom in a single ica tx
// - wait for icq to update latest balances
// - add liquidity & bond in a single ica tx
// - initiate and wait for icq to update latest balances
// - update to phase to `LqWithdraw`
// `Withdraw` phase operations
// - This phase starts when `DepositEnding` phase ends
// - Mark unbond ending queue items on contract
// - execute remove liquidity operation
// - initiate and wait or icq to update latest balances
// - swap full osmo to atom
// - initiate and wait or icq to update latest balances
// - ibc transfer full atom balance from ica to contract
// - wait for ica callback for ibc transfer finalization
// - calculate amount to return, contract balance - stacked atom balance for deposit
// - send amounts to marked unbond ending items proportionally
// - switch to `Deposit` phase
pub fn determine_ica_amounts(config: Config) -> IcaAmounts {
    if config.phase == Phase::Withdraw {
        let mut amount_to_return = Uint128::from(0u128);
        if config.controller_config.free_amount > config.controller_config.stacked_amount_to_deposit
        {
            amount_to_return = config.controller_config.free_amount
                - config.controller_config.stacked_amount_to_deposit;
        }
        return IcaAmounts {
            to_swap_atom: Uint128::from(0u128),
            to_swap_osmo: config.host_config.free_atom_amount,
            to_remove_lp: config.host_config.free_lp_amount,
            to_add_lp: Uint128::from(0u128),
            to_transfer_to_controller: config.host_config.free_atom_amount,
            to_transfer_to_host: Uint128::from(0u128),
            to_return_amount: amount_to_return,
        };
    } else {
        let to_add_lp = config.host_config.free_atom_amount
            * HOST_LP_RATE_MULTIPLIER
            * Uint128::from(2u128)
            * Uint128::from(9u128)
            / Uint128::from(10u128)
            / config.host_config.lp_redemption_rate;

        return IcaAmounts {
            to_swap_atom: config.host_config.free_atom_amount / Uint128::from(2u128),
            to_swap_osmo: config.host_config.free_osmo_amount / Uint128::from(2u128),
            to_add_lp: to_add_lp,
            to_remove_lp: Uint128::from(0u128),
            to_transfer_to_controller: Uint128::from(0u128),
            to_transfer_to_host: config.controller_config.free_amount,
            to_return_amount: Uint128::from(0u128),
        };
    }
}

pub fn execute_ibc_transfer_to_controller(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_transfer_to_controller = ica_amounts.to_transfer_to_controller;
    if to_transfer_to_controller.is_zero() {
        return Ok(Response::new());
    }
    let msg = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: config.host_config.transfer_channel_id,
        token: Some(ProtoCoin {
            denom: config.host_config.atom_denom,
            amount: to_transfer_to_controller.to_string(),
        }),
        sender: config.ica_account,
        receiver: env.contract.address.to_string(),
        timeout_height: None,
        timeout_timestamp: env.block.time.nanos() + config.transfer_timeout * 1000_000_000,
    };
    if let Ok(msg_any) = msg.to_any() {
        return send_ica_tx(
            store,
            env,
            "transfer_to_controller".to_string(),
            vec![msg_any],
        );
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_add_and_bond_liquidity(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = CONFIG.load(store)?;
    let share_out_amount = determine_ica_amounts(config.to_owned()).to_add_lp;
    if share_out_amount.is_zero() {
        return Ok(Response::new());
    }
    config.host_config.pending_bond_lp_amount = share_out_amount;
    CONFIG.save(store, &config)?;

    let mut tokens_in: Vec<OsmosisCoin> = vec![
        OsmosisCoin {
            denom: config.host_config.osmo_denom,
            amount: config.host_config.free_osmo_amount.to_string(),
        },
        OsmosisCoin {
            denom: config.host_config.atom_denom.to_string(),
            amount: config.host_config.free_atom_amount.to_string(),
        },
    ];
    tokens_in.sort_by_key(|d| d.denom.to_string());

    let msg1 = MsgJoinPool {
        sender: config.ica_account.to_string(),
        share_out_amount: share_out_amount.to_string(),
        pool_id: config.host_config.pool_id,
        token_in_maxs: tokens_in,
    };

    let msg2 = MsgLockTokens {
        owner: config.ica_account.to_string(),
        coins: vec![OsmosisCoin {
            denom: config.host_config.lp_denom,
            amount: share_out_amount.to_string(),
        }],
        duration: Some(Duration {
            seconds: config.unbond_period as i64,
            nanos: 0,
        }),
    };
    if let Ok(msg_any1) = join_pool_to_any(msg1) {
        if let Ok(msg_any2) = lock_tokens_msg_to_any(msg2) {
            return send_ica_tx(
                store,
                env,
                "add_and_bond_lp_tokens".to_string(),
                vec![msg_any1, msg_any2],
            );
        }
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_remove_liquidity(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_remove_lp = ica_amounts.to_remove_lp;
    if to_remove_lp.is_zero() {
        return Ok(Response::new());
    }

    config.host_config.pending_lp_removal_amount = to_remove_lp;
    CONFIG.save(store, &config)?;

    let mut tokens_out: Vec<OsmosisCoin> = vec![
        OsmosisCoin {
            denom: config.host_config.osmo_denom,
            amount: "1".to_string(),
        },
        OsmosisCoin {
            denom: config.host_config.atom_denom.to_string(),
            amount: "1".to_string(),
        },
    ];
    tokens_out.sort_by_key(|d| d.denom.to_string());

    let msg = MsgExitPool {
        sender: config.ica_account.to_string(),
        share_in_amount: to_remove_lp.to_string(),
        pool_id: config.host_config.pool_id,
        token_out_mins: tokens_out,
    };
    if let Ok(msg_any) = exit_pool_to_any(msg) {
        return send_ica_tx(store, env, "remove_liquidity".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_swap_two_tokens_to_deposit_token(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_swap_osmo = ica_amounts.to_swap_osmo;
    if to_swap_osmo.is_zero() {
        return Ok(Response::new());
    }
    let msg = MsgSwapExactAmountIn {
        sender: config.ica_account.to_string(),
        token_in: Some(OsmosisCoin {
            denom: config.host_config.osmo_denom,
            amount: to_swap_osmo.to_string(),
        }),
        token_out_min_amount: "1".to_string(),
        routes: vec![SwapAmountInRoute {
            pool_id: config.host_config.pool_id,
            token_out_denom: config.host_config.atom_denom,
        }],
    };
    if let Ok(msg_any) = swap_msg_to_any(msg) {
        return send_ica_tx(store, env, "swap_to_atom".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_swap_balance_to_two_tokens(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_swap_atom = ica_amounts.to_swap_atom;
    let to_swap_osmo = ica_amounts.to_swap_osmo;

    let mut msgs: Vec<Any> = vec![];
    if !to_swap_osmo.is_zero() {
        let msg = MsgSwapExactAmountIn {
            sender: config.ica_account.to_string(),
            token_in: Some(OsmosisCoin {
                denom: config.host_config.osmo_denom.to_string(),
                amount: to_swap_osmo.to_string(),
            }),
            token_out_min_amount: "1".to_string(),
            routes: vec![SwapAmountInRoute {
                pool_id: config.host_config.pool_id,
                token_out_denom: config.host_config.atom_denom.to_string(),
            }],
        };
        if let Ok(msg_any) = swap_msg_to_any(msg) {
            msgs.push(msg_any);
        }
    }

    if !to_swap_atom.is_zero() {
        let msg = MsgSwapExactAmountIn {
            sender: config.ica_account.to_string(),
            token_in: Some(OsmosisCoin {
                denom: config.host_config.atom_denom,
                amount: to_swap_atom.to_string(),
            }),
            token_out_min_amount: "1".to_string(),
            routes: vec![SwapAmountInRoute {
                pool_id: config.host_config.pool_id,
                token_out_denom: config.host_config.osmo_denom,
            }],
        };
        if let Ok(msg_any) = swap_msg_to_any(msg) {
            msgs.push(msg_any);
        }
    }
    if msgs.len() > 0 {
        return send_ica_tx(store, env, "swap_to_lp_underlyings".to_string(), msgs);
    }
    return Ok(Response::new());
}

pub fn execute_ica_begin_unbonding_lp_tokens(
    store: &mut dyn Storage,
    env: Env,
    unbonding_lp_amount: Uint128,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;
    if unbonding_lp_amount.is_zero() {
        return Ok(Response::new());
    }
    let msg = MsgBeginUnlocking {
        owner: config.ica_account.to_string(),
        id: config.host_config.lock_id,
        coins: vec![OsmosisCoin {
            denom: config.host_config.lp_denom,
            amount: unbonding_lp_amount.to_string(),
        }],
    };
    if let Ok(msg_any) = begin_unlocking_msg_to_any(msg) {
        return send_ica_tx(store, env, "begin_unbonding_lp".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}
