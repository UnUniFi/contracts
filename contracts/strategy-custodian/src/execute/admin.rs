use crate::error::ContractError;
use crate::msgs::AdminMsg;
use crate::{state::PARAMS, types::Params};
use cosmwasm_std::{DepsMut, Response};
use cosmwasm_std::{Env, MessageInfo};

#[cfg(not(feature = "library"))]
pub fn execute_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: AdminMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let params: Params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.admin {
        return Err(ContractError::Unauthorized {});
    }

    response = response.add_messages(msg.cosmos_msgs);

    Ok(response)
}
