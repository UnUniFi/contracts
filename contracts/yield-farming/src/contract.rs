use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use yield_farming::farming::{ConfigResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        owner: info.sender,
        unbond_period: msg.unbond_period,
        is_freeze: false,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new())
}

//Execute the handle messages.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdateConfig {
            owner,
            unbond_period,
        } => execute_update_config(deps, env, info, owner, unbond_period),
        ExecuteMsg::UpdateFreezeFlag { freeze_flag } => {
            execute_update_freeze_flag(deps, env, info, freeze_flag)
        }
        ExecuteMsg::Deposit { denom, amount } => todo!(),
        ExecuteMsg::ClaimReward { denom, amount } => todo!(),
        ExecuteMsg::ClaimAllRewards {} => todo!(),
        ExecuteMsg::StartUnbond {} => todo!(),
        ExecuteMsg::ClaimUnbond {} => todo!(),
        ExecuteMsg::SwapReward {
            source_token,
            dest_token,
        } => todo!(),
        ExecuteMsg::AutoCompoundRewards {} => todo!(),
    }
}

/// Only owner can execute it. To update the owner address
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<String>,
    unbond_period: Option<u64>,
) -> StdResult<Response> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(StdError::generic_err("unauthorized"));
    }
    if let Some(owner) = owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }
    if let Some(unbond_period) = unbond_period {
        config.unbond_period = unbond_period;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn execute_update_freeze_flag(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    freeze_flag: bool,
) -> StdResult<Response> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(StdError::generic_err("unauthorized"));
    }

    config.is_freeze = freeze_flag;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_freeze_flag"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner.to_string(),
        unbond_period: config.unbond_period,
        is_freeze: config.is_freeze,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
