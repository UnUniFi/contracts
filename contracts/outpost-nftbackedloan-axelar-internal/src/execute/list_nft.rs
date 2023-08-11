use crate::error::ContractError;
use crate::msgs::ListNftMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_list_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ListNftMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    // https://docs.axelar.dev/dev/general-message-passing/cosmos-gmp

    // Mint voucher

    Ok(response)
}
