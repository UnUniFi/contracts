use crate::error::ContractError;
use crate::msgs::CreateVerificationMsg;
use crate::state::{PROVIDERS, VERIFICATIONS};
use crate::types::Verification;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_create_verification(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: CreateVerificationMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let provider = PROVIDERS.load(deps.storage, msg.provider_id)?;

    if info.sender != provider.address {
        return Err(ContractError::Unauthorized {});
    }

    let address = deps.api.addr_validate(&msg.customer)?;
    let verification = Verification {
        address: address.clone(),
        provider_id: msg.provider_id,
    };
    VERIFICATIONS.save(deps.storage, (address, msg.provider_id), &verification)?;

    response = response.add_attribute("action", "create_verification");

    Ok(response)
}
