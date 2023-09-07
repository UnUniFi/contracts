use crate::error::ContractError;
use crate::msgs::DistributeBonusMsg;
use crate::state::PARAMS;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_distribute_bonus(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: DistributeBonusMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    Ok(response)
}
