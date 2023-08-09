use crate::state::{Config, IcaAmounts, CONFIG};
use cosmwasm_std::{Env, Response, StdError, Storage, Uint128};
use ica_tx::helpers::send_ica_tx;
use osmosis_std::shim::Duration;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgExitPool, MsgJoinSwapExternAmountIn, MsgSwapExactAmountIn,
};
use osmosis_std::types::osmosis::lockup::{MsgBeginUnlocking, MsgLockTokens};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use ununifi_binding::v0::binding::UnunifiMsg;
// use prost::EncodeError;
use crate::error::ContractError;
use crate::helpers::{
    begin_unlocking_msg_to_any, exit_pool_to_any, join_swap_extern_amount_in_to_any,
    lock_tokens_msg_to_any, swap_msg_to_any,
};
use crate::msgs::Phase;
use prost_types::Any;
use proto::cosmos::base::v1beta1::Coin as ProtoCoin;
use proto::ibc::applications::transfer::v1::MsgTransfer;
use proto::traits::MessageExt;

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
        let amount_to_return = config
            .controller_config
            .free_amount
            .checked_sub(config.controller_config.stacked_amount_to_deposit)
            .unwrap_or(Uint128::from(0u128));

        return IcaAmounts {
            to_swap_base: Uint128::from(0u128),
            to_swap_quote: config.host_config.free_base_amount,
            to_remove_lp: config.host_config.free_lp_amount,
            to_transfer_to_controller: config.host_config.free_base_amount,
            to_transfer_to_host: Uint128::from(0u128),
            to_return_amount: amount_to_return,
        };
    } else {
        return IcaAmounts {
            to_swap_base: config.host_config.free_base_amount / Uint128::from(2u128),
            to_swap_quote: config.host_config.free_quote_amount / Uint128::from(2u128),
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
            denom: config.host_config.base_denom,
            amount: to_transfer_to_controller.to_string(),
        }),
        sender: config.ica_account,
        receiver: env.contract.address.to_string(),
        timeout_height: None,
        timeout_timestamp: env.block.time.nanos() + config.transfer_timeout * 1000_000_000,
    };
    if let Ok(msg_any) = msg.to_any() {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "transfer_to_controller".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_bond_liquidity(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let share_out_amount = config.host_config.pending_bond_lp_amount;
    if share_out_amount.is_zero() {
        return Ok(Response::new());
    }

    let mut tokens_in: Vec<OsmosisCoin> = vec![
        OsmosisCoin {
            denom: config.host_config.quote_denom,
            amount: config.host_config.free_quote_amount.to_string(),
        },
        OsmosisCoin {
            denom: config.host_config.base_denom.to_string(),
            amount: config.host_config.free_base_amount.to_string(),
        },
    ];
    tokens_in.sort_by_key(|d| d.denom.to_string());

    let msg = MsgLockTokens {
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
    if let Ok(msg_any) = lock_tokens_msg_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "bond_lp_tokens".to_string(),
            vec![msg_any],
        )?);
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
            denom: config.host_config.quote_denom,
            amount: "1".to_string(),
        },
        OsmosisCoin {
            denom: config.host_config.base_denom.to_string(),
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
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "remove_liquidity".to_string(),
            vec![msg_any],
        )?);
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
    let to_swap_quote = ica_amounts.to_swap_quote;
    if to_swap_quote.is_zero() {
        return Ok(Response::new());
    }
    let msg = MsgSwapExactAmountIn {
        sender: config.ica_account.to_string(),
        token_in: Some(OsmosisCoin {
            denom: config.host_config.quote_denom,
            amount: to_swap_quote.to_string(),
        }),
        token_out_min_amount: "1".to_string(),
        routes: vec![SwapAmountInRoute {
            pool_id: config.host_config.pool_id,
            token_out_denom: config.host_config.base_denom,
        }],
    };
    if let Ok(msg_any) = swap_msg_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "swap_to_base".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_join_swap_extern_amount_in(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;

    let mut msgs: Vec<Any> = vec![];
    if !config.host_config.free_quote_amount.is_zero() {
        let msg = MsgJoinSwapExternAmountIn {
            sender: config.ica_account.to_string(),
            share_out_min_amount: "1".to_string(),
            pool_id: config.host_config.pool_id,
            token_in: Some(OsmosisCoin {
                denom: config.host_config.quote_denom,
                amount: config.host_config.free_quote_amount.to_string(),
            }),
        };
        if let Ok(msg_any) = join_swap_extern_amount_in_to_any(msg) {
            msgs.push(msg_any);
        }
    }
    if !config.host_config.free_base_amount.is_zero() {
        let msg = MsgJoinSwapExternAmountIn {
            sender: config.ica_account.to_string(),
            share_out_min_amount: "1".to_string(),
            pool_id: config.host_config.pool_id,
            token_in: Some(OsmosisCoin {
                denom: config.host_config.base_denom,
                amount: config.host_config.free_base_amount.to_string(),
            }),
        };
        if let Ok(msg_any) = join_swap_extern_amount_in_to_any(msg) {
            msgs.push(msg_any);
        }
    }

    if msgs.len() > 0 {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "join_swap_extern_amount_in".to_string(),
            msgs,
        )?);
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
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "begin_unbonding_lp".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}
