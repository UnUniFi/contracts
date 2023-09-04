use crate::error::ContractError;
use crate::msgs::RepayMsg;
use crate::state::{bidding_key, listing_key, BIDDINGS, LISTINGS};
use cosmwasm_std::{coins, BankMsg, CosmosMsg, Response};
use cosmwasm_std::{DepsMut, Env, MessageInfo};
use cw_utils::one_coin;

// #[cfg(not(feature = "library"))]
pub fn execute_repay(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: RepayMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let coin = one_coin(&info)?;

    let bidding_key = bidding_key(msg.bid_id.clone());
    let bidding = BIDDINGS.load(deps.storage, bidding_key)?;

    let listing_key = listing_key(bidding.bid_id.nft_id);
    let mut listing = LISTINGS.load(deps.storage, listing_key.clone())?;

    if let None = &listing.borrowing {
        // TODO: change error
        return Err(ContractError::Unauthorized {});
    }
    let borrowing = listing.borrowing.unwrap();

    if coin.amount < borrowing.total_repay_amount {
        // TODO: change error
        return Err(ContractError::Unauthorized {});
    }

    let remaining_amount = coin.amount.checked_sub(borrowing.repay_amount)?;

    response = response.add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: bidding.address.to_string(),
        amount: coins(borrowing.repay_amount.u128(), coin.denom.clone()),
    }));

    response = response.add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: bidding.address.to_string(),
        amount: coins(remaining_amount.u128(), coin.denom.clone()),
    }));

    listing.borrowing = None;
    LISTINGS.save(deps.storage, listing_key, &listing)?;

    response = response.add_attribute("action", "repay");

    Ok(response)
}
