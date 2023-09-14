use crate::msgs::UpdateParamsMsg;
use crate::state::PARAMS;
use crate::{error::ContractError, types::Params};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use ununifi_binding::v1::binding::UnunifiMsg;

// #[cfg(not(feature = "library"))]
pub fn execute_update_params(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateParamsMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut response = Response::new();
    let mut params: Params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(authority) = msg.authority {
        params.authority = deps.api.addr_validate(&authority)?;
    }

    if let Some(denom_swap_contract_map) = msg.denom_swap_contract_map {
        let denom_swap_contract_map = denom_swap_contract_map
            .iter()
            .map(|(k, v)| -> Result<_, ContractError> {
                Ok((k.clone(), deps.api.addr_validate(v)?))
            })
            .collect::<Result<_, ContractError>>()?;

        params.denom_swap_contract_map = denom_swap_contract_map;
    }

    PARAMS.save(deps.storage, &params)?;
    response = response
        .add_attribute("action", "update_params")
        .add_attribute("authority", params.authority.to_string());

    Ok(response)
}
