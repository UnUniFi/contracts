use crate::msg::{
    ChannelResponse, ExecuteMsg, FeeInfo, InstantiateMsg, ListChannelsResponse, MigrateMsg,
    QueryMsg,
};
use crate::state::{
    Config, DepositInfo, IcaAmounts, Unbonding, CHANNEL_INFO, CONFIG, DEPOSITS, UNBONDINGS,
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
        total_unbonding_amount: Uint128::from(0u128),
        transfer_timeout: 300, // 300s
        ica_channel_id: "".to_string(),
        ica_account: "".to_string(),
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
            execute_stake(deps, coin, info.sender)
        }
        ExecuteMsg::Unstake(msg) => execute_unstake(deps, msg.amount, info.sender),
        ExecuteMsg::ExecuteEpoch() => execute_epoch(deps),
        ExecuteMsg::IbcTransferToHost(msg) => execute_ibc_transfer_to_host(
            deps,
            msg.ica_channel_id,
            msg.channel_id,
            msg.denom,
            msg.amount,
            msg.timeout,
        ),
        ExecuteMsg::IbcTransferToController(msg) => {
            execute_ibc_transfer_to_controller(deps, env, msg.amount)
        }
        ExecuteMsg::IcaAddLiquidity(_) => execute_ica_add_liquidity(deps, env),
        ExecuteMsg::IcaRemoveLiquidity(_) => execute_ica_remove_liquidity(deps, env),
        ExecuteMsg::IcaSwapRewardsToTwoTokens(msg) => {
            execute_ica_swap_rewards_to_two_tokens(deps, env)
        }
        ExecuteMsg::IcaSwapTwoTokensToDepositToken(msg) => {
            execute_ica_swap_two_tokens_to_deposit_token(deps, env)
        }
        ExecuteMsg::IcaSwapDepositTokenToTwoTokens(msg) => {
            execute_ica_swap_deposit_token_to_two_tokens(
                deps,
                env,
                msg.channel_id,
                msg.denom,
                msg.amount,
                msg.timeout,
            )
        }
        ExecuteMsg::IcaBondLpTokens(_) => execute_ica_bond_lp_tokens(deps, env),
        ExecuteMsg::IcaBeginUnbondLpTokens(_) => execute_ica_begin_unbonding_lp_tokens(deps, env),
        ExecuteMsg::StoreIcaUnlockedBalances(msg) => {
            execute_store_ica_unlocked_balances(deps, msg.coins)
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

pub fn determine_ica_amounts(config: Config) -> IcaAmounts {
    let lp_redemption_rate_multiplier = Uint128::from(1000000_000000_000000u128); // 10^18
    let to_add_liquidity_lp = config.host_config.free_atom_amount
        * lp_redemption_rate_multiplier
        * Uint128::from(2u128)
        * Uint128::from(8u128)
        / Uint128::from(10u128)
        / config.host_config.lp_redemption_rate;

    let to_withdraw_atom_from_host =
        config.total_unbonding_amount - config.controller_config.free_amount;
    let to_remove_liquidity = to_withdraw_atom_from_host - config.host_config.free_atom_amount;
    let mut to_remove_lp =
        to_remove_liquidity * lp_redemption_rate_multiplier / config.host_config.lp_redemption_rate;
    let mut to_unbond_lp = Uint128::from(0u128);
    if to_remove_lp > config.host_config.free_lp_amount {
        to_unbond_lp = to_remove_lp - config.host_config.free_lp_amount;
        to_remove_lp = config.host_config.free_lp_amount;
    }

    let to_bond_lp = config.host_config.free_lp_amount - to_remove_lp;

    return IcaAmounts {
        to_swap_atom: config.host_config.free_atom_amount,
        to_swap_osmo: config.host_config.free_osmo_amount,
        to_add_liquidity_lp: to_add_liquidity_lp,
        to_bond_lp: to_bond_lp,
        to_remove_lp: to_remove_lp,
        to_add_lp: to_bond_lp,
        to_unbond_lp: to_unbond_lp,
        to_transfer_to_controller: Uint128::from(30u128), // TODO: test value
        to_transfer_to_host: Uint128::from(111u128),      // TODO: test value
    };
}

pub fn execute_stake(deps: DepsMut, coin: Coin, sender: Addr) -> Result<Response, ContractError> {
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

    execute_ibc_transfer_to_host(
        deps,
        config.ica_channel_id,
        config.host_config.transfer_channel_id,
        config.controller_config.deposit_denom,
        amount,
        config.transfer_timeout,
    )?;

    let rsp = Response::default()
        .add_attribute("action", "stake")
        .add_attribute("sender", sender)
        .add_attribute("amount", amount);
    Ok(rsp)
}

pub fn execute_epoch(deps: DepsMut) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // execute finalized unbondings
    let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
    let mut free_atom_balance = config.controller_config.free_amount;
    let mut total_unbonding_amount = config.total_unbonding_amount;
    let mut rsp = Response::new();
    for unbonding in unbondings {
        if free_atom_balance < unbonding.amount {
            break;
        }
        UNBONDINGS.remove(deps.storage, unbonding.id);
        let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: unbonding.sender.to_string(),
            amount: coins(
                unbonding.amount.u128(),
                &config.controller_config.deposit_denom,
            ),
        });
        rsp = rsp.add_message(bank_send_msg);
        free_atom_balance -= unbonding.amount;
        total_unbonding_amount -= unbonding.amount;
    }
    config.controller_config.free_amount = free_atom_balance;
    config.total_unbonding_amount = total_unbonding_amount;
    CONFIG.save(deps.storage, &config)?;

    // TODO: execute ibc transfer to host
    // TODO: execute ibc transfer to controller
    // TODO: execute lockup
    // TODO: execute unbonding
    // TODO: execute swap
    // TODO: execute add_liquidity
    // TODO: execute remove_liquidity
    Ok(rsp)
}

