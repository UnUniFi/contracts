use crate::msg::{
    ChannelResponse, ExecuteMsg, FeeInfo, InstantiateMsg, ListChannelsResponse, MigrateMsg,
    QueryMsg,
};
use crate::state::{
    Config, DepositInfo, EpochCallSource, IcaAmounts, Phase, Unbonding, CHANNEL_INFO, CONFIG,
    DEPOSITS, UNBONDINGS,
};
use crate::state::{ControllerConfig, HostConfig, InterchainAccountPacketData};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, coins, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env,
    IbcMsg, IbcTimeout, MessageInfo, Order, Response, StdError, StdResult, Storage, Timestamp,
    Uint128,
};
use cw_utils::one_coin;
use osmosis_std::shim::Duration;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::gamm::poolmodels::balancer::v1beta1::{
    MsgCreateBalancerPool, MsgCreateBalancerPoolResponse,
};
use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgExitPool, MsgExitPoolResponse, MsgJoinPool, MsgJoinPoolResponse, MsgSwapExactAmountIn,
    MsgSwapExactAmountInResponse,
};
use osmosis_std::types::osmosis::lockup::{
    MsgBeginUnlocking,
    MsgBeginUnlockingAll,
    MsgBeginUnlockingAllResponse,
    MsgBeginUnlockingResponse,
    MsgLockTokens,
    MsgLockTokensResponse,
    // MsgSetRewardReceiverAddress, MsgSetRewardReceiverAddressResponse,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use prost::{EncodeError, Message};
use prost_types::Any;
use proto::cosmos::base::v1beta1::Coin as ProtoCoin;
use proto::cosmos::staking::v1beta1::MsgDelegate;
use proto::ibc::applications::interchain_accounts::v1::CosmosTx;
use proto::ibc::applications::transfer::v1::MsgTransfer;
use proto::traits::MessageExt;
use proto::traits::TypeUrl;
use strategy::error::ContractError;

fn join_pool_to_any(msg: MsgJoinPool) -> Result<Any, EncodeError> {
    return msg.to_bytes().map(|bytes| Any {
        type_url: "/osmosis.gamm.v1beta1.MsgJoinPool".to_owned(),
        value: bytes,
    });
}

fn exit_pool_to_any(msg: MsgExitPool) -> Result<Any, EncodeError> {
    return msg.to_bytes().map(|bytes| Any {
        type_url: "/osmosis.gamm.v1beta1.MsgExitPool".to_owned(),
        value: bytes,
    });
}

fn swap_msg_to_any(msg: MsgSwapExactAmountIn) -> Result<Any, EncodeError> {
    return msg.to_bytes().map(|bytes| Any {
        type_url: "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn".to_owned(),
        value: bytes,
    });
}

fn lock_tokens_msg_to_any(msg: MsgLockTokens) -> Result<Any, EncodeError> {
    return msg.to_bytes().map(|bytes| Any {
        type_url: "/osmosis.lockup.MsgLockTokens".to_owned(),
        value: bytes,
    });
}

fn begin_unlocking_msg_to_any(msg: MsgBeginUnlocking) -> Result<Any, EncodeError> {
    return msg.to_bytes().map(|bytes| Any {
        type_url: "/osmosis.lockup.MsgBeginUnlocking".to_owned(),
        value: bytes,
    });
}

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let redemption_rate_multiplier = Uint128::from(1000000u128); // 10^6
    let config = Config {
        owner: info.sender,
        unbond_period: msg.unbond_period,
        total_deposit: Uint128::from(0u128),
        last_unbonding_id: 1u64,
        redemption_rate: redemption_rate_multiplier,
        total_withdrawn: Uint128::from(0u128),
        transfer_timeout: 300, // 300s
        ica_channel_id: "".to_string(),
        ica_account: "".to_string(),
        phase: Phase::Deposit,
        phase_step: 1u64,
        host_config: HostConfig {
            transfer_channel_id: "".to_string(),
            lp_redemption_rate: Uint128::from(2000u128),
            lp_denom: "gamm/pool/1".to_string(), // ATOM-OSMO
            bonded_lp_amount: Uint128::from(0u128),
            unbonding_lp_amount: Uint128::from(0u128),
            free_lp_amount: Uint128::from(0u128),
            pending_bond_lp_amount: Uint128::from(0u128),
            pending_unbond_lp_amount: Uint128::from(0u128),
            pending_swap_lp_amount: Uint128::from(0u128), // pending swap from lp to deposit token amount
            osmo_denom: "uosmo".to_string(),              // OSMO
            free_osmo_amount: Uint128::from(0u128),
            pending_swap_to_atom_amount: Uint128::from(0u128), // Convert OSMO to ATOM
            atom_denom: "stake".to_string(),                   // ATOM
            free_atom_amount: Uint128::from(0u128),            // free ATOM balance
            pending_swap_to_osmo_amount: Uint128::from(0u128), // pending swap from ATOM -> OSMO to add liquidity
            pending_add_liquidity_amount: Uint128::from(0u128), // amount of ATOM used on liquidity addition
            pending_transfer_amount: Uint128::from(0u128), // pending transfer to controller - TODO: how to get hook for transfer finalization?
            required_withdrawal_amount: Uint128::from(0u128),
        },
        controller_config: ControllerConfig {
            transfer_channel_id: "channel-1".to_string(),
            deposit_denom: "stake".to_string(), // `ibc/xxxxuatom`
            free_amount: Uint128::from(0u128),
            pending_transfer_amount: Uint128::from(0u128), // TODO: where to get hook for transfer finalization?
            stacked_amount_to_deposit: Uint128::from(0u128), // TODO: to be set to 0 when deposit happens at `Deposit` phase
        },
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

//Execute the handle messages.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            owner,
            unbond_period,
            deposit_denom,
            lp_redemption_rate,
        } => execute_update_config(
            deps,
            env,
            info,
            owner,
            unbond_period,
            deposit_denom,
            lp_redemption_rate,
        ),
        ExecuteMsg::Stake(_) => {
            let coin: Coin = one_coin(&info)?;
            execute_stake(deps, env, coin, info.sender)
        }
        ExecuteMsg::Unstake(msg) => execute_unstake(deps, msg.amount, info.sender),
        ExecuteMsg::ExecuteEpoch(_) => execute_epoch(deps, env, EpochCallSource::NormalEpoch, true),
        ExecuteMsg::IbcTransferToHost(_) => execute_ibc_transfer_to_host(deps.storage, env),
        ExecuteMsg::IbcTransferToController(_) => {
            execute_ibc_transfer_to_controller(deps.storage, env)
        }
        ExecuteMsg::IcaAddAndBondLiquidity(_) => {
            execute_ica_add_and_bond_liquidity(deps.storage, env)
        }
        ExecuteMsg::IcaRemoveLiquidity(_) => execute_ica_remove_liquidity(deps.storage, env),
        ExecuteMsg::IcaSwapRewardsToTwoTokens(_) => {
            execute_ica_swap_rewards_to_two_tokens(deps.storage, env)
        }
        ExecuteMsg::IcaSwapTwoTokensToDepositToken(_) => {
            execute_ica_swap_two_tokens_to_deposit_token(deps.storage, env)
        }
        ExecuteMsg::IcaSwapDepositTokenToTwoTokens(_) => {
            execute_ica_swap_deposit_token_to_two_tokens(deps.storage, env)
        }
        ExecuteMsg::IcaBeginUnbondLpTokens(msg) => {
            execute_ica_begin_unbonding_lp_tokens(deps.storage, env, msg.unbonding_lp_amount)
        }
        ExecuteMsg::StoreIcaUnlockedBalances(msg) => {
            execute_store_ica_unlocked_balances(deps, env, msg.coins)
        }
    }
}

