use crate::error::ContractError;
use crate::msgs::SendBackMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_send_back(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SendBackMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    // TODO: send IBC packet to the external contract

    Ok(response)
}
