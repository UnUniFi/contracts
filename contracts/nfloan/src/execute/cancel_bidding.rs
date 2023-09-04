use crate::error::ContractError;
use crate::msgs::CancelBiddingMsg;
use crate::state::{bidding_key, listing_key, BIDDINGS, LISTINGS};
use cosmwasm_std::Response;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

// #[cfg(not(feature = "library"))]
pub fn execute_cancel_bidding(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: CancelBiddingMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let bidding_key = bidding_key(msg.bid_id.clone());
    let bidding = BIDDINGS.load(deps.storage, bidding_key.clone())?;

    if bidding.address != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let listing_key = listing_key(msg.bid_id.nft_id.clone());
    let listing = LISTINGS.load(deps.storage, listing_key)?;

    if let Some(borrowing) = &listing.borrowing {
        if borrowing.bid_id == msg.bid_id {
            // TODO: change error
            return Err(ContractError::Unauthorized {});
        }
    }

    BIDDINGS.remove(deps.storage, bidding_key);

    response = response.add_attribute("action", "cancel_bidding");

    Ok(response)
}