/// Only owner can execute it.
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<String>,
    unbond_period: Option<u64>,
    deposit_denom: Option<String>,
    lp_redemption_rate: Option<Uint128>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }
    if let Some(unbond_period) = unbond_period {
        config.unbond_period = unbond_period;
    }
    if let Some(deposit_denom) = deposit_denom {
        config.controller_config.deposit_denom = deposit_denom;
    }
    if let Some(lp_redemption_rate) = lp_redemption_rate {
        config.host_config.lp_redemption_rate = lp_redemption_rate;
    }

    CONFIG.save(deps.storage, &config)?;
    let resp = Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("owner", config.owner.to_string())
        .add_attribute("unbond_period", config.unbond_period.to_string())
        .add_attribute(
            "lp_redemption_rate",
            config.host_config.lp_redemption_rate.to_string(),
        )
        .add_attribute(
            "deposit_denom",
            config.controller_config.deposit_denom.to_string(),
        );

    Ok(resp)
}

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
    // TODO: to_unbond_lp if it's unbonding epoch
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
            to_unbond_lp: Uint128::from(0u128),
            to_transfer_to_controller: config.host_config.free_atom_amount,
            to_transfer_to_host: Uint128::from(0u128),
            to_return_amount: amount_to_return,
        };
    } else {
        let lp_redemption_rate_multiplier = Uint128::from(1000000_000000_000000u128); // 10^18
        let to_add_lp = config.host_config.free_atom_amount
            * lp_redemption_rate_multiplier
            * Uint128::from(2u128)
            * Uint128::from(9u128)
            / Uint128::from(10u128)
            / config.host_config.lp_redemption_rate;

        let to_transfer_to_controller = config.controller_config.free_amount;

        return IcaAmounts {
            to_swap_atom: config.host_config.free_atom_amount / Uint128::from(2u128),
            to_swap_osmo: config.host_config.free_osmo_amount / Uint128::from(2u128),
            to_add_lp: to_add_lp,
            to_remove_lp: Uint128::from(0u128),
            to_unbond_lp: Uint128::from(0u128),
            to_transfer_to_controller: to_transfer_to_controller,
            to_transfer_to_host: Uint128::from(0u128),
            to_return_amount: Uint128::from(0u128),
        };
    }
}

