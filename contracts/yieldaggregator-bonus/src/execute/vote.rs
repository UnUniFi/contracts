use crate::error::ContractError;
use crate::msgs::VoteMsg;
use crate::state::BONUS_WINDOWS;
use crate::state::VOTED_VAULTS;
use crate::types::VotedVault;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;

#[cfg(not(feature = "library"))]
pub fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: VoteMsg,
) -> Result<Response, ContractError> {
    use cosmwasm_std::StdResult;

    let mut response = Response::new();
    let coin = one_coin(&info)?;

    let bonus_window = BONUS_WINDOWS.load(deps.storage, msg.bonus_window_id)?;

    if bonus_window.denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }

    if env.block.time < bonus_window.start_at || bonus_window.end_at < env.block.time {
        return Err(ContractError::InvalidBonusWindowPeriod {});
    }

    VOTED_VAULTS.update(
        deps.storage,
        (msg.bonus_window_id, msg.vault_id),
        |voted_vault| -> StdResult<_> {
            match voted_vault {
                Some(mut voted_vault) => {
                    voted_vault.voted_amount += coin.amount;
                    Ok(voted_vault)
                }
                None => Ok(VotedVault {
                    bonus_window_id: msg.bonus_window_id,
                    vault_id: msg.vault_id,
                    voted_amount: coin.amount,
                }),
            }
        },
    )?;

    response = response.add_attribute("action", "vote");

    Ok(response)
}
