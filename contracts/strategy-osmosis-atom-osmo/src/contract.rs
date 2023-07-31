use crate::binding::{SudoMsg, UnunifiMsg};
use crate::epoch::execute_epoch;
use crate::icq::{sudo_kv_query_result, sudo_transfer_callback};
use crate::query::{
    query_bonded, query_channel, query_config, query_fee_info, query_list_channels,
    query_unbonding, query_unbondings, DEFAULT_LIMIT,
};
use crate::state::{
    Config, DepositInfo, EpochCallSource, Unbonding, CHANNEL_INFO, CONFIG, DEPOSITS,
    HOST_LP_RATE_MULTIPLIER, STAKE_RATE_MULTIPLIER, UNBONDINGS,
};
use crate::state::{ControllerConfig, HostConfig};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw_utils::{one_coin};
use strategy::error::{ContractError, NoDeposit};
use strategy_osmosis::strategy::{
    ChannelInfo, ExecuteMsg, InstantiateMsg, MigrateMsg, Phase, QueryMsg, UpdateConfigMsg,
};

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config = Config {
        owner: info.sender,
        unbond_period: msg.unbond_period,
        total_deposit: Uint128::from(0u128),
        last_unbonding_id: 1u64,
        redemption_rate: STAKE_RATE_MULTIPLIER,
        total_withdrawn: Uint128::from(0u128),
        transfer_timeout: 300, // 300s
        ica_connection_id: "".to_string(),
        ica_channel_id: "".to_string(),
        ica_account: "".to_string(),
        phase: Phase::Deposit,
        phase_step: 1u64,
        pending_icq: 0u64,
        host_config: HostConfig {
            chain_id: "test-1".to_string(),
            pool_id: 1,
            transfer_channel_id: "channel-1".to_string(),
            lp_redemption_rate: Uint128::from(200000u128),
            lock_id: 0u64,
            lp_denom: "gamm/pool/1".to_string(), // ATOM-OSMO
            bonded_lp_amount: Uint128::from(0u128),
            free_lp_amount: Uint128::from(0u128),
            pending_bond_lp_amount: Uint128::from(0u128),
            pending_lp_removal_amount: Uint128::from(0u128), // pending swap from lp to deposit token amount
            osmo_denom: "uosmo".to_string(),                 // OSMO
            free_osmo_amount: Uint128::from(0u128),
            pending_swap_to_atom_amount: Uint128::from(0u128), // Convert OSMO to ATOM
            atom_denom: "stake".to_string(),                   // ATOM
            free_atom_amount: Uint128::from(0u128),            // free ATOM balance
            pending_swap_to_osmo_amount: Uint128::from(0u128), // pending swap from ATOM -> OSMO to add liquidity
            pending_add_liquidity_amount: Uint128::from(0u128), // amount of ATOM used on liquidity addition
            pending_transfer_amount: Uint128::from(0u128), // pending transfer to controller - TODO: how to get hook for transfer finalization?
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

#[entry_point]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response<UnunifiMsg>, ContractError> {
    match msg {
        SudoMsg::KVQueryResult {
            connection_id,
            chain_id,
            query_prefix,
            query_key,
            data,
        } => sudo_kv_query_result(
            deps,
            env,
            connection_id,
            chain_id,
            query_prefix,
            query_key,
            data,
        ),
        SudoMsg::TransferCallback {
            denom,
            amount,
            sender,
            receiver,
            memo,
            success,
        } => sudo_transfer_callback(deps, env, denom, amount, sender, receiver, memo, success),
    }
}

//Execute the handle messages.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig(msg) => execute_update_config(deps, env, info, msg),
        ExecuteMsg::Stake(_) => {
            let coin: Coin = one_coin(&info).map_err(|err| ContractError::Payment(err))?;
            execute_stake(deps, env, coin, info.sender)
        }
        ExecuteMsg::Unstake(msg) => execute_unstake(deps, msg.amount, info.sender),
        ExecuteMsg::ExecuteEpoch(_) => {
            execute_epoch(deps, env, EpochCallSource::NormalEpoch, true, None)
        }
    }
}

/// Only owner can execute it.
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = msg.owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }
    if let Some(unbond_period) = msg.unbond_period {
        config.unbond_period = unbond_period;
    }
    if let Some(pool_id) = msg.pool_id {
        config.host_config.pool_id = pool_id;
    }
    if let Some(ica_channel_id) = msg.ica_channel_id {
        let info: ChannelInfo = CHANNEL_INFO.load(deps.storage, ica_channel_id.as_str())?;
        config.ica_account = info.address.to_string();
        config.ica_channel_id = info.id;
        config.ica_connection_id = info.connection_id.to_string();
    }
    if let Some(phase) = msg.phase {
        config.phase = phase;
    }
    if let Some(phase_step) = msg.phase_step {
        config.phase_step = phase_step;
    }
    if let Some(transfer_timeout) = msg.transfer_timeout {
        config.transfer_timeout = transfer_timeout;
    }
    if let Some(lp_denom) = msg.lp_denom {
        config.host_config.lp_denom = lp_denom;
    }
    if let Some(lp_redemption_rate) = msg.lp_redemption_rate {
        config.host_config.lp_redemption_rate = lp_redemption_rate;
    }
    if let Some(transfer_channel_id) = msg.transfer_channel_id {
        config.host_config.transfer_channel_id = transfer_channel_id;
    }
    if let Some(osmo_denom) = msg.osmo_denom {
        config.host_config.osmo_denom = osmo_denom;
    }
    if let Some(atom_denom) = msg.atom_denom {
        config.host_config.atom_denom = atom_denom;
    }
    if let Some(deposit_denom) = msg.controller_deposit_denom {
        config.controller_config.deposit_denom = deposit_denom;
    }
    if let Some(transfer_channel_id) = msg.controller_transfer_channel_id {
        config.controller_config.transfer_channel_id = transfer_channel_id;
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

pub fn execute_stake(
    deps: DepsMut,
    env: Env,
    coin: Coin,
    sender: Addr,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.controller_config.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let amount = coin.amount;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let stake_amount = amount * STAKE_RATE_MULTIPLIER / config.redemption_rate;
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

pub fn execute_unstake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    let unstake_amount = amount * STAKE_RATE_MULTIPLIER / config.redemption_rate;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_sub(unstake_amount)?,
                });
            }
            Err(NoDeposit {}.into())
        },
    )?;

    let unbonding = &Unbonding {
        id: config.last_unbonding_id + 1,
        sender: sender.to_owned(),
        amount: amount * HOST_LP_RATE_MULTIPLIER / config.host_config.lp_redemption_rate,
        pending_start: false,
        start_time: 0u64,
        marked: false,
    };
    UNBONDINGS.save(deps.storage, unbonding.id, unbonding)?;

    // increase last unbonding id
    // NOTE: eventually, we should remove these params from config because it's simply double counting
    config.last_unbonding_id += 1;
    CONFIG.save(deps.storage, &config)?;

    let rsp = Response::new()
        .add_attribute("sender", sender.to_string())
        .add_attribute("amount", amount);
    Ok(rsp)
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    Ok(Response::default())
}