pub fn execute_stake(
    deps: DepsMut,
    env: Env,
    coin: Coin,
    sender: Addr,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.controller_config.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let amount = coin.amount;
    let redemption_rate_multiplier = Uint128::from(1000000_000000_000000u128); // 10^18
    let redemption_rate = config.host_config.lp_redemption_rate;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let stake_amount = amount * redemption_rate_multiplier / redemption_rate;
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_add(stake_amount)?,
                });
            }
            Ok(DepositInfo {
                sender: sender.clone(),
                amount: amount,
            })
        },
    )?;
    config.total_deposit += amount;
    CONFIG.save(deps.storage, &config)?;

    let rsp = Response::default()
        .add_attribute("action", "stake")
        .add_attribute("sender", sender)
        .add_attribute("amount", amount);
    Ok(rsp)
}

// pub fn query_balance(
//     querier: &QuerierWrapper,
//     account_addr: Addr,
//     denom: String,
// ) -> StdResult<Uint128> {
//     // load price form the oracle
//     let balance: BalanceResponse = querier.query(&QueryRequest::Bank(BankQuery::Balance {
//         address: account_addr.to_string(),
//         denom,
//     }))?;
//     Ok(balance.amount.amount)
// }
// let balance = query_balance(&deps.querier, env.contract.address, denom.to_string())?;
// if balance.is_zero() {
//     return Err(StdError::generic_err(
//         "a balance greater than zero is required by the factory for verification",
//     ));
// }

// Submit the ICQ for the withdrawal account balance
pub fn submit_icq_for_host(store: &dyn Storage, env: Env) -> Result<Response, ContractError> {
    // TODO: query balace of ica account
    // Note: bonded lp and unbonding lp token balance could be managed without icq on contract side
    // TODO: query bonded lp token balance of ica account
    // TODO: query unbonding lp token balance of ica account
    return Ok(Response::new());
}

