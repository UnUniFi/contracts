use crate::error::ContractError;
use crate::msgs::CancelListingMsg;
use crate::state::{listing_key, LISTINGS};
use cosmwasm_std::Response;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

// #[cfg(not(feature = "library"))]
pub fn execute_cancel_listing(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: CancelListingMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let listing_key = listing_key(msg.nft_id);
    let listing = LISTINGS.load(deps.storage, listing_key.clone())?;

    if listing.address != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(_) = &listing.borrowing {
        // TODO: change error
        return Err(ContractError::Unauthorized {});
    }

    LISTINGS.remove(deps.storage, listing_key);

    response = response.add_attribute("action", "cancel_listing");

    Ok(response)
}
