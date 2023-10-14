use crate::error::ContractError;
use crate::msgs::DeleteBonusWindowMsg;
use crate::state::BONUS_WINDOWS;
use crate::state::PARAMS;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_delete_bonus_window(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: DeleteBonusWindowMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    let bonus_window = BONUS_WINDOWS.load(deps.storage, msg.bonus_window_id)?;

    // Check if the bonus window is already ended
    if env.block.time < bonus_window.end_at {
        return Err(ContractError::BonusWindowNotEndedYet {});
    }

    BONUS_WINDOWS.remove(deps.storage, msg.bonus_window_id);

    response = response.add_attribute("action", "delete_bonus_window");

    Ok(response)
}
