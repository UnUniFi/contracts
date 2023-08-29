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
    let mut params: Params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(authority) = msg.owner {
        params.authority = deps.api.addr_validate(&authority)?;
    }
    if let Some(unbond_period) = msg.unbond_period {
        params.unbond_period = unbond_period;
    }
    if let Some(deposit_denom) = msg.deposit_denom {
        params.deposit_denom = deposit_denom;
    }

    PARAMS.save(deps.storage, &params)?;
    let resp = Response::new()
        .add_attribute("action", "update_params")
        .add_attribute("owner", params.authority.to_string())
        .add_attribute("unbond_period", params.unbond_period.to_string())
        .add_attribute("deposit_denom", params.deposit_denom.to_string());

    Ok(resp)
}
