use crate::error::ContractError;
use crate::msgs::UpdateParamsMsg;
use crate::state::PARAMS;
use cosmwasm_std::Response;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

// #[cfg(not(feature = "library"))]
pub fn execute_update_params(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateParamsMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let mut params = PARAMS.load(deps.storage)?;
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(authority) = msg.authority {
        params.authority = deps.api.addr_validate(&authority)?;
    }

    if let Some(fee_collector) = msg.fee_collector {
        params.fee_collector = deps.api.addr_validate(&fee_collector)?;
    }

    if let Some(selling_fee_rate) = msg.selling_fee_rate {
        params.selling_fee_rate = selling_fee_rate;
    }

    if let Some(interest_fee_rate) = msg.interest_fee_rate {
        params.interest_fee_rate = interest_fee_rate;
    }

    PARAMS.save(deps.storage, &params)?;

    response = response.add_attribute("action", "update_params");

    Ok(response)
}
