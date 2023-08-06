use crate::error::ContractError;
use crate::state::{DepositInfo, CONFIG, DEPOSITS};
use cosmwasm_std::{Addr, Coin, DepsMut, Response, StdResult, Uint128};

#[cfg(not(feature = "library"))]
pub fn execute_stake(deps: DepsMut, coin: Coin, sender: Addr) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let amount = coin.amount;
    let redemption_rate_multiplier = Uint128::from(1000000u128);
    let redemption_rate = config.redemption_rate;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let stake_amount = amount * redemption_rate_multiplier / redemption_rate;
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_add(stake_amount)?,
                });
            }
            Ok(DepositInfo {
                sender: sender.clone(),
                amount: amount,
            })
        },
    )?;
    config.total_deposit += amount;
    CONFIG.save(deps.storage, &config)?;

    let rsp = Response::default()
        .add_attribute("action", "stake")
        .add_attribute("sender", sender)
        .add_attribute("amount", amount);
    Ok(rsp)
}