pub fn execute_epoch(
    deps: DepsMut,
    env: Env,
    called_from: EpochCallSource,
    success: bool,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    let mut rsp: Result<Response, ContractError> = Ok(Response::new());
    if config.phase == Phase::Withdraw {
        if config.phase_step == 1u64 {
            // - Mark unbond ending queue items on contract
            // assumption: matured unbondings on the contract is same as matured unbondings on controller chain
            let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
            for mut unbonding in unbondings {
                if unbonding.start_time + config.unbond_period < env.block.time.seconds() {
                    unbonding.marked = true;
                    UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                }
            }
            // - execute remove liquidity operation
            rsp = execute_ica_remove_liquidity(deps.storage, env);
            config.phase_step += 1;
        } else if config.phase_step == 2u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                config.phase_step += 1;
            }
        } else if config.phase_step == 3u64 {
            // - initiate and wait or icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            config.phase_step += 1;
        } else if config.phase_step == 4u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                config.phase_step += 1;
            }
        } else if config.phase_step == 5u64 {
            // - swap full osmo to atom
            rsp = execute_ica_swap_two_tokens_to_deposit_token(deps.storage, env);
            config.phase_step += 1;
        } else if config.phase_step == 6u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                config.phase_step += 1;
            }
        } else if config.phase_step == 7u64 {
            // - initiate and wait or icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            config.phase_step += 1;
        } else if config.phase_step == 8u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                config.phase_step += 1;
            }
        } else if config.phase_step == 9u64 {
            // - ibc transfer full atom balance from ica to contract
            rsp = execute_ibc_transfer_to_controller(deps.storage, env);
            config.phase_step += 1;
        } else if config.phase_step == 10u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                config.phase_step += 1;
            }
        } else if config.phase_step == 11u64 {
            // - refresh balance of host chain after ibc transfer callback
            rsp = submit_icq_for_host(deps.storage, env);
            config.phase_step += 1;
        } else if config.phase_step == 12u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                config.phase_step += 1;
            }
        } else {
            // 13u64
            // - calculate amount to return, contract balance - stacked atom balance for deposit
            let amount_to_return = config.controller_config.free_amount
                - config.controller_config.stacked_amount_to_deposit;
            // - send amounts to marked unbond ending items proportionally
            let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
            let mut total_marked_lp_amount = Uint128::from(0u128);
            for unbonding in unbondings.as_slice() {
                if unbonding.marked {
                    total_marked_lp_amount += unbonding.amount;
                }
            }
            if !total_marked_lp_amount.is_zero() {
                let mut resp: Response = Response::new();
                for unbonding in unbondings {
                    if unbonding.marked {
                        let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
                            to_address: unbonding.sender.to_string(),
                            amount: coins(
                                (amount_to_return * unbonding.amount / total_marked_lp_amount)
                                    .u128(),
                                &config.controller_config.deposit_denom,
                            ),
                        });
                        resp = resp.add_message(bank_send_msg);
                        UNBONDINGS.remove(deps.storage, unbonding.id);
                    }
                }
                rsp = Ok(resp);
            }
            // - switch to `Deposit` phase
            config.phase = Phase::Deposit;
            CONFIG.save(deps.storage, &config)?;
        }
    } else {
        if config.phase_step == 1u64 {
            // - ibc transfer to host for newly incoming atoms
            // - ibc transfer to host for stacked atoms during withdraw phases
            rsp = execute_ibc_transfer_to_host(deps.storage, env);
            // TODO: if nothing transferred increase step by +2
            config.phase_step += 1;
        } else if config.phase_step == 2u64 {
            // do nothing - waiting for transfer callback
        } else if config.phase_step == 3u64 {
            // - icq balance of ica account when `Deposit` phase
            rsp = submit_icq_for_host(deps.storage, env);
            config.phase_step += 1;
        } else if config.phase_step == 4u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                config.phase_step += 1;
            }
        } else if config.phase_step == 5u64 {
            // - swap half atom to osmo & half osmo to atom in a single ica tx
            rsp = execute_ica_swap_deposit_token_to_two_tokens(deps.storage, env);
            // TODO: if nothing transferred increase step by +2
            config.phase_step += 1;
        } else if config.phase_step == 6u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                config.phase_step += 1;
            }
        } else if config.phase_step == 7u64 {
            // - initiate and wait for icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            config.phase_step += 1;
        } else if config.phase_step == 8u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                config.phase_step += 1;
            }
        } else if config.phase_step == 9u64 {
            // - add liquidity & bond in a single ica tx
            rsp = execute_ica_add_and_bond_liquidity(deps.storage, env);
            // TODO: if nothing transferred increase step by +2
            config.phase_step += 1;
        } else if config.phase_step == 8u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                config.phase_step += 1;
            }
        } else if config.phase_step == 9u64 {
            // - initiate and wait for icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            config.phase_step += 1;
        } else if config.phase_step == 10u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                config.phase_step += 1;
            }
        } else if config.phase_step == 11u64 {
            // Unbonding epoch operation
            // - begin lp unbonding on host through ica tx per unbonding epoch - per day probably - (if to unbond lp is not enough, wait for icq to update bonded lp correctly)
            let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
            let mut unbonding_lp_amount = Uint128::from(0u128);
            for mut unbonding in unbondings {
                if unbonding.start_time != 0 {
                    break;
                }
                unbonding.start_time = env.block.time.seconds();
                UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                unbonding_lp_amount += unbonding.amount;
            }

            if !unbonding_lp_amount.is_zero() {
                config.host_config.unbonding_lp_amount += unbonding_lp_amount;
                CONFIG.save(deps.storage, &config)?;
                rsp = execute_ica_begin_unbonding_lp_tokens(deps.storage, env, unbonding_lp_amount);
                config.phase_step += 1;
            } else {
                config.phase_step += 2;
            }
        } else if config.phase_step == 12u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                config.phase_step += 1;
            }
        } else {
            // 13u64
            if !config.host_config.free_lp_amount.is_zero() {
                config.phase = Phase::Withdraw;
            }
            config.phase_step = 1u64;
        }
    }
    CONFIG.save(deps.storage, &config)?;
    return rsp;
}

