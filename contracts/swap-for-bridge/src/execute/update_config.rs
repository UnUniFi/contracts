use crate::error::ContractError;
use crate::msgs::UpdateConfigMsg;
use crate::state::CONFIG;
use crate::types::Config;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.authority {
        return Err(ContractError::Unauthorized);
    }

    if let Some(authority) = msg.authority {
        config.authority = deps.api.addr_validate(&authority)?;
    }

    if let Some(treasury) = msg.treasury {
        config.treasury = deps.api.addr_validate(&treasury)?;
    }

    if let Some(fee) = msg.fee {
        config.fee = fee;
    }

    CONFIG.save(deps.storage, &config)?;
    response = response.add_attribute("action", "update_config");
    // TODO: add events

    Ok(response)
}
