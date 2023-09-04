use crate::error::ContractError;
use crate::msgs::BidMsg;
use crate::state::{bidding_key, BIDDINGS, LISTINGS};
use crate::types::{BidId, Bidding};
use cosmwasm_std::Response;
use cosmwasm_std::{DepsMut, Env, MessageInfo};
use cw_utils::one_coin;

// #[cfg(not(feature = "library"))]
pub fn execute_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: BidMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let coin = one_coin(&info)?;

    let listing_key = (msg.nft_id.class_id, msg.nft_id.token_id);
    let listing = LISTINGS.load(deps.storage, listing_key)?;

    if listing.bid_denom != coin.denom {
        return Err(ContractError::BidDenomMismatch {});
    }

    if msg.expiry < env.block.time {
        // TODO: change error
        return Err(ContractError::Unauthorized {});
    }

    let address = match msg.bidder {
        Some(bidder) => deps.api.addr_validate(&bidder)?,
        None => info.sender,
    };
    let bid_id = BidId {
        nft_id: listing.nft_id.clone(),
        bidder: address.clone(),
    };
    let bidding_key = bidding_key(bid_id.clone());

    if BIDDINGS.has(deps.storage, bidding_key.clone()) {
        return Err(ContractError::BidAlreadyExists {});
    }

    let bidding = Bidding {
        bid_id,
        address,
        expiry: msg.expiry,
        annual_interest_rate: msg.interest_rate,
        amount: coin.amount,
    };

    BIDDINGS.save(deps.storage, bidding_key, &bidding)?;

    response = response.add_attribute("action", "bid");

    Ok(response)
}
