use crate::state::{PARAMS, PROVIDERS, PROVIDER_ID};
use crate::types::{Params, Provider};
use crate::{error::ContractError, msgs::RegisterProviderMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_register_provider(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: RegisterProviderMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let params: Params = PARAMS.load(deps.storage)?;

    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    let id = PROVIDER_ID.load(deps.storage)?;
    let provider = Provider {
        id,
        address: deps.api.addr_validate(&msg.address)?,
        name: msg.name,
        identity: msg.identity,
        website: msg.website,
        security_contact: msg.security_contact,
        details: msg.details,
    };

    PROVIDER_ID.save(deps.storage, &(id + 1))?;
    PROVIDERS.save(deps.storage, id, &provider)?;

    response = response.add_attribute("action", "register_provider");

    Ok(response)
}
