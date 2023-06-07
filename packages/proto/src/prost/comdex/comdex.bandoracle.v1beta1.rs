// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchPriceCallData {
    #[prost(string, repeated, tag = "1")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "2")]
    pub multiplier: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchPriceResult {
    #[prost(uint64, repeated, tag = "1")]
    pub rates: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Market {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub script_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscardData {
    #[prost(int64, tag = "1")]
    pub block_height: i64,
    #[prost(bool, tag = "2")]
    pub discard_bool: bool,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub flag: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFetchPriceData {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub oracle_script_id: u64,
    #[prost(string, tag = "3")]
    pub source_channel: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub calldata: ::core::option::Option<FetchPriceCallData>,
    #[prost(uint64, tag = "5")]
    pub ask_count: u64,
    #[prost(uint64, tag = "6")]
    pub min_count: u64,
    #[prost(message, repeated, tag = "7")]
    pub fee_limit: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "8")]
    pub request_key: ::prost::alloc::string::String,
    #[prost(uint64, tag = "9")]
    pub prepare_gas: u64,
    #[prost(uint64, tag = "10")]
    pub execute_gas: u64,
    #[prost(string, tag = "11")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "12")]
    pub twa_batch_size: u64,
    #[prost(int64, tag = "13")]
    pub accepted_height_diff: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFetchPriceDataResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchPriceProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub fetch_price: ::core::option::Option<MsgFetchPriceData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandoraclePacketData {
    #[prost(oneof = "bandoracle_packet_data::Packet", tags = "1")]
    pub packet: ::core::option::Option<bandoracle_packet_data::Packet>,
}
/// Nested message and enum types in `BandoraclePacketData`.
pub mod bandoracle_packet_data {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Packet {
        #[prost(message, tag = "1")]
        NoData(super::NoData),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoData {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFetchPriceRequest {
    #[prost(int64, tag = "1")]
    pub request_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFetchPriceResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<FetchPriceResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastFetchPriceIdRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastFetchPriceIdResponse {
    #[prost(int64, tag = "1")]
    pub request_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFetchPriceDataRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFetchPriceDataResponse {
    #[prost(message, optional, tag = "1")]
    pub msg_fetch_price_data: ::core::option::Option<MsgFetchPriceData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDiscardDataRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDiscardDataResponse {
    #[prost(message, optional, tag = "1")]
    pub discard_data: ::core::option::Option<DiscardData>,
}
include!("comdex.bandoracle.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
