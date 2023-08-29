use crate::error::ContractError;
use crate::execute::create_verification::execute_create_verification;
use crate::execute::register_provider::execute_register_provider;
use crate::execute::remove_verification::execute_remove_verification;
use crate::execute::update_params::execute_update_params;
use crate::execute::update_provider::execute_update_provider;
use crate::msgs::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::params::query_params;
use crate::query::providers::query_providers;
use crate::query::verifications::query_verifications;
use crate::state::{PARAMS, PROVIDER_ID};
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
    let config = Params { authority };
    PARAMS.save(deps.storage, &config)?;

    PROVIDER_ID.save(deps.storage, &0)?;

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
        ExecuteMsg::RegisterProvider(msg) => execute_register_provider(deps, env, info, msg),
        ExecuteMsg::UpdateProvider(msg) => execute_update_provider(deps, env, info, msg),
        ExecuteMsg::CreateVerification(msg) => execute_create_verification(deps, env, info, msg),
        ExecuteMsg::RemoveVerification(msg) => execute_remove_verification(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
        QueryMsg::Providers {} => to_binary(&query_providers(deps)?),
        QueryMsg::Verifications { address } => to_binary(&query_verifications(deps, address)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
