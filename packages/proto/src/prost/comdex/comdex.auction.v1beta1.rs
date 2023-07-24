// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SurplusAuction {
    #[prost(uint64, tag = "1")]
    pub auction_id: u64,
    #[prost(message, optional, tag = "2")]
    pub sell_token: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
    pub buy_token: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "4")]
    pub active_bidding_id: u64,
    #[prost(string, tag = "5")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub bid: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "8")]
    pub bid_factor: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "9")]
    pub bidding_ids: ::prost::alloc::vec::Vec<BidOwnerMapping>,
    #[prost(uint64, tag = "10")]
    pub auction_status: u64,
    #[prost(uint64, tag = "11")]
    pub app_id: u64,
    #[prost(uint64, tag = "12")]
    pub asset_id: u64,
    #[prost(uint64, tag = "13")]
    pub auction_mapping_id: u64,
    #[prost(uint64, tag = "14")]
    pub asset_in_id: u64,
    #[prost(uint64, tag = "15")]
    pub asset_out_id: u64,
    #[prost(message, optional, tag = "16")]
    pub bid_end_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebtAuction {
    #[prost(uint64, tag = "1")]
    pub auction_id: u64,
    #[prost(message, optional, tag = "2")]
    pub auctioned_token: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
    pub expected_user_token:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub expected_minted_token:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "6")]
    pub active_bidding_id: u64,
    #[prost(string, tag = "7")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub current_bid_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "9")]
    pub auction_status: u64,
    #[prost(uint64, tag = "10")]
    pub app_id: u64,
    #[prost(uint64, tag = "11")]
    pub asset_id: u64,
    #[prost(message, repeated, tag = "12")]
    pub bidding_ids: ::prost::alloc::vec::Vec<BidOwnerMapping>,
    #[prost(uint64, tag = "13")]
    pub auction_mapping_id: u64,
    #[prost(string, tag = "14")]
    pub bid_factor: ::prost::alloc::string::String,
    #[prost(uint64, tag = "15")]
    pub asset_in_id: u64,
    #[prost(uint64, tag = "16")]
    pub asset_out_id: u64,
    #[prost(message, optional, tag = "17")]
    pub bid_end_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DutchAuction {
    #[prost(uint64, tag = "1")]
    pub auction_id: u64,
    #[prost(message, optional, tag = "2")]
    pub outflow_token_init_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
    pub outflow_token_current_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub inflow_token_target_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub inflow_token_current_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "6")]
    pub outflow_token_initial_price: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub outflow_token_current_price: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub outflow_token_end_price: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub inflow_token_current_price: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "10")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "11")]
    pub auction_status: u64,
    #[prost(message, optional, tag = "12")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag = "13")]
    pub bidding_ids: ::prost::alloc::vec::Vec<BidOwnerMapping>,
    #[prost(uint64, tag = "14")]
    pub auction_mapping_id: u64,
    #[prost(uint64, tag = "15")]
    pub app_id: u64,
    #[prost(uint64, tag = "16")]
    pub asset_in_id: u64,
    #[prost(uint64, tag = "17")]
    pub asset_out_id: u64,
    #[prost(uint64, tag = "18")]
    pub locked_vault_id: u64,
    #[prost(string, tag = "19")]
    pub vault_owner: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub liquidation_penalty: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidOwnerMapping {
    #[prost(uint64, tag = "1")]
    pub bid_id: u64,
    #[prost(string, tag = "2")]
    pub bid_owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolStatistics {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(string, tag = "3")]
    pub loss: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuctionParams {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_duration_seconds: u64,
    #[prost(string, tag = "3")]
    pub buffer: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub cusp: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub step: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub price_function_type: u64,
    #[prost(uint64, tag = "7")]
    pub surplus_id: u64,
    #[prost(uint64, tag = "8")]
    pub debt_id: u64,
    #[prost(uint64, tag = "9")]
    pub dutch_id: u64,
    #[prost(uint64, tag = "10")]
    pub bid_duration_seconds: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SurplusBiddings {
    #[prost(uint64, tag = "1")]
    pub bidding_id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_id: u64,
    #[prost(string, tag = "3")]
    pub auction_status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub auctioned_collateral:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "5")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub bid: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "7")]
    pub bidding_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "8")]
    pub bidding_status: ::prost::alloc::string::String,
    #[prost(uint64, tag = "9")]
    pub auction_mapping_id: u64,
    #[prost(uint64, tag = "10")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebtBiddings {
    #[prost(uint64, tag = "1")]
    pub bidding_id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_id: u64,
    #[prost(string, tag = "3")]
    pub auction_status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub outflow_tokens: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "5")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub bid: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "7")]
    pub bidding_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "8")]
    pub bidding_status: ::prost::alloc::string::String,
    #[prost(uint64, tag = "9")]
    pub auction_mapping_id: u64,
    #[prost(uint64, tag = "10")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DutchBiddings {
    #[prost(uint64, tag = "1")]
    pub bidding_id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_id: u64,
    #[prost(string, tag = "3")]
    pub auction_status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub outflow_token_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub inflow_token_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "6")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub bidding_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "8")]
    pub bidding_status: ::prost::alloc::string::String,
    #[prost(uint64, tag = "9")]
    pub auction_mapping_id: u64,
    #[prost(uint64, tag = "10")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub surplus_auction: ::prost::alloc::vec::Vec<SurplusAuction>,
    #[prost(message, repeated, tag = "2")]
    pub debt_auction: ::prost::alloc::vec::Vec<DebtAuction>,
    #[prost(message, repeated, tag = "3")]
    pub dutch_auction: ::prost::alloc::vec::Vec<DutchAuction>,
    #[prost(message, repeated, tag = "4")]
    pub protocol_statistics: ::prost::alloc::vec::Vec<ProtocolStatistics>,
    #[prost(message, repeated, tag = "5")]
    pub auction_params: ::prost::alloc::vec::Vec<AuctionParams>,
    #[prost(message, repeated, tag = "6")]
    pub dutch_lend_auction: ::prost::alloc::vec::Vec<DutchAuction>,
    #[prost(message, optional, tag = "7")]
    pub params: ::core::option::Option<Params>,
    #[prost(uint64, tag = "8")]
    pub user_bidding_id: u64,
}
// SURPLUS

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySurplusAuctionRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_mapping_id: u64,
    #[prost(uint64, tag = "3")]
    pub auction_id: u64,
    #[prost(bool, tag = "4")]
    pub history: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySurplusAuctionResponse {
    #[prost(message, optional, tag = "1")]
    pub auction: ::core::option::Option<SurplusAuction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySurplusAuctionsRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(bool, tag = "2")]
    pub history: bool,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySurplusAuctionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub auctions: ::prost::alloc::vec::Vec<SurplusAuction>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySurplusBiddingsRequest {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(bool, tag = "3")]
    pub history: bool,
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySurplusBiddingsResponse {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub biddings: ::prost::alloc::vec::Vec<SurplusBiddings>,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
// DEBT

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDebtAuctionRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_mapping_id: u64,
    #[prost(uint64, tag = "3")]
    pub auction_id: u64,
    #[prost(bool, tag = "4")]
    pub history: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDebtAuctionResponse {
    #[prost(message, optional, tag = "1")]
    pub auction: ::core::option::Option<DebtAuction>,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDebtAuctionsRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(bool, tag = "2")]
    pub history: bool,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDebtAuctionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub auctions: ::prost::alloc::vec::Vec<DebtAuction>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDebtBiddingsRequest {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(bool, tag = "3")]
    pub history: bool,
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDebtBiddingsResponse {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub biddings: ::prost::alloc::vec::Vec<DebtBiddings>,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
// DUTCH

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchAuctionRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_mapping_id: u64,
    #[prost(uint64, tag = "3")]
    pub auction_id: u64,
    #[prost(bool, tag = "4")]
    pub history: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchAuctionResponse {
    #[prost(message, optional, tag = "1")]
    pub auction: ::core::option::Option<DutchAuction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchAuctionsRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(bool, tag = "2")]
    pub history: bool,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchAuctionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub auctions: ::prost::alloc::vec::Vec<DutchAuction>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchBiddingsRequest {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(bool, tag = "3")]
    pub history: bool,
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchBiddingsResponse {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub biddings: ::prost::alloc::vec::Vec<DutchBiddings>,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBiddingsForSurplusAuctionRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_mapping_id: u64,
    #[prost(uint64, tag = "3")]
    pub auction_id: u64,
    #[prost(bool, tag = "4")]
    pub history: bool,
    #[prost(message, optional, tag = "5")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBiddingsForSurplusAuctionResponse {
    #[prost(message, repeated, tag = "1")]
    pub biddings: ::prost::alloc::vec::Vec<SurplusBiddings>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProtocolStatisticsRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProtocolStatisticsResponse {
    #[prost(message, repeated, tag = "1")]
    pub stats: ::prost::alloc::vec::Vec<ProtocolStatistics>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGenericAuctionParamRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGenericAuctionParamResponse {
    #[prost(message, optional, tag = "1")]
    pub auction_params: ::core::option::Option<AuctionParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchLendAuctionRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_mapping_id: u64,
    #[prost(uint64, tag = "3")]
    pub auction_id: u64,
    #[prost(bool, tag = "4")]
    pub history: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchLendAuctionResponse {
    #[prost(message, optional, tag = "1")]
    pub auction: ::core::option::Option<DutchAuction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchLendAuctionsRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(bool, tag = "2")]
    pub history: bool,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchLendAuctionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub auctions: ::prost::alloc::vec::Vec<DutchAuction>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchLendBiddingsRequest {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(bool, tag = "3")]
    pub history: bool,
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDutchLendBiddingsResponse {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub biddings: ::prost::alloc::vec::Vec<DutchBiddings>,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFilterDutchAuctionsRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, repeated, tag = "2")]
    pub denom: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "3")]
    pub history: bool,
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFilterDutchAuctionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub auctions: ::prost::alloc::vec::Vec<DutchAuction>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceSurplusBidRequest {
    #[prost(uint64, tag = "1")]
    pub auction_id: u64,
    #[prost(string, tag = "2")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
    #[prost(uint64, tag = "5")]
    pub auction_mapping_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceSurplusBidResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceDebtBidRequest {
    #[prost(uint64, tag = "1")]
    pub auction_id: u64,
    #[prost(string, tag = "2")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub bid: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub expected_user_token:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "5")]
    pub app_id: u64,
    #[prost(uint64, tag = "6")]
    pub auction_mapping_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceDebtBidResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceDutchBidRequest {
    #[prost(uint64, tag = "1")]
    pub auction_id: u64,
    #[prost(string, tag = "2")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
    #[prost(uint64, tag = "5")]
    pub auction_mapping_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceDutchBidResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceDutchLendBidRequest {
    #[prost(uint64, tag = "1")]
    pub auction_id: u64,
    #[prost(string, tag = "2")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
    #[prost(uint64, tag = "5")]
    pub auction_mapping_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceDutchLendBidResponse {}
include!("comdex.auction.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
