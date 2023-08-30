use crate::error::ContractError;
use crate::state::BONDEDS;
use crate::state::TOTAL_SHARE;
use crate::state::TOTAL_UNBONDING;
use crate::state::UNBONDINGS;
use crate::types::Bonded;
use crate::types::Unbonding;
use cosmwasm_std::{DepsMut, Response, StdResult, Uint128};
use cosmwasm_std::{Env, MessageInfo};
use strategy::v0::msgs::UnstakeMsg;

#[cfg(not(feature = "library"))]
pub fn execute_unstake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UnstakeMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let share = msg.amount;
    let total_share = TOTAL_SHARE.load(deps.storage)?;

    if share > total_share {
        return Err(ContractError::InsufficientFunds {});
    }

    BONDEDS.update(
        deps.storage,
        info.sender.clone(),
        |deposit: Option<Bonded>| -> StdResult<_> {
            Ok(Bonded {
                address: info.sender.clone(),
                share: match deposit {
                    Some(deposit) => deposit.share.checked_sub(share)?,
                    None => Uint128::zero(),
                },
            })
        },
    )?;

    UNBONDINGS.update(
        deps.storage,
        info.sender.clone(),
        |unstake_request: Option<Unbonding>| -> StdResult<_> {
            Ok(Unbonding {
                address: info.sender.clone(),
                share: match unstake_request {
                    Some(unstake_request) => unstake_request.share.checked_add(share)?,
                    None => share,
                },
            })
        },
    )?;
    TOTAL_UNBONDING.update(deps.storage, |total_unbonding: Uint128| -> StdResult<_> {
        Ok(total_unbonding.checked_add(share)?)
    })?;

    response = response
        .add_attribute("action", "unstake")
        .add_attribute("sender", info.sender)
        .add_attribute("amount", msg.amount);

    Ok(response)
}
