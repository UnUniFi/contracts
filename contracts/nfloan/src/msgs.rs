use crate::types::{BidId, NftId, Params};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Timestamp};

#[cw_serde]
pub struct InstantiateMsg {
    pub authority: String,
    pub fee_collector: String,
    pub selling_fee_rate: Decimal,
    pub interest_fee_rate: Decimal,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateParams(UpdateParamsMsg),
    ListNft(ListNftMsg),
    CancelListing(CancelListingMsg),
    Bid(BidMsg),
    CancelBidding(CancelBiddingMsg),
    Borrow(BorrowMsg),
    Repay(RepayMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub authority: Option<String>,
    pub fee_collector: Option<String>,
    pub selling_fee_rate: Option<Decimal>,
    pub interest_fee_rate: Option<Decimal>,
}

#[cw_serde]
pub struct ListNftMsg {
    pub nft_id: NftId,
    pub bid_denom: String,
    pub lister: Option<String>,
}

#[cw_serde]
pub struct CancelListingMsg {
    pub nft_id: NftId,
}

#[cw_serde]
pub struct BidMsg {
    pub nft_id: NftId,
    pub expiry: Timestamp,
    pub interest_rate: Decimal,
    pub bidder: Option<String>,
}

#[cw_serde]
pub struct CancelBiddingMsg {
    pub bid_id: BidId,
}

#[cw_serde]
pub struct BorrowMsg {
    pub bid_id: BidId,
}

#[cw_serde]
pub struct RepayMsg {
    pub bid_id: BidId,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Params)]
    Config {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