pub fn execute_unstake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    let lp_redemption_rate_multiplier = Uint128::from(1000000_000000_000000u128); // 10^18
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let unstake_amount =
                    amount * lp_redemption_rate_multiplier / config.host_config.lp_redemption_rate;
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_sub(unstake_amount)?,
                });
            }
            Ok(DepositInfo {
                sender: sender.clone(),
                amount: amount,
            })
        },
    )?;

    let unbonding = &Unbonding {
        id: config.last_unbonding_id + 1,
        sender: sender.to_owned(),
        amount: amount * lp_redemption_rate_multiplier / config.host_config.lp_redemption_rate,
        start_time: 0u64,
        marked: false,
    };
    UNBONDINGS.save(deps.storage, unbonding.id, unbonding)?;
    config.host_config.unbonding_lp_amount += amount;
    CONFIG.save(deps.storage, &config)?;

    let rsp = Response::new()
        .add_attribute("sender", sender.to_string())
        .add_attribute("amount", amount);
    Ok(rsp)
}

pub fn send_ica_tx(
    store: &dyn Storage,
    env: Env,
    action: String,
    msgs: Vec<Any>,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let cosmos_tx = CosmosTx { messages: msgs };
    let mut cosmos_tx_buf = vec![];
    cosmos_tx.encode(&mut cosmos_tx_buf).unwrap();

    let ibc_packet = InterchainAccountPacketData {
        r#type: 1,
        data: cosmos_tx_buf,
        memo: action.to_string(),
    };

    let timeout = env.block.time.plus_seconds(config.transfer_timeout);
    let ibc_msg = IbcMsg::SendPacket {
        channel_id: config.ica_channel_id,
        data: to_binary(&ibc_packet)?,
        timeout: IbcTimeout::from(timeout),
    };

    let res = Response::new()
        .add_message(ibc_msg)
        .add_attribute("action", action.to_string());
    return Ok(res);
}

