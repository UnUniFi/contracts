use crate::error::ContractError;
use crate::execute::epoch::epoch::execute_epoch;
use crate::execute::stake::execute_stake;
use crate::execute::superfluid::execute_superfluid_delegate;
use crate::execute::unstake::{
    execute_instant_unbondings, execute_unstake, execute_update_unbonding_recipients,
};
use crate::execute::update_params::execute_update_params;
use crate::msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, Phase, PhaseStep, QueryMsg};
use crate::query::amounts::query_amounts;
use crate::query::bonded::query_bonded;
use crate::query::channel::query_channel;
use crate::query::fee_info::query_fee_info;
use crate::query::kyc::query_kyc_info;
use crate::query::list_channels::query_list_channels;
use crate::query::params::{query_deposit_denom, query_params};
use crate::query::state::query_state;
use crate::query::unbonding::query_unbonding;
use crate::query::unbondings::{query_unbondings, UNBONDING_ITEM_LIMIT};
use crate::state::{
    DepositInfo, DepositToken, EpochCallSource, LegacyDepositInfo, Params, State, DEPOSITS,
    LEGACY_CONFIG, LEGACY_DEPOSITS, PARAMS, STAKE_RATE_MULTIPLIER, STATE,
};
use crate::sudo::deposit_callback::sudo_deposit_callback;
use crate::sudo::kv_query_result::sudo_kv_query_result;
use crate::sudo::transfer_callback::sudo_transfer_callback;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, Uint128,
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
    let params = Params {
        authority: info.sender,
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
        superfluid_validator: msg.superfluid_validator,
        automate_superfluid: msg.automate_superfluid,
        extern_tokens: msg.extern_tokens.clone(),
    };
    PARAMS.save(deps.storage, &params)?;

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
        extern_token_amounts: vec![Uint128::from(0u128); msg.extern_tokens.len()],
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
        SudoMsg::DepositCallback(data) => sudo_deposit_callback(
            deps,
            env,
            data.denom,
            data.amount,
            data.sender,
            data.receiver,
            data.success,
        ),
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
        ExecuteMsg::UpdateParams(msg) => execute_update_params(deps, env, info, msg),
        ExecuteMsg::Stake(msg) => execute_stake(deps, env, info, msg),
        ExecuteMsg::Unstake(msg) => execute_unstake(deps, env, info, msg),
        ExecuteMsg::SuperfluidDelegate(_) => execute_superfluid_delegate(deps, env, info),
        ExecuteMsg::Epoch(_) => execute_epoch(deps, env, EpochCallSource::NormalEpoch, true, None),
        ExecuteMsg::UpdateLegacyUnbondingRecipients(msg) => {
            execute_update_unbonding_recipients(deps, env, info, msg)
        }
        ExecuteMsg::ProcessInstantUnbondings(msg) => {
            execute_instant_unbondings(deps, env, info, msg)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Version {} => to_binary(&query_version(deps)?),
        QueryMsg::DepositDenom {} => to_binary(&query_deposit_denom(deps)?),
        QueryMsg::Fee {} => to_binary(&query_fee_info(deps)?),
        QueryMsg::Amounts { addr } => to_binary(&query_amounts(deps, addr)?),
        QueryMsg::Kyc {} => to_binary(&query_kyc_info(deps)?),
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Unbonding { addr } => to_binary(&query_unbonding(deps, addr)?),
        QueryMsg::Bonded { addr } => to_binary(&query_bonded(deps, addr)?),
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
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    // Config to Params (owner => authority)
    let legacy_config = LEGACY_CONFIG.load(deps.storage)?;
    let params = Params {
        authority: legacy_config.owner,
        unbond_period: legacy_config.unbond_period,
        phase: legacy_config.phase,
        phase_step: legacy_config.phase_step,
        chain_id: legacy_config.chain_id,
        pool_id: legacy_config.pool_id,
        superfluid_validator: legacy_config.superfluid_validator,
        automate_superfluid: legacy_config.automate_superfluid,
        deposit_token: legacy_config.deposit_token,
        controller_deposit_denom: legacy_config.controller_deposit_denom,
        quote_denom: legacy_config.quote_denom,
        base_denom: legacy_config.base_denom,
        lp_denom: legacy_config.lp_denom,
        extern_tokens: legacy_config.extern_tokens,
        transfer_timeout: legacy_config.transfer_timeout,
        transfer_channel_id: legacy_config.transfer_channel_id,
        controller_transfer_channel_id: legacy_config.controller_transfer_channel_id,
        ica_channel_id: legacy_config.ica_channel_id,
        ica_connection_id: legacy_config.ica_connection_id,
        ica_account: legacy_config.ica_account,
    };
    PARAMS.save(deps.storage, &params)?;

    // Config to Params (amount => share)
    let legacy_deposits = LEGACY_DEPOSITS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (_, v) = item?;
            Ok(v)
        })
        .collect::<StdResult<Vec<LegacyDepositInfo>>>()?;
    for ld in legacy_deposits {
        let deposit = DepositInfo {
            sender: ld.sender.to_owned(),
            share: ld.amount,
        };
        DEPOSITS.save(deps.storage, ld.sender.to_string(), &deposit)?;
    }

    Ok(Response::default())
}
