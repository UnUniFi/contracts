use crate::error::ContractError;
use crate::msgs::RemoveVerificationMsg;
use crate::state::{PROVIDERS, VERIFICATIONS};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_remove_verification(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: RemoveVerificationMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let provider = PROVIDERS.load(deps.storage, msg.provider_id)?;

    if info.sender != provider.address {
        return Err(ContractError::Unauthorized {});
    }

    let address = deps.api.addr_validate(&msg.customer)?;
    if !VERIFICATIONS.has(deps.storage, (address.clone(), msg.provider_id)) {
        return Err(ContractError::VerificationNotFound {});
    }

    VERIFICATIONS.remove(deps.storage, (address, msg.provider_id));

    response = response.add_attribute("action", "remove_verification");

    Ok(response)
}
