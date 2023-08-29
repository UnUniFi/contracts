use crate::error::ContractError;
use crate::state::PARAMS;
use cosmwasm_std::{Coin, DepsMut, Response};

#[cfg(not(feature = "library"))]
pub fn execute_add_rewards(deps: DepsMut, coin: Coin) -> Result<Response, ContractError> {
    let mut params = PARAMS.load(deps.storage)?;
    if params.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let amount = coin.amount;
    let old_total_deposit = params.total_deposit;
    params.total_deposit += amount;
    params.redemption_rate = params.redemption_rate * params.total_deposit / old_total_deposit;
    PARAMS.save(deps.storage, &params)?;
    let rsp = Response::default()
        .add_attribute("action", "add_rewards")
        .add_attribute("amount", amount);
    Ok(rsp)
}
