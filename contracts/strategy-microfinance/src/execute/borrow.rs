use crate::error::ContractError;
use crate::state::CONFIG;
use cosmwasm_std::{Addr, Coin, DepsMut, QueryRequest, Response};

#[cfg(not(feature = "library"))]
pub fn execute_borrow(deps: DepsMut, coin: Coin) -> Result<Response, ContractError> {
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

//
// check tx histories of MsgSend
// min(config.max_borrow_amount, max(received_amount_in_tx))
fn borrowable_amount(addr: Addr) {
    let req = QueryRequest::Stargate {
        path: "".to_string(),
        data: (),
    };
}
