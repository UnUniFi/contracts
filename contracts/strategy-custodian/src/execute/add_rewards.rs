use crate::error::ContractError;
use crate::state::CONFIG;
use cosmwasm_std::{Coin, DepsMut, Response};

#[cfg(not(feature = "library"))]
pub fn execute_add_rewards(deps: DepsMut, coin: Coin) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let amount = coin.amount;
    let old_total_deposit = config.total_deposit;
    config.total_deposit += amount;
    config.redemption_rate = config.redemption_rate * config.total_deposit / old_total_deposit;
    CONFIG.save(deps.storage, &config)?;
    let rsp = Response::default()
        .add_attribute("action", "add_rewards")
        .add_attribute("amount", amount);
    Ok(rsp)
}
