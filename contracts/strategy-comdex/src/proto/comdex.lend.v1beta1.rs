// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendAsset {
    #[prost(uint64, tag = "1")]
    pub lending_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(uint64, tag = "3")]
    pub pool_id: u64,
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub amount_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub lending_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "7")]
    pub available_to_borrow: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub app_id: u64,
    #[prost(string, tag = "9")]
    pub global_index: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "10")]
    pub last_interaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "11")]
    pub cpool_name: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub total_rewards: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowAsset {
    #[prost(uint64, tag = "1")]
    pub borrowing_id: u64,
    #[prost(uint64, tag = "2")]
    pub lending_id: u64,
    #[prost(bool, tag = "3")]
    pub is_stable_borrow: bool,
    #[prost(uint64, tag = "4")]
    pub pair_id: u64,
    #[prost(message, optional, tag = "5")]
    pub amount_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub amount_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "7")]
    pub bridged_asset_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "8")]
    pub borrowing_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "9")]
    pub stable_borrow_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub interest_accumulated: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub global_index: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub reserve_global_index: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "13")]
    pub last_interaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "14")]
    pub cpool_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "15")]
    pub is_liquidated: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cpool_name: ::prost::alloc::string::String,
    // string reserve_funds = 4 [
    //    (gogoproto.customtype) = "github.com/cosmos/cosmos-sdk/types.Dec",
    //    (gogoproto.nullable) = false,
    //    (gogoproto.moretags) = "yaml:\"reserve_funds\""
    // ];
    #[prost(message, repeated, tag = "4")]
    pub asset_data: ::prost::alloc::vec::Vec<AssetDataPoolMapping>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAssetLendBorrowMapping {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// to check if poool id is needed
    #[prost(uint64, tag = "2")]
    pub lend_id: u64,
    #[prost(uint64, tag = "3")]
    pub pool_id: u64,
    #[prost(uint64, repeated, packed = "false", tag = "4")]
    pub borrow_id: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetDataPoolMapping {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    /// 1 for main_asset, 2 for 1st transit_asset, 3 for 2nd transit_asset
    #[prost(uint64, tag = "2")]
    pub asset_transit_type: u64,
    #[prost(string, tag = "3")]
    pub supply_cap: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedPair {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_in: u64,
    #[prost(uint64, tag = "3")]
    pub asset_out: u64,
    #[prost(bool, tag = "4")]
    pub is_inter_pool: bool,
    #[prost(uint64, tag = "5")]
    pub asset_out_pool_id: u64,
    #[prost(uint64, tag = "6")]
    pub min_usd_value_left: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetToPairMapping {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub pair_id: ::prost::alloc::vec::Vec<u64>,
}
/// AssetStats
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolAssetLbMapping {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub lend_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, packed = "false", tag = "4")]
    pub borrow_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag = "5")]
    pub total_borrowed: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub total_stable_borrowed: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub total_lend: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub total_interest_accumulated: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub lend_apr: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub borrow_apr: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub stable_borrow_apr: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub utilisation_ratio: ::prost::alloc::string::String,
}
/// AssetRatesStats
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetRatesParams {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(string, tag = "2")]
    pub u_optimal: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub slope1: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub slope2: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub enable_stable_borrow: bool,
    #[prost(string, tag = "7")]
    pub stable_base: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub stable_slope1: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub stable_slope2: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub ltv: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub liquidation_threshold: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub liquidation_penalty: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub liquidation_bonus: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub reserve_factor: ::prost::alloc::string::String,
    #[prost(uint64, tag = "15")]
    pub c_asset_id: u64,
}
/// BalanceStats
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveBuybackAssetData {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(string, tag = "2")]
    pub reserve_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub buyback_amount: ::prost::alloc::string::String,
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
    pub dutch_id: u64,
    #[prost(uint64, tag = "8")]
    pub bid_duration_seconds: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowInterestTracker {
    #[prost(uint64, tag = "1")]
    pub borrowing_id: u64,
    #[prost(string, tag = "3")]
    pub reserve_pool_interest: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendRewardsTracker {
    #[prost(uint64, tag = "1")]
    pub lending_id: u64,
    #[prost(string, tag = "2")]
    pub rewards_accumulated: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBalance {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub module_balance_stats: ::prost::alloc::vec::Vec<ModuleBalanceStats>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBalanceStats {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModBal {
    #[prost(message, repeated, tag = "1")]
    pub fund_module_balance: ::prost::alloc::vec::Vec<FundModBal>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveBal {
    #[prost(message, repeated, tag = "1")]
    pub fund_reserve_balance: ::prost::alloc::vec::Vec<FundReserveBal>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundModBal {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "3")]
    pub amount_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub deposit_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "5")]
    pub funder: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundReserveBal {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(message, optional, tag = "2")]
    pub amount_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
    pub deposit_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub funder: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllReserveStats {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(string, tag = "2")]
    pub amount_out_from_reserve_to_lenders: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount_out_from_reserve_for_auction: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount_in_from_liq_penalty: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub amount_in_from_repayments: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub total_amount_out_to_lenders: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetToPairSingleMapping {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(uint64, tag = "3")]
    pub pair_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolPairs {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cpool_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub asset_data: ::prost::alloc::vec::Vec<AssetDataPoolMapping>,
    #[prost(uint64, tag = "5")]
    pub min_usd_value_left: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInterestData {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(string, tag = "2")]
    pub lend_interest: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInterest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub pool_interest_data: ::prost::alloc::vec::Vec<PoolInterestData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInterestDataB {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(string, tag = "2")]
    pub borrow_interest: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInterestB {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub pool_interest_data: ::prost::alloc::vec::Vec<PoolInterestDataB>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetRatesPoolPairs {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(string, tag = "2")]
    pub u_optimal: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub slope1: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub slope2: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub enable_stable_borrow: bool,
    #[prost(string, tag = "7")]
    pub stable_base: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub stable_slope1: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub stable_slope2: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub ltv: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub liquidation_threshold: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub liquidation_penalty: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub liquidation_bonus: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub reserve_factor: ::prost::alloc::string::String,
    #[prost(uint64, tag = "15")]
    pub c_asset_id: u64,
    #[prost(string, tag = "16")]
    pub module_name: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub cpool_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "18")]
    pub asset_data: ::prost::alloc::vec::Vec<AssetDataPoolMapping>,
    #[prost(uint64, tag = "19")]
    pub min_usd_value_left: u64,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub borrow_asset: ::prost::alloc::vec::Vec<BorrowAsset>,
    #[prost(message, repeated, tag = "2")]
    pub borrow_interest_tracker: ::prost::alloc::vec::Vec<BorrowInterestTracker>,
    #[prost(message, repeated, tag = "3")]
    pub lend_asset: ::prost::alloc::vec::Vec<LendAsset>,
    #[prost(message, repeated, tag = "4")]
    pub pool: ::prost::alloc::vec::Vec<Pool>,
    #[prost(message, repeated, tag = "5")]
    pub asset_to_pair_mapping: ::prost::alloc::vec::Vec<AssetToPairMapping>,
    #[prost(message, repeated, tag = "6")]
    pub pool_asset_lb_mapping: ::prost::alloc::vec::Vec<PoolAssetLbMapping>,
    #[prost(message, repeated, tag = "7")]
    pub lend_rewards_tracker: ::prost::alloc::vec::Vec<LendRewardsTracker>,
    #[prost(message, repeated, tag = "8")]
    pub user_asset_lend_borrow_mapping: ::prost::alloc::vec::Vec<UserAssetLendBorrowMapping>,
    #[prost(message, repeated, tag = "9")]
    pub reserve_buyback_asset_data: ::prost::alloc::vec::Vec<ReserveBuybackAssetData>,
    #[prost(message, repeated, tag = "10")]
    pub extended_pair: ::prost::alloc::vec::Vec<ExtendedPair>,
    #[prost(message, repeated, tag = "11")]
    pub auction_params: ::prost::alloc::vec::Vec<AuctionParams>,
    #[prost(message, repeated, tag = "12")]
    pub asset_rates_params: ::prost::alloc::vec::Vec<AssetRatesParams>,
    #[prost(message, optional, tag = "13")]
    pub mod_bal: ::core::option::Option<ModBal>,
    #[prost(message, optional, tag = "14")]
    pub reserve_bal: ::core::option::Option<ReserveBal>,
    #[prost(message, repeated, tag = "15")]
    pub all_reserve_stats: ::prost::alloc::vec::Vec<AllReserveStats>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendPairsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pairs: ::core::option::Option<ExtendedPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultipleLendPairsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub pairs: ::prost::alloc::vec::Vec<ExtendedPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPoolsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pool: ::core::option::Option<Pool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAssetToPairProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub asset_to_pair_mapping: ::core::option::Option<AssetToPairMapping>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMultipleAssetToPairProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub asset_to_pair_single_mapping: ::prost::alloc::vec::Vec<AssetToPairSingleMapping>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAssetRatesParams {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub asset_rates_params: ::core::option::Option<AssetRatesParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAuctionParamsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub auction_params: ::core::option::Option<AuctionParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPoolPairsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pool_pairs: ::core::option::Option<PoolPairs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAssetRatesPoolPairsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub asset_rates_pool_pairs: ::core::option::Option<AssetRatesPoolPairs>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLendsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLendsResponse {
    #[prost(message, repeated, tag = "1")]
    pub lends: ::prost::alloc::vec::Vec<LendAsset>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLendRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLendResponse {
    #[prost(message, optional, tag = "1")]
    pub lend: ::core::option::Option<LendAsset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllLendByOwnerRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllLendByOwnerResponse {
    #[prost(message, repeated, tag = "1")]
    pub lends: ::prost::alloc::vec::Vec<LendAsset>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllLendByOwnerAndPoolRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllLendByOwnerAndPoolResponse {
    #[prost(message, repeated, tag = "1")]
    pub lends: ::prost::alloc::vec::Vec<LendAsset>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPairsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPairsResponse {
    #[prost(message, repeated, tag = "1")]
    pub extended_pairs: ::prost::alloc::vec::Vec<ExtendedPair>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPairRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPairResponse {
    #[prost(message, optional, tag = "1")]
    pub extended_pair: ::core::option::Option<ExtendedPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetRatesParamsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetRatesParamsResponse {
    #[prost(message, repeated, tag = "1")]
    pub asset_rates_params: ::prost::alloc::vec::Vec<AssetRatesParams>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetRatesParamRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetRatesParamResponse {
    #[prost(message, optional, tag = "1")]
    pub asset_ratesparams: ::core::option::Option<AssetRatesParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<Pool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetToPairMappingsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetToPairMappingsResponse {
    #[prost(message, repeated, tag = "1")]
    pub asset_to_pair_mappings: ::prost::alloc::vec::Vec<AssetToPairMapping>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetToPairMappingRequest {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetToPairMappingResponse {
    #[prost(message, optional, tag = "1")]
    pub asset_to_pair_mapping: ::core::option::Option<AssetToPairMapping>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBorrowsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBorrowsResponse {
    #[prost(message, repeated, tag = "1")]
    pub borrows: ::prost::alloc::vec::Vec<BorrowAsset>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBorrowRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBorrowResponse {
    #[prost(message, optional, tag = "1")]
    pub borrow: ::core::option::Option<BorrowAsset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBorrowByOwnerRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBorrowByOwnerResponse {
    #[prost(message, repeated, tag = "1")]
    pub borrows: ::prost::alloc::vec::Vec<BorrowAsset>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBorrowByOwnerAndPoolRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBorrowByOwnerAndPoolResponse {
    #[prost(message, repeated, tag = "1")]
    pub borrows: ::prost::alloc::vec::Vec<BorrowAsset>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolAssetLbMappingRequest {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolAssetLbMappingResponse {
    #[prost(message, optional, tag = "1")]
    pub pool_asset_lb_mapping: ::core::option::Option<PoolAssetLbMapping>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryReserveBuybackAssetDataRequest {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryReserveBuybackAssetDataResponse {
    #[prost(message, optional, tag = "1")]
    pub reserve_buyback_asset_data: ::core::option::Option<ReserveBuybackAssetData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionParamRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionParamResponse {
    #[prost(message, optional, tag = "1")]
    pub auction_params: ::core::option::Option<AuctionParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleBalanceRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleBalanceResponse {
    #[prost(message, optional, tag = "1")]
    pub module_balance: ::core::option::Option<ModuleBalance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFundModBalRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFundModBalResponse {
    #[prost(message, optional, tag = "1")]
    pub fund_mod_balance: ::core::option::Option<ModBal>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFundReserveBalRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFundReserveBalResponse {
    #[prost(message, optional, tag = "1")]
    pub fund_reserve_balance: ::core::option::Option<ReserveBal>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllReserveStatsRequest {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllReserveStatsResponse {
    #[prost(message, optional, tag = "1")]
    pub all_reserve_stats: ::core::option::Option<AllReserveStats>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFundModBalByAssetPoolRequest {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFundModBalByAssetPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLendInterestRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLendInterestResponse {
    #[prost(message, repeated, tag = "1")]
    pub pool_interest: ::prost::alloc::vec::Vec<PoolInterest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBorrowInterestRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBorrowInterestResponse {
    #[prost(message, repeated, tag = "1")]
    pub pool_interest: ::prost::alloc::vec::Vec<PoolInterestB>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLend {
    #[prost(string, tag = "1")]
    pub lender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "4")]
    pub pool_id: u64,
    #[prost(uint64, tag = "5")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdraw {
    #[prost(string, tag = "1")]
    pub lender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub lend_id: u64,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeposit {
    #[prost(string, tag = "1")]
    pub lender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub lend_id: u64,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCloseLend {
    #[prost(string, tag = "1")]
    pub lender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub lend_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBorrow {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub lend_id: u64,
    #[prost(uint64, tag = "3")]
    pub pair_id: u64,
    #[prost(bool, tag = "4")]
    pub is_stable_borrow: bool,
    #[prost(message, optional, tag = "5")]
    pub amount_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub amount_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRepay {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub borrow_id: u64,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositBorrow {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub borrow_id: u64,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDraw {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub borrow_id: u64,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCloseBorrow {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub borrow_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBorrowAlternate {
    #[prost(string, tag = "1")]
    pub lender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(uint64, tag = "3")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "4")]
    pub amount_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "5")]
    pub pair_id: u64,
    #[prost(bool, tag = "6")]
    pub is_stable_borrow: bool,
    #[prost(message, optional, tag = "7")]
    pub amount_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "8")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundModuleAccounts {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(string, tag = "3")]
    pub lender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCalculateInterestAndRewards {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundReserveAccounts {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(string, tag = "2")]
    pub lender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLendResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCloseLendResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBorrowResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRepayResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositBorrowResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDrawResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCloseBorrowResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBorrowAlternateResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundModuleAccountsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCalculateInterestAndRewardsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundReserveAccountsResponse {}
include!("comdex.lend.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
