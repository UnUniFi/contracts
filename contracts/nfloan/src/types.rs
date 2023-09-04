use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Timestamp, Uint128};

#[cw_serde]
pub struct Params {
    pub authority: Addr,
    pub fee_collector: Addr,
    pub selling_fee_rate: Decimal,
    pub interest_fee_rate: Decimal,
}

#[cw_serde]
pub struct Listing {
    pub nft_id: NftId,
    pub address: Addr,
    pub bid_denom: String,
    pub borrowing: Option<Borrowing>,
}

#[cw_serde]
pub struct Borrowing {
    pub bid_id: BidId,
    pub expiry: Timestamp,
    pub repay_amount: Uint128,
    pub fee_amount: Uint128,
    pub total_repay_amount: Uint128,
}

#[cw_serde]
pub struct Bidding {
    pub bid_id: BidId,
    pub address: Addr,
    pub expiry: Timestamp,
    pub annual_interest_rate: Decimal,
    pub amount: Uint128,
}

#[cw_serde]
pub struct NftId {
    pub class_id: String,
    pub token_id: String,
}

#[cw_serde]
pub struct BidId {
    pub nft_id: NftId,
    pub bidder: Addr,
}
