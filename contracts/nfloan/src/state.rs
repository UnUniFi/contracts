use crate::types::{BidId, Bidding, Listing, NftId, Params};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const PARAMS: Item<Params> = Item::new("params");

pub const LISTINGS: Map<(String, String), Listing> = Map::new("listings");

pub const BIDDINGS: Map<(String, String, Addr), Bidding> = Map::new("biddings");

pub fn listing_key(nft_id: NftId) -> (String, String) {
    (nft_id.class_id, nft_id.token_id)
}

pub fn bidding_key(bid_id: BidId) -> (String, String, Addr) {
    (
        bid_id.nft_id.class_id,
        bid_id.nft_id.token_id,
        bid_id.bidder,
    )
}
