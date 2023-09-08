use crate::error::ContractError;
use crate::msgs::ApproveInformationRequestMsg;
use crate::state::INFORMATION_REQUESTS;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_approve_information_request(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ApproveInformationRequestMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let mut request = INFORMATION_REQUESTS.load(deps.storage, msg.request_id)?;
    if let Some(_) = &request.approved {
        return Err(ContractError::AlreadyApprovedOrRejected {});
    }

    if info.sender != request.customer {
        return Err(ContractError::Unauthorized {});
    }

    request.approved = Some(true);

    INFORMATION_REQUESTS.save(deps.storage, msg.request_id, &request)?;

    response = response.add_attribute("action", "approve_information_request");

    Ok(response)
}
