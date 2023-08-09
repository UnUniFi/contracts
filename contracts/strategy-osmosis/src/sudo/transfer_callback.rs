use crate::epoch::execute_epoch;
use crate::error::ContractError;
use crate::state::EpochCallSource;
use cosmwasm_std::{DepsMut, Env, Response};
use ununifi_binding::v0::binding::UnunifiMsg;

pub fn sudo_transfer_callback(
    deps: DepsMut,
    env: Env,
    _denom: String,
    _amount: String,
    _sender: String,
    _receiver: String,
    _memo: String,
    success: bool,
) -> Result<Response<UnunifiMsg>, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: sudo_transfer_callback received",).as_str());
    execute_epoch(deps, env, EpochCallSource::TransferCallback, success, None)?;
    let res = Response::new().add_attribute("action", "ibc_transfer_callback".to_string());
    return Ok(res);
}
