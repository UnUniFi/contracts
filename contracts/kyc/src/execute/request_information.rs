use crate::error::ContractError;
use crate::msgs::RequestInformationMsg;
use crate::state::INFORMATION_REQUESTS;
use crate::state::INFORMATION_REQUEST_ID;
use crate::state::PROVIDERS;
use crate::types::InformationRequest;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;

#[cfg(not(feature = "library"))]
pub fn execute_request_information(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: RequestInformationMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let coin = one_coin(&info)?;
    let provider = PROVIDERS.load(deps.storage, msg.provider_id)?;

    if coin != provider.information_fee {
        return Err(ContractError::InvalidInformationFee {});
    }

    let id = INFORMATION_REQUEST_ID.load(deps.storage)?;
    let customer = deps.api.addr_validate(&msg.customer)?;
    let request = InformationRequest {
        customer: customer.clone(),
        id,
        sender: info.sender,
        provider_id: msg.provider_id,
        information_fee: coin,
        email: msg.email,
        approved: None,
    };

    INFORMATION_REQUEST_ID.save(deps.storage, &(id + 1))?;
    INFORMATION_REQUESTS.save(deps.storage, (customer, id), &request)?;

    response = response.add_attribute("action", "request_information");

    Ok(response)
}
