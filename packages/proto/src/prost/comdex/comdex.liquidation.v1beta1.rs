// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub liquidation_batch_size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedVault {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub original_vault_id: u64,
    #[prost(uint64, tag = "4")]
    pub extended_pair_vault_id: u64,
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub amount_out: ::prost::alloc::string::String,
    /// updated_amount_out = amount_out + interest_accumulated + opening_fee_accumulated
    /// + closing_fee_accumulated
    #[prost(string, tag = "8")]
    pub updated_amount_out: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub initiator: ::prost::alloc::string::String,
    #[prost(bool, tag = "10")]
    pub is_auction_complete: bool,
    #[prost(bool, tag = "11")]
    pub is_auction_in_progress: bool,
    #[prost(string, tag = "12")]
    pub cr_at_liquidation: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub current_collateralisation_ratio: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub collateral_to_be_auctioned: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "15")]
    pub liquidation_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, repeated, tag = "16")]
    pub selloff_history: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "17")]
    pub interest_accumulated: ::prost::alloc::string::String,
    #[prost(oneof = "locked_vault::Kind", tags = "18")]
    pub kind: ::core::option::Option<locked_vault::Kind>,
}
/// Nested message and enum types in `LockedVault`.
pub mod locked_vault {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "18")]
        BorrowMetaData(super::BorrowMetaData),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowMetaData {
    #[prost(uint64, tag = "1")]
    pub lending_id: u64,
    #[prost(bool, tag = "2")]
    pub is_stable_borrow: bool,
    #[prost(string, tag = "3")]
    pub stable_borrow_rate: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub bridged_asset_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub locked_vault: ::prost::alloc::vec::Vec<LockedVault>,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub whitelisted_apps: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidationOffsetHolder {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub current_offset: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockedVaultRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockedVaultResponse {
    #[prost(message, optional, tag = "1")]
    pub locked_vault: ::core::option::Option<LockedVault>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockedVaultsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockedVaultsResponse {
    #[prost(message, repeated, tag = "1")]
    pub locked_vaults: ::prost::alloc::vec::Vec<LockedVault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockedVaultsHistoryRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockedVaultsHistoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub locked_vaults_history: ::prost::alloc::vec::Vec<LockedVault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserLockedVaultsRequest {
    #[prost(string, tag = "1")]
    pub user_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserLockedVaultsResponse {
    #[prost(message, repeated, tag = "1")]
    pub user_locked_vaults: ::prost::alloc::vec::Vec<LockedVault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserLockedVaultsHistoryRequest {
    #[prost(string, tag = "1")]
    pub user_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserLockedVaultsHistoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub user_locked_vaults_history: ::prost::alloc::vec::Vec<LockedVault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockedVaultsPairRequest {
    #[prost(uint64, tag = "1")]
    pub pair_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockedVaultsPairResponse {
    #[prost(message, repeated, tag = "1")]
    pub locked_vaults_pair: ::prost::alloc::vec::Vec<LockedVault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppIdsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppIdsResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub whitelisted_app_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidateVaultRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub vault_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidateVaultResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidateBorrowRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub borrow_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidateBorrowResponse {}
include!("comdex.liquidation.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
