use crate::error::ContractError;
use crate::state::BONDEDS;
use crate::state::TOTAL_SHARE;
use crate::state::TOTAL_UNBONDING_SHARE;
use crate::state::UNBONDINGS;
use crate::types::Unbonding;
use cosmwasm_std::{DepsMut, Response, StdResult, Uint128};
use cosmwasm_std::{Env, MessageInfo};
use strategy::v1::msgs::UnstakeMsg;

#[cfg(not(feature = "library"))]
pub fn execute_unstake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UnstakeMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let share = msg.share_amount;
    let total_share = TOTAL_SHARE.load(deps.storage)?;

    if share > total_share {
        return Err(ContractError::InsufficientFunds {});
    }

    let mut bonded = BONDEDS.load(deps.storage, info.sender.clone())?;
    if bonded.share < share {
        return Err(ContractError::InsufficientFunds {});
    }
    bonded.share = bonded.share.checked_sub(share)?;

    BONDEDS.save(deps.storage, info.sender.clone(), &bonded)?;

    let recipient = match msg.recipient {
        Some(recipient) => deps.api.addr_validate(&recipient)?,
        None => info.sender.clone(),
    };

    UNBONDINGS.update(
        deps.storage,
        recipient.clone(),
        |unstake_request: Option<Unbonding>| -> StdResult<_> {
            Ok(Unbonding {
                address: recipient,
                share: match unstake_request {
                    Some(unstake_request) => unstake_request.share.checked_add(share)?,
                    None => share,
                },
            })
        },
    )?;
    // TOTAL_SHARE includes TOTAL_UNBONDING_SHARE, so no need to subtract it.
    TOTAL_UNBONDING_SHARE.update(deps.storage, |total_unbonding: Uint128| -> StdResult<_> {
        Ok(total_unbonding.checked_add(share)?)
    })?;

    response = response
        .add_attribute("action", "unstake")
        .add_attribute("sender", info.sender);

    Ok(response)
}
