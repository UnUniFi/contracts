use std::vec;

use crate::error::ContractError;
use crate::execute::epoch::execute_epoch;
use crate::execute::set_redemption_rate::execute_set_redemption_rate;
use crate::execute::stake::execute_stake;
use crate::execute::unstake::execute_unstake;
use crate::execute::update_params::execute_update_params;
use crate::msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::amounts::query_amounts;
use crate::query::bonded::query_bonded;
use crate::query::fee_info::query_fee_info;
use crate::query::kyc::query_kyc_info;
use crate::query::params::{query_deposit_denom, query_params};
use crate::query::state::query_state;
use crate::state::{Params, State, PARAMS, STATE};
use crate::sudo::deposit_callback::sudo_deposit_callback;
use crate::sudo::kv_query_result::sudo_kv_query_result;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
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
        chain_id: msg.chain_id,
        denom: msg.denom,                     // `uatom`
        ls_denom: msg.ls_denom,               // `ibc/xxxxxstuatom`
        ls_denom_native: msg.ls_denom_native, // `stuatom`
        ls_rate_feeder: msg.ls_rate_feeder,
        connection_id: msg.connection_id,
    };
    PARAMS.save(deps.storage, &params)?;

    let state = State {
        total_amount: Uint128::from(0u128),
        total_deposit: Uint128::from(0u128),
        total_withdrawn: Uint128::from(0u128),
        ls_redemption_rate: Decimal::one(),
        ls_denom_apy: Decimal::zero(),
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
        SudoMsg::TransferCallback(_) => Ok(Response::new()),
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
        ExecuteMsg::Epoch(_) => execute_epoch(deps, env, true, None),
        ExecuteMsg::SetRedemptionRate(msg) => execute_set_redemption_rate(deps, env, info, msg),
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
        QueryMsg::Unbonding { addr } => to_binary(&Uint128::from(0u128)),
        QueryMsg::Bonded { addr } => to_binary(&query_bonded(deps, addr)?),
        QueryMsg::Unbondings {} => to_binary(&vec::Vec::<Uint128>::new()),
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
    Ok(Response::default())
}
