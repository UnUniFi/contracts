use crate::error::ContractError;
use crate::msgs::ListNftMsg;
use crate::state::LISTINGS;
use crate::types::Listing;
use cosmwasm_std::Response;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

// #[cfg(not(feature = "library"))]
pub fn execute_list_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ListNftMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    // TODO: check that the NFT is owned by this contract
    // In practical, users need to send tx with two msgs: sending nft to this contract and wasm execute msg for this.

    let address = match msg.lister {
        Some(lister) => deps.api.addr_validate(&lister)?,
        None => info.sender,
    };

    let nft_id = msg.nft_id.clone();
    let listing_key = (nft_id.class_id, nft_id.token_id);
    let listing = Listing {
        address,
        nft_id: msg.nft_id.clone(),
        bid_denom: msg.bid_denom,
        borrowing: None,
    };

    LISTINGS.save(deps.storage, listing_key, &listing)?;

    response = response.add_attribute("action", "list_nft");

    Ok(response)
}
