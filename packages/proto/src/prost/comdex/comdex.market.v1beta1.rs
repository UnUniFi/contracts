// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWeightedAverage {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(uint64, tag = "2")]
    pub script_id: u64,
    #[prost(uint64, tag = "3")]
    pub twa: u64,
    #[prost(uint64, tag = "4")]
    pub current_index: u64,
    #[prost(bool, tag = "5")]
    pub is_price_active: bool,
    #[prost(uint64, repeated, packed = "false", tag = "6")]
    pub price_value: ::prost::alloc::vec::Vec<u64>,
    #[prost(int64, tag = "7")]
    pub discarded_height_diff: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub time_weighted_average: ::prost::alloc::vec::Vec<TimeWeightedAverage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketsResponse {
    #[prost(message, repeated, tag = "1")]
    pub time_weighted_average: ::prost::alloc::vec::Vec<TimeWeightedAverage>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketRequest {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketResponse {
    #[prost(message, optional, tag = "1")]
    pub time_weighted_average: ::core::option::Option<TimeWeightedAverage>,
}
include!("comdex.market.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
