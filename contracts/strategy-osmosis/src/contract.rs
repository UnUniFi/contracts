use crate::error::ContractError;
use crate::execute::epoch::epoch::execute_epoch;
use crate::execute::stake::execute_stake;
use crate::execute::unstake::execute_unstake;
use crate::execute::update_config::execute_update_config;
use crate::msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, Phase, PhaseStep, QueryMsg};
use crate::query::bonded::query_bonded;
use crate::query::channel::query_channel;
use crate::query::config::query_config;
use crate::query::fee_info::query_fee_info;
use crate::query::list_channels::query_list_channels;
use crate::query::state::query_state;
use crate::query::unbonding::query_unbonding;
use crate::query::unbondings::{query_unbondings, UNBONDING_ITEM_LIMIT};
use crate::state::{
    Config, DepositToken, EpochCallSource, State, CONFIG, STAKE_RATE_MULTIPLIER, STATE,
};
use crate::sudo::kv_query_result::sudo_kv_query_result;
use crate::sudo::transfer_callback::sudo_transfer_callback;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use strategy::v1::msgs::SudoMsg;
use strategy::v1::msgs::VersionResp;
use ununifi_binding::v1::binding::UnunifiMsg;

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
        phase: Phase::Deposit,
        phase_step: PhaseStep::IbcTransferToHost,
        chain_id: msg.chain_id,
        pool_id: msg.pool_id,
        deposit_token: DepositToken::Base, // ATOM
        controller_deposit_denom: msg.controller_deposit_denom, // `ibc/xxxxuatom`
        quote_denom: msg.quote_denom,      // OSMO
        base_denom: msg.base_denom,        // ATOM
        lp_denom: msg.lp_denom,            // ATOM-OSMO
        transfer_timeout: msg.transfer_timeout, // 300s
        transfer_channel_id: msg.transfer_channel_id,
        controller_transfer_channel_id: msg.controller_transfer_channel_id,
        ica_channel_id: "".to_string(),
        ica_connection_id: "".to_string(),
        ica_account: "".to_string(),
    };
    CONFIG.save(deps.storage, &config)?;

    let state = State {
        last_unbonding_id: 1u64,
        redemption_rate: STAKE_RATE_MULTIPLIER,
        total_shares: Uint128::from(0u128),
        total_deposit: Uint128::from(0u128),
        total_withdrawn: Uint128::from(0u128),
        pending_icq: 0u64,
        lp_redemption_rate: Uint128::from(200000u128),
        lock_id: 0u64,
        bonded_lp_amount: Uint128::from(0u128),
        unbonding_lp_amount: Uint128::from(0u128),
        free_lp_amount: Uint128::from(0u128),
        pending_bond_lp_amount: Uint128::from(0u128),
        pending_lp_removal_amount: Uint128::from(0u128), // pending swap from lp to deposit token amount
        free_quote_amount: Uint128::from(0u128),
        free_base_amount: Uint128::from(0u128), // free ATOM balance
        controller_free_amount: Uint128::from(0u128),
        controller_pending_transfer_amount: Uint128::from(0u128),
        controller_stacked_amount_to_deposit: Uint128::from(0u128),
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new())
}

#[entry_point]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response<UnunifiMsg>, ContractError> {
    match msg {
        SudoMsg::KvIcqCallback(data) => sudo_kv_query_result(
            deps,
            env,
            data.connection_id,
            data.chain_id,
            data.query_prefix,
            data.query_key,
            data.data,
        ),
        SudoMsg::TransferCallback(data) => sudo_transfer_callback(
            deps,
            env,
            data.denom,
            data.amount,
            data.sender,
            data.receiver,
            data.memo,
            data.success,
        ),
        SudoMsg::IBCLifecycleComplete(_) => Ok(Response::new()),
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
        ExecuteMsg::Stake(msg) => execute_stake(deps, env, info, msg),
        ExecuteMsg::Unstake(msg) => execute_unstake(deps, env, info, msg),
        ExecuteMsg::Epoch(_) => execute_epoch(deps, env, EpochCallSource::NormalEpoch, true, None),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Version {} => to_binary(&query_version(deps)?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
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

pub fn query_version(_: Deps) -> StdResult<VersionResp> {
    Ok(VersionResp { version: 1u8 })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    Ok(Response::default())
}
