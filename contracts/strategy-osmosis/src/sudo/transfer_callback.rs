use crate::state::EpochCallSource;
use crate::{error::ContractError, execute::epoch::epoch::execute_epoch};
use cosmwasm_std::{DepsMut, Env, Response};
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn sudo_transfer_callback(
    deps: DepsMut,
    env: Env,
    denom: String,
    amount: String,
    sender: String,
    receiver: String,
    _memo: String,
    success: bool,
) -> Result<Response<UnunifiMsg>, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: sudo_transfer_callback received",).as_str());
    execute_epoch(deps, env, EpochCallSource::TransferCallback, success, None)?;
    let res = Response::new()
        .add_attribute("action", "ibc_transfer_callback".to_string())
        .add_attribute("denom", denom.to_string())
        .add_attribute("amount", amount.to_string())
        .add_attribute("sender", sender.to_string())
        .add_attribute("receiver", receiver.to_string())
        .add_attribute("success", success.to_string());
    return Ok(res);
}
