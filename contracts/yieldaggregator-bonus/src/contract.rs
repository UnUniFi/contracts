use crate::error::ContractError;
use crate::execute::delete_bonus_window::execute_delete_bonus_window;
use crate::execute::register_bonus_window::execute_register_bonus_window;
use crate::execute::stake_vault_share::execute_stake_vault_share;
use crate::execute::update_params::execute_update_params;
use crate::execute::vote::execute_vote;
use crate::msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::distribution_amount::query_distribution_amount;
use crate::query::params::query_params;
use crate::state::PARAMS;
use crate::types::Params;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let authority = deps.api.addr_validate(&msg.authority)?;

    let params = Params { authority };
    PARAMS.save(deps.storage, &params)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateParams(msg) => execute_update_params(deps, env, info, msg),
        ExecuteMsg::RegisterBonusWindow(msg) => execute_register_bonus_window(deps, env, info, msg),
        ExecuteMsg::DeleteBonusWindow(msg) => execute_delete_bonus_window(deps, env, info, msg),
        ExecuteMsg::StakeVaultShare(msg) => execute_stake_vault_share(deps, env, info, msg),
        ExecuteMsg::Vote(msg) => execute_vote(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
        QueryMsg::DistributionAmount { bonus_window_id } => {
            to_binary(&query_distribution_amount(deps, bonus_window_id)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