pub fn execute_unstake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    let redemption_rate_multiplier = Uint128::from(1000000_000000_000000u128); // 10^18
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let unstake_amount =
                    amount * redemption_rate_multiplier / config.host_config.lp_redemption_rate;
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
        amount: amount,
    };
    UNBONDINGS.save(deps.storage, unbonding.id, unbonding)?;
    config.total_unbonding_amount += amount;
    CONFIG.save(deps.storage, &config)?;

    let rsp = Response::new()
        .add_attribute("sender", sender.to_string())
        .add_attribute("amount", amount);
    Ok(rsp)
}

pub fn send_ica_tx(
    deps: DepsMut,
    env: Env,
    action: String,
    msgs: Vec<Any>,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
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
    deps: DepsMut,
    ica_channel_id: String,
    channel_id: String,
    denom: String,
    amount: Uint128,
    timeout: u64,
) -> Result<Response, ContractError> {
    let info = CHANNEL_INFO.load(deps.storage, &ica_channel_id)?;
    let timestamp = Timestamp::from_seconds(timeout);
    let ibc_msg = IbcMsg::Transfer {
        channel_id: channel_id,
        to_address: info.address,
        amount: coin(amount.u128(), denom),
        timeout: IbcTimeout::from(timestamp),
    };

    let mut config: Config = CONFIG.load(deps.storage)?;
    config.controller_config.pending_transfer_amount += amount;
    CONFIG.save(deps.storage, &config)?;

    let res = Response::new()
        .add_message(ibc_msg)
        .add_attribute("action", "ibc_transfer_to_host");
    Ok(res)
}

pub fn execute_ibc_transfer_to_controller(
    deps: DepsMut,
    env: Env,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let msg = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: config.controller_config.transfer_channel_id,
        token: Some(ProtoCoin {
            denom: config.host_config.atom_denom,
            amount: amount.to_string(),
        }),
        sender: config.ica_account,
        receiver: env.contract.address.to_string(),
        timeout_height: None,
        timeout_timestamp: env.block.time.nanos() + config.transfer_timeout * 1000_000_000,
    };
    if let Ok(msg_any) = msg.to_any() {
        return send_ica_tx(
            deps,
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

pub fn execute_ica_add_liquidity(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
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

    let lp_redemption_rate_multiplier = Uint128::from(1000000_000000_000000u128); // 10^18

    // share_out_amount = atom_balance * 2 * 80% / redemption_rate
    let share_out_amount = config.host_config.free_atom_amount
        * lp_redemption_rate_multiplier
        * Uint128::from(2u128)
        * Uint128::from(8u128)
        / Uint128::from(10u128)
        / config.host_config.lp_redemption_rate;
    let msg = MsgJoinPool {
        sender: config.ica_account.to_string(),
        share_out_amount: share_out_amount.to_string(),
        pool_id: 1u64,
        token_in_maxs: tokens_in,
    };
    if let Ok(msg_any) = join_pool_to_any(msg) {
        return send_ica_tx(deps, env, "add_liquidity".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_remove_liquidity(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_remove_lp = ica_amounts.to_remove_lp;
    if to_remove_lp.is_zero() {
        return Ok(Response::new());
    }

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
        return send_ica_tx(deps, env, "remove_liquidity".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_swap_rewards_to_two_tokens(
    deps: DepsMut,
    env: Env,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
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
        return send_ica_tx(deps, env, "swap_rewards".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_swap_two_tokens_to_deposit_token(
    deps: DepsMut,
    env: Env,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
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
        return send_ica_tx(deps, env, "swap_to_atom".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_swap_deposit_token_to_two_tokens(
    deps: DepsMut,
    env: Env,
    channel_id: String,
    denom: String,
    amount: Uint128,
    timeout: u64,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
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
            deps,
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

pub fn execute_ica_bond_lp_tokens(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_bond_lp = ica_amounts.to_bond_lp;
    if to_bond_lp.is_zero() {
        return Ok(Response::new());
    }

    let msg = MsgLockTokens {
        owner: config.ica_account.to_string(),
        coins: vec![OsmosisCoin {
            denom: config.host_config.lp_denom,
            amount: to_bond_lp.to_string(),
        }],
        duration: Some(Duration {
            seconds: config.unbond_period as i64,
            nanos: 0,
        }),
    };
    if let Ok(msg_any) = lock_tokens_msg_to_any(msg) {
        return send_ica_tx(deps, env, "bond_lp_tokens".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_ica_begin_unbonding_lp_tokens(
    deps: DepsMut,
    env: Env,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_unbond_lp = ica_amounts.to_unbond_lp;
    if to_unbond_lp.is_zero() {
        return Ok(Response::new());
    }
    let msg = MsgBeginUnlocking {
        owner: config.ica_account.to_string(),
        id: 1u64,
        coins: vec![OsmosisCoin {
            denom: config.host_config.lp_denom,
            amount: to_unbond_lp.to_string(),
        }],
    };
    if let Ok(msg_any) = begin_unlocking_msg_to_any(msg) {
        return send_ica_tx(deps, env, "begin_unbonding_lp".to_string(), vec![msg_any]);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}

pub fn execute_store_ica_unlocked_balances(
    deps: DepsMut,
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
