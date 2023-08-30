use crate::error::ContractError;
use crate::execute::report_profit::execute_report_profit;
use crate::execute::send_back::execute_send_back;
use crate::execute::stake::execute_stake;
use crate::execute::unstake::execute_unstake;
use crate::execute::update_params::execute_update_params;
use crate::msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::bonded::query_bonded;
use crate::query::fee::query_fee;
use crate::query::params::query_params;
use crate::query::unbonding::query_unbonding;
use crate::state::{PARAMS, TOTAL_DEPOSIT, TOTAL_SHARE, TOTAL_UNBONDING};
use crate::types::Params;
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Params {
        authority: info.sender,
        deposit_denom: msg.deposit_denom,
    };
    PARAMS.save(deps.storage, &config)?;
    TOTAL_DEPOSIT.save(deps.storage, &Uint128::new(0))?;
    TOTAL_SHARE.save(deps.storage, &Uint128::new(0))?;
    TOTAL_UNBONDING.save(deps.storage, &Uint128::new(0))?;

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
        ExecuteMsg::UpdateParams(msg) => execute_update_params(deps, env, info, msg),
        ExecuteMsg::Stake(msg) => execute_stake(deps, env, info, msg),
        ExecuteMsg::Unstake(msg) => execute_unstake(deps, env, info, msg),
        ExecuteMsg::SendBack(msg) => execute_send_back(deps, env, info, msg),
        ExecuteMsg::ReportProfit(msg) => execute_report_profit(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_params(deps)?),
        QueryMsg::Unbonding { addr } => to_binary(&query_unbonding(deps, addr)?),
        QueryMsg::Bonded { addr } => to_binary(&query_bonded(deps, addr)?),
        QueryMsg::Fee {} => to_binary(&query_fee(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
