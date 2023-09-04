use std::time::Duration;

use crate::error::ContractError;
use crate::msgs::BorrowMsg;
use crate::state::{bidding_key, listing_key, BIDDINGS, LISTINGS, PARAMS};
use crate::types::Borrowing;
use cosmwasm_std::{coins, BankMsg, CosmosMsg, Decimal, Response};
use cosmwasm_std::{DepsMut, Env, MessageInfo};

// #[cfg(not(feature = "library"))]
pub fn execute_borrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: BorrowMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let bidding_key = bidding_key(msg.bid_id.clone());
    let bidding = BIDDINGS.load(deps.storage, bidding_key)?;

    if bidding.expiry < env.block.time {
        // TODO: change error
        return Err(ContractError::Unauthorized {});
    }

    let listing_key = listing_key(bidding.bid_id.nft_id.clone());
    let mut listing = LISTINGS.load(deps.storage, listing_key.clone())?;

    if let Some(_) = &listing.borrowing {
        // TODO: change error
        return Err(ContractError::Unauthorized {});
    }

    response = response.add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.into_string(),
        amount: coins(bidding.amount.u128(), listing.bid_denom.clone()),
    }));

    let params = PARAMS.load(deps.storage)?;

    let duration = Duration::from_secs(env.block.time.seconds() - bidding.expiry.seconds());
    let one_year_duration = Duration::from_secs(365 * 24 * 60 * 60);

    let interest_rate = bidding
        .annual_interest_rate
        .checked_mul(Decimal::from_atomics(duration.as_secs(), 0)?)?
        .checked_div(Decimal::from_atomics(one_year_duration.as_secs(), 0)?)
        .unwrap();

    let interest_amount = bidding.amount * interest_rate;
    let repay_amount = bidding.amount + interest_amount;
    let fee_amount = interest_amount * params.interest_fee_rate;
    let total_repay_amount = repay_amount + fee_amount;

    listing.borrowing = Some(Borrowing {
        bid_id: bidding.bid_id,
        expiry: bidding.expiry,
        repay_amount,
        fee_amount,
        total_repay_amount,
    });

    LISTINGS.save(deps.storage, listing_key, &listing)?;

    response = response.add_attribute("action", "borrow");

    Ok(response)
}