pub fn execute_ibc_transfer_to_host(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_transfer_to_host = ica_amounts.to_transfer_to_host;
    if to_transfer_to_host.is_zero() {
        return Ok(Response::new());
    }
    let timeout = env.block.time.plus_seconds(config.transfer_timeout);
    let ibc_msg = IbcMsg::Transfer {
        channel_id: config.controller_config.transfer_channel_id,
        to_address: config.ica_account,
        amount: coin(to_transfer_to_host.u128(), config.host_config.atom_denom),
        timeout: IbcTimeout::from(timeout),
    };

    let mut config: Config = CONFIG.load(store)?;
    config.controller_config.stacked_amount_to_deposit = Uint128::from(0u128);
    config.controller_config.pending_transfer_amount += to_transfer_to_host;
    CONFIG.save(store, &config)?;

    let res = Response::new()
        .add_message(ibc_msg)
        .add_attribute("action", "ibc_transfer_to_host");
    Ok(res)
}

pub fn execute_ibc_transfer_to_controller(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_transfer_to_controller = ica_amounts.to_transfer_to_controller;
    if to_transfer_to_controller.is_zero() {
        return Ok(Response::new());
    }
    let msg = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: config.controller_config.transfer_channel_id,
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

// TODO: add endpoint for ibc transfer initiated by yieldaggregator module endblocker
// TODO: add endpoint for initiating stake, unstake, claim rewards + autocompound for each epoch yieldaggregator trigger

pub fn execute_ica_add_and_bond_liquidity(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(store)?;
    let share_out_amount = determine_ica_amounts(config.to_owned()).to_add_lp;
    config.host_config.bonded_lp_amount += share_out_amount;
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
        pool_id: 1u64,
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
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_remove_lp = ica_amounts.to_remove_lp;
    if to_remove_lp.is_zero() {
        return Ok(Response::new());
    }

    config.host_config.bonded_lp_amount -= to_remove_lp;
    CONFIG.save(store, &config)?;

    let msg = MsgExitPool {
        sender: config.ica_account.to_string(),
        share_in_amount: to_remove_lp.to_string(),
        pool_id: 1u64,
        token_out_mins: vec![
            OsmosisCoin {
                denom: config.host_config.osmo_denom,
                amount: "1".to_string(),
            },
            OsmosisCoin {
                denom: config.host_config.atom_denom.to_string(),
                amount: "1".to_string(),
            },
        ],
    };
    if let Ok(msg_any) = exit_pool_to_any(msg) {
        return send_ica_tx(store, env, "remove_liquidity".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_swap_rewards_to_two_tokens(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let msg = MsgSwapExactAmountIn {
        sender: config.ica_account.to_string(),
        token_in: Some(OsmosisCoin {
            denom: config.host_config.osmo_denom,
            amount: config.host_config.free_osmo_amount.to_string(),
        }),
        token_out_min_amount: "1".to_string(),
        routes: vec![SwapAmountInRoute {
            pool_id: 1u64,
            token_out_denom: config.host_config.atom_denom,
        }],
    };
    if let Ok(msg_any) = swap_msg_to_any(msg) {
        return send_ica_tx(store, env, "swap_rewards".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_swap_two_tokens_to_deposit_token(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response, ContractError> {
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
            pool_id: 1u64,
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

pub fn execute_ica_swap_deposit_token_to_two_tokens(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_swap_atom = ica_amounts.to_swap_atom;
    if to_swap_atom.is_zero() {
        return Ok(Response::new());
    }

    let msg = MsgSwapExactAmountIn {
        sender: config.ica_account.to_string(),
        token_in: Some(OsmosisCoin {
            denom: config.host_config.atom_denom,
            amount: to_swap_atom.to_string(),
        }),
        token_out_min_amount: "1".to_string(),
        routes: vec![SwapAmountInRoute {
            pool_id: 1u64,
            token_out_denom: config.host_config.osmo_denom,
        }],
    };
    if let Ok(msg_any) = swap_msg_to_any(msg) {
        return send_ica_tx(
            store,
            env,
            "swap_to_lp_underlyings".to_string(),
            vec![msg_any],
        );
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_begin_unbonding_lp_tokens(
    store: &mut dyn Storage,
    env: Env,
    unbonding_lp_amount: Uint128,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(store)?;
    if unbonding_lp_amount.is_zero() {
        return Ok(Response::new());
    }
    let msg = MsgBeginUnlocking {
        owner: config.ica_account.to_string(),
        id: 1u64,
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

pub fn execute_store_ica_unlocked_balances(
    deps: DepsMut,
    env: Env,
    coins: Vec<Coin>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    for coin in coins.iter() {
        if coin.denom == config.host_config.osmo_denom {
            config.host_config.free_osmo_amount = coin.amount;
        } else if coin.denom == config.host_config.atom_denom {
            config.host_config.free_atom_amount = coin.amount;
        } else if coin.denom == config.host_config.lp_denom {
            config.host_config.free_lp_amount = coin.amount;
        }
    }
    CONFIG.save(deps.storage, &config)?;
    execute_epoch(deps, env, EpochCallSource::IcqCallback, true)?;
    let res = Response::new().add_attribute("action", "store_ica_unlocked_balances".to_string());
    return Ok(res);
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Unbonding { addr } => to_binary(&query_unbonding(deps, addr)?),
        QueryMsg::Bonded { addr } => to_binary(&query_bonded(deps, addr)?),
        QueryMsg::Fee {} => to_binary(&query_fee_info(deps)?),
        QueryMsg::ListChannels {} => to_binary(&query_list_channels(deps)?),
        QueryMsg::Channel { id } => to_binary(&query_channel(deps, id)?),
        QueryMsg::Unbondings {} => to_binary(&query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(config)
}

fn query_list_channels(deps: Deps) -> StdResult<ListChannelsResponse> {
    let channels = CHANNEL_INFO
        .range_raw(deps.storage, None, None, Order::Ascending)
        .map(|r| r.map(|(_, v)| v))
        .collect::<StdResult<_>>()?;
    Ok(ListChannelsResponse { channels: channels })
}

// make public for ibc tests
pub fn query_channel(deps: Deps, id: String) -> StdResult<ChannelResponse> {
    let info = CHANNEL_INFO.load(deps.storage, &id)?;
    Ok(ChannelResponse { info })
}

pub fn query_fee_info(_: Deps) -> StdResult<FeeInfo> {
    Ok(FeeInfo {
        deposit_fee_rate: Decimal::zero(),
        withdraw_fee_rate: Decimal::zero(),
        interest_fee_rate: Decimal::zero(),
    })
}

pub fn query_unbonding(_: Deps, _: String) -> StdResult<Uint128> {
    Ok(Uint128::from(0u128))
}

pub fn query_bonded(deps: Deps, addr: String) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(deps.storage)?;
    let deposit = DEPOSITS.load(deps.storage, addr)?;
    let redemption_rate_multiplier = Uint128::from(1000000_000000_000000u128); // 10^18
    Ok(deposit.amount * config.host_config.lp_redemption_rate / redemption_rate_multiplier)
}

const DEFAULT_LIMIT: u32 = 10;
pub fn query_unbondings(storage: &dyn Storage, limit: Option<u32>) -> StdResult<Vec<Unbonding>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT) as usize;

    UNBONDINGS
        .range(storage, None, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, v) = item?;
            Ok(v)
        })
        .collect::<StdResult<Vec<Unbonding>>>()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::*;

    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn execute_update_config() {}

    #[test]
    fn execute_stake() {}

    #[test]
    fn execute_unstake() {}

    #[test]
    fn query_unbonding() {}

    #[test]
    fn query_bonded() {}
}
