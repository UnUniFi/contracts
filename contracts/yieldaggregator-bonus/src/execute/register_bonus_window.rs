use crate::error::ContractError;
use crate::msgs::RegisterBonusWindowMsg;
use crate::state::BONUS_WINDOWS;
use crate::state::PARAMS;
use crate::types::BonusWindow;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_register_bonus_window(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: RegisterBonusWindowMsg,
) -> Result<Response, ContractError> {
    use cw_utils::one_coin;

    let mut response = Response::new();
    let params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    let deposit = one_coin(&info)?;
    if deposit.denom != msg.denom {
        return Err(ContractError::NoAllowedToken {});
    }

    if deposit.amount < msg.budget_for_all {
        return Err(ContractError::InsufficientBudget {});
    }

    let latest_id = BONUS_WINDOWS.last(deps.storage)
        .map(|res|  
            match res {
                Some((_, bw)) => bw.id.checked_add(1).unwrap(),
                None => 0,
            }
        )
        .unwrap_or(0);

    let bonus_window = BonusWindow {
        id: latest_id,
        denom: msg.denom,
        budget_for_all: msg.budget_for_all,
        apr_for_winners: msg.apr_for_winners,
        start_at: msg.start_at,
        end_at: msg.end_at,
    };
    BONUS_WINDOWS.save(deps.storage, latest_id, &bonus_window)?;

    response = response.add_attribute("action", "register_bonus_window");

    Ok(response)
}
