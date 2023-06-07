// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectorData {
    #[prost(string, tag = "1")]
    pub collected_stability_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collected_closing_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub collected_opening_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub liquidation_rewards_collected: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppAssetIdToFeeCollectedData {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(string, tag = "3")]
    pub net_fees_collected: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppToAssetIdCollectorMapping {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(message, optional, tag = "3")]
    pub collector: ::core::option::Option<CollectorData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectorLookupTableData {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub collector_asset_id: u64,
    #[prost(uint64, tag = "3")]
    pub secondary_asset_id: u64,
    #[prost(string, tag = "4")]
    pub surplus_threshold: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub debt_threshold: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub locker_saving_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub lot_size: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub bid_factor: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub debt_lot_size: ::prost::alloc::string::String,
    #[prost(int64, tag = "10")]
    pub block_height: i64,
    #[prost(message, optional, tag = "11")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppToDenomsMapping {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppAssetIdToAuctionLookupTable {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(bool, tag = "3")]
    pub is_surplus_auction: bool,
    #[prost(bool, tag = "4")]
    pub is_debt_auction: bool,
    #[prost(bool, tag = "5")]
    pub is_distributor: bool,
    #[prost(bool, tag = "6")]
    pub is_auction_active: bool,
    #[prost(bool, tag = "7")]
    pub asset_out_oracle_price: bool,
    #[prost(uint64, tag = "8")]
    pub asset_out_price: u64,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub net_fee_collected_data: ::prost::alloc::vec::Vec<AppAssetIdToFeeCollectedData>,
    #[prost(message, repeated, tag = "2")]
    pub app_id_to_asset_collector_mapping: ::prost::alloc::vec::Vec<AppToAssetIdCollectorMapping>,
    #[prost(message, repeated, tag = "3")]
    pub collector_lookup: ::prost::alloc::vec::Vec<CollectorLookupTableData>,
    #[prost(message, repeated, tag = "4")]
    pub collector_auction_lookup_table: ::prost::alloc::vec::Vec<AppAssetIdToAuctionLookupTable>,
    #[prost(message, repeated, tag = "5")]
    pub app_to_denoms_mapping: ::prost::alloc::vec::Vec<AppToDenomsMapping>,
    #[prost(message, optional, tag = "6")]
    pub params: ::core::option::Option<Params>,
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
pub struct QueryCollectorLookupByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCollectorLookupByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub collector_lookup: ::prost::alloc::vec::Vec<CollectorLookupTableData>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCollectorLookupByAppAndAssetRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCollectorLookupByAppAndAssetResponse {
    #[prost(message, optional, tag = "1")]
    pub collector_lookup: ::core::option::Option<CollectorLookupTableData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCollectorDataByAppAndAssetRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCollectorDataByAppAndAssetResponse {
    #[prost(message, optional, tag = "1")]
    pub collector_data: ::core::option::Option<CollectorData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionMappingForAppAndAssetRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionMappingForAppAndAssetResponse {
    #[prost(message, optional, tag = "1")]
    pub asset_id_to_auction_lookup_table: ::core::option::Option<AppAssetIdToAuctionLookupTable>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNetFeeCollectedForAppAndAssetRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNetFeeCollectedForAppAndAssetResponse {
    #[prost(message, optional, tag = "1")]
    pub asset_id_to_fee_collected: ::core::option::Option<AppAssetIdToFeeCollectedData>,
}
include!("comdex.collector.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
