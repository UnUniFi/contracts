use crate::error::ContractError;
use crate::state::{BONDEDS, PARAMS};
use crate::state::{TOTAL_DEPOSIT, TOTAL_SHARE};
use crate::types::Bonded;
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_utils::one_coin;
use strategy::v1::msgs::StakeMsg;

#[cfg(not(feature = "library"))]
pub fn execute_stake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: StakeMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let coin: Coin = one_coin(&info)?;

    let params = PARAMS.load(deps.storage)?;
    if params.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }

    let amount = coin.amount;
    let total_deposit = TOTAL_DEPOSIT.load(deps.storage)?;
    let total_share = TOTAL_SHARE.load(deps.storage)?;
    let share = if total_share.is_zero() {
        amount
    } else {
        amount
            .checked_mul(total_share)?
            .checked_div(total_deposit)
            .unwrap()
    };

    BONDEDS.update(
        deps.storage,
        info.sender.clone(),
        |deposit: Option<Bonded>| -> StdResult<_> {
            Ok(Bonded {
                address: info.sender.clone(),
                share: match deposit {
                    Some(deposit) => deposit.share.checked_add(share)?,
                    None => share,
                },
            })
        },
    )?;

    TOTAL_DEPOSIT.save(deps.storage, &(total_deposit.checked_add(amount)?))?;
    TOTAL_SHARE.save(deps.storage, &(total_share.checked_add(share)?))?;

    response = response
        .add_attribute("action", "stake")
        .add_attribute("sender", info.sender)
        .add_attribute("amount", amount);
    Ok(response)
}
