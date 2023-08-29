use crate::error::ContractError;
use crate::msgs::UpdateParamsMsg;
use crate::state::PARAMS;
use crate::types::Params;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_update_params(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateParamsMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let mut params: Params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(authority) = msg.authority {
        params.authority = deps.api.addr_validate(&authority)?;
    }

    PARAMS.save(deps.storage, &params)?;

    response = response.add_attribute("action", "update_params");

    Ok(response)
}
