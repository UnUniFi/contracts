use crate::error::ContractError;
use crate::msgs::UpdateParamsMsg;
use crate::state::PARAMS;
use crate::types::Params;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Only owner can execute it.
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
    if info.sender != params.admin {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(authority) = msg.admin {
        params.admin = deps.api.addr_validate(&authority)?;
    }
    if let Some(deposit_denom) = msg.deposit_denom {
        params.deposit_denom = deposit_denom;
    }

    PARAMS.save(deps.storage, &params)?;
    response = response
        .add_attribute("action", "update_params")
        .add_attribute("admin", params.admin.to_string())
        .add_attribute("deposit_denom", params.deposit_denom.to_string());

    Ok(response)
}
