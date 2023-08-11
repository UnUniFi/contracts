use crate::error::ContractError;
use crate::msgs::SendBackMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_send_back(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: SendBackMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    // https://docs.axelar.dev/dev/general-message-passing/cosmos-gmp

    // Mint voucher

    Ok(response)
}
