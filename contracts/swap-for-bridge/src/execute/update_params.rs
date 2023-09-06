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
    use cosmwasm_std::Decimal;

    let mut response = Response::new();

    let mut config: Params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != config.authority {
        return Err(ContractError::Unauthorized);
    }

    if let Some(authority) = msg.authority {
        config.authority = deps.api.addr_validate(&authority)?;
    }

    if let Some(denoms_same_origin) = msg.denoms_same_origin {
        config.denoms_same_origin = denoms_same_origin;
    }

    if let Some(fee_collector) = msg.fee_collector {
        config.fee_collector = deps.api.addr_validate(&fee_collector)?;
    }

    if let Some(fee_rate) = msg.fee_rate {
        config.fee_rate = fee_rate;
    }

    if let Some(lp_fee_weight) = msg.lp_fee_weight {
        if Decimal::percent(100) < lp_fee_weight {
            return Err(ContractError::InvalidLpFeeWeight);
        }
        config.lp_fee_weight = lp_fee_weight;
    }

    PARAMS.save(deps.storage, &config)?;
    response = response.add_attribute("action", "update_params");

    Ok(response)
}
