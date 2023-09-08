use crate::error::ContractError;
use crate::msgs::UpdateProviderMsg;
use crate::state::PROVIDERS;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_update_provider(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateProviderMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let mut provider = PROVIDERS.load(deps.storage, msg.id)?;

    if info.sender != provider.address {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(address) = msg.address {
        provider.address = deps.api.addr_validate(&address)?;
    }
    if let Some(name) = msg.name {
        provider.name = name;
    }
    if let Some(identity) = msg.identity {
        provider.identity = identity;
    }
    if let Some(website) = msg.website {
        provider.website = website;
    }
    if let Some(security_contact) = msg.security_contact {
        provider.security_contact = security_contact;
    }
    if let Some(details) = msg.details {
        provider.details = details;
    }
    if let Some(information_fee) = msg.information_fee {
        provider.information_fee = information_fee;
    }

    PROVIDERS.save(deps.storage, msg.id, &provider)?;

    response = response.add_attribute("action", "update_provider");

    Ok(response)
}
