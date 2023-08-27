use crate::error::ContractError;
use crate::state::CONFIG;
use crate::types::Config;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Only owner can execute it.
#[cfg(not(feature = "library"))]
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<String>,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }

    CONFIG.save(deps.storage, &config)?;
    response = response
        .add_attribute("action", "update_config")
        .add_attribute("owner", config.owner.to_string());

    Ok(response)
}
