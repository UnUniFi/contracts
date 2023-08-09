use crate::epoch::execute_epoch;
use crate::error::ContractError;
use crate::execute::stake::execute_stake;
use crate::execute::unstake::execute_unstake;
use crate::execute::update_config::execute_update_config;
use crate::msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, Phase, QueryMsg};
use crate::query::bonded::query_bonded;
use crate::query::channel::query_channel;
use crate::query::config::query_config;
use crate::query::fee_info::query_fee_info;
use crate::query::list_channels::query_list_channels;
use crate::query::unbonding::query_unbonding;
use crate::query::unbondings::{query_unbondings, UNBONDING_ITEM_LIMIT};
use crate::state::{Config, EpochCallSource, CONFIG, STAKE_RATE_MULTIPLIER};
use crate::state::{ControllerConfig, HostConfig};
use crate::sudo::kv_query_result::sudo_kv_query_result;
use crate::sudo::transfer_callback::sudo_transfer_callback;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw_utils::one_coin;
use strategy::v0::msgs::SudoMsg;
use ununifi_binding::v0::binding::UnunifiMsg;

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
        last_unbonding_id: 1u64,
        redemption_rate: STAKE_RATE_MULTIPLIER,
        total_shares: Uint128::from(0u128),
        total_deposit: Uint128::from(0u128),
        total_withdrawn: Uint128::from(0u128),
        transfer_timeout: msg.transfer_timeout, // 300s
        ica_connection_id: "".to_string(),
        ica_channel_id: "".to_string(),
        ica_account: "".to_string(),
        phase: Phase::Deposit,
        phase_step: 1u64,
        pending_icq: 0u64,
        host_config: HostConfig {
            chain_id: msg.chain_id,
            pool_id: msg.pool_id,
            transfer_channel_id: msg.transfer_channel_id,
            lp_redemption_rate: Uint128::from(200000u128),
            lock_id: 0u64,
            lp_denom: msg.lp_denom, // ATOM-OSMO
            bonded_lp_amount: Uint128::from(0u128),
            unbonding_lp_amount: Uint128::from(0u128),
            free_lp_amount: Uint128::from(0u128),
            pending_bond_lp_amount: Uint128::from(0u128),
            pending_lp_removal_amount: Uint128::from(0u128), // pending swap from lp to deposit token amount
            quote_denom: msg.quote_denom,                    // OSMO
            free_quote_amount: Uint128::from(0u128),
            pending_swap_to_base_amount: Uint128::from(0u128), // Convert OSMO to ATOM
            base_denom: msg.base_denom,                        // ATOM
            free_base_amount: Uint128::from(0u128),            // free ATOM balance
            pending_swap_to_quote_amount: Uint128::from(0u128), // pending swap from ATOM -> OSMO to add liquidity
            pending_add_liquidity_amount: Uint128::from(0u128), // amount of ATOM used on liquidity addition
            pending_transfer_amount: Uint128::from(0u128), // pending transfer to controller - TODO: how to get hook for transfer finalization?
        },
        controller_config: ControllerConfig {
            transfer_channel_id: msg.controller_transfer_channel_id,
            deposit_denom: msg.controller_deposit_denom, // `ibc/xxxxuatom`
            free_amount: Uint128::from(0u128),
            pending_transfer_amount: Uint128::from(0u128),
            stacked_amount_to_deposit: Uint128::from(0u128),
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Unbonding { addr } => to_binary(&query_unbonding(deps, addr)?),
        QueryMsg::Bonded { addr } => to_binary(&query_bonded(deps, addr)?),
        QueryMsg::Fee {} => to_binary(&query_fee_info(deps)?),
        QueryMsg::ListChannels {} => to_binary(&query_list_channels(deps)?),
        QueryMsg::Channel { id } => to_binary(&query_channel(deps, id)?),
        QueryMsg::Unbondings {} => {
            to_binary(&query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?)
        }
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
