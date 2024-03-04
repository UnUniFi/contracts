use crate::error::ContractError;
use crate::msgs::UpdateParamsMsg;

use crate::state::PARAMS;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use ununifi_binding::v1::binding::UnunifiMsg;

/// Only authority can execute it.
pub fn execute_update_params(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateParamsMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    let mut resp = Response::new().add_attribute("action", "update_params");
    if let Some(authority) = msg.authority {
        params.authority = deps.api.addr_validate(&authority)?;
        resp = resp.add_attribute("authority", authority.to_string());
    }
    if let Some(denom) = msg.denom {
        params.denom = denom.to_owned();
        resp = resp.add_attribute("denom", denom.to_string());
    }
    if let Some(ls_denom) = msg.ls_denom {
        params.ls_denom = ls_denom.to_owned();
        resp = resp.add_attribute("ls_denom", ls_denom.to_string());
    }
    if let Some(ls_denom_native) = msg.ls_denom_native {
        params.ls_denom_native = ls_denom_native.to_owned();
        resp = resp.add_attribute("ls_denom_native", ls_denom_native.to_string());
    }
    if let Some(ls_rate_feeder) = msg.ls_rate_feeder {
        params.ls_rate_feeder = ls_rate_feeder.to_owned();
        resp = resp.add_attribute("ls_rate_feeder", ls_rate_feeder.to_string());
    }
    if let Some(chain_id) = msg.chain_id {
        params.chain_id = chain_id.to_owned();
        resp = resp.add_attribute("chain_id", chain_id.to_string());
    }
    if let Some(connection_id) = msg.connection_id {
        params.connection_id = connection_id.to_owned();
        resp = resp.add_attribute("connection_id", connection_id.to_string());
    }

    PARAMS.save(deps.storage, &params)?;
    Ok(resp)
}
