use crate::error::ContractError;
use crate::msgs::DeleteBonusWindowMsg;
use crate::state::BONUS_WINDOWS;
use crate::state::PARAMS;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_delete_bonus_window(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: DeleteBonusWindowMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    BONUS_WINDOWS.remove(deps.storage, msg.bonus_window_id);

    response = response.add_attribute("action", "delete_bonus_window");

    Ok(response)
}
