// @generated
/// locker_id will be the key which will be derived from the LockerLookUpTable
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Locker {
    #[prost(uint64, tag = "1")]
    pub locker_id: u64,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub returns_accumulated: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub net_balance: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "6")]
    pub asset_deposit_id: u64,
    #[prost(bool, tag = "7")]
    pub is_locked: bool,
    #[prost(uint64, tag = "8")]
    pub app_id: u64,
    #[prost(int64, tag = "9")]
    pub block_height: i64,
    #[prost(message, optional, tag = "10")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
}
// Key is user address

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAppAssetLockerMapping {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub asset_id: u64,
    #[prost(uint64, tag = "4")]
    pub locker_id: u64,
    #[prost(message, repeated, tag = "5")]
    pub user_data: ::prost::alloc::vec::Vec<UserTxData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserTxData {
    #[prost(string, tag = "1")]
    pub tx_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub balance: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub tx_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockerLookupTableData {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub locker_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag = "4")]
    pub deposited_amount: ::prost::alloc::string::String,
}
/// Key is app_mapping_id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockerProductAssetMapping {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedDepositedAmountDataMap {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(string, tag = "2")]
    pub deposited_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockerTotalRewardsByAssetAppWise {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(string, tag = "3")]
    pub total_rewards: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub lockers: ::prost::alloc::vec::Vec<Locker>,
    #[prost(message, repeated, tag = "2")]
    pub locker_product_asset_mapping: ::prost::alloc::vec::Vec<LockerProductAssetMapping>,
    #[prost(message, repeated, tag = "3")]
    pub locker_total_rewards_by_asset_app_wise:
        ::prost::alloc::vec::Vec<LockerTotalRewardsByAssetAppWise>,
    #[prost(message, repeated, tag = "4")]
    pub locker_lookup_table: ::prost::alloc::vec::Vec<LockerLookupTableData>,
    #[prost(message, repeated, tag = "5")]
    pub user_locker_asset_mapping: ::prost::alloc::vec::Vec<UserAppAssetLockerMapping>,
    #[prost(message, optional, tag = "6")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerInfoRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub locker_info: ::core::option::Option<Locker>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockersByAppToAssetIdRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockersByAppToAssetIdResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub locker_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerInfoByAppIdRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerInfoByAppIdResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub locker_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalDepositByAppAndAssetIdRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalDepositByAppAndAssetIdResponse {
    #[prost(uint64, tag = "1")]
    pub total_deposit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerLockerByAppIDbyOwnerRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerLockerByAppIDbyOwnerResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub locker_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerLockerOfAllAppsByOwnerRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerLockerOfAllAppsByOwnerResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub locker_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerTxDetailsLockerOfAppByOwnerByAssetRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub asset_id: u64,
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerTxDetailsLockerOfAppByOwnerByAssetResponse {
    #[prost(message, repeated, tag = "1")]
    pub user_tx_data: ::prost::alloc::vec::Vec<UserTxData>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerLockerByAppToAssetIDbyOwnerRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerLockerByAppToAssetIDbyOwnerResponse {
    #[prost(message, optional, tag = "1")]
    pub locker_info: ::core::option::Option<Locker>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerByAppByOwnerRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerByAppByOwnerResponse {
    #[prost(message, repeated, tag = "1")]
    pub locker_info: ::prost::alloc::vec::Vec<Locker>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerCountByAppIdRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerCountByAppIdResponse {
    #[prost(uint64, tag = "1")]
    pub total_count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerCountByAppToAssetIdRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerCountByAppToAssetIdResponse {
    #[prost(uint64, tag = "1")]
    pub total_count: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWhiteListedAssetIDsByAppIdRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWhiteListedAssetIDsByAppIdResponse {
    #[prost(uint64, repeated, tag = "1")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWhiteListedAssetByAllAppsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWhiteListedAssetByAllAppsResponse {
    #[prost(message, repeated, tag = "1")]
    pub product_to_all_asset: ::prost::alloc::vec::Vec<AppToAllAsset>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppToAllAsset {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub assets: ::prost::alloc::vec::Vec<super::super::asset::v1beta1::Asset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerLookupTableByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerLookupTableByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub token_to_locker_mapping: ::prost::alloc::vec::Vec<LockerLookupTableData>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerLookupTableByAppAndAssetIdRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerLookupTableByAppAndAssetIdResponse {
    #[prost(message, optional, tag = "1")]
    pub token_to_locker_mapping: ::core::option::Option<LockerLookupTableData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerTotalDepositedByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerTotalDepositedByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub locked_deposited_amount_data_map: ::prost::alloc::vec::Vec<LockedDepositedAmountDataMap>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerTotalRewardsByAssetAppWiseRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockerTotalRewardsByAssetAppWiseResponse {
    #[prost(message, optional, tag = "1")]
    pub total_rewards: ::core::option::Option<LockerTotalRewardsByAssetAppWise>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateLockerRequest {
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub asset_id: u64,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateLockerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddWhiteListedAssetRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddWhiteListedAssetResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositAssetRequest {
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub locker_id: u64,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub asset_id: u64,
    #[prost(uint64, tag = "5")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositAssetResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawAssetRequest {
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub locker_id: u64,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub asset_id: u64,
    #[prost(uint64, tag = "5")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawAssetResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCloseLockerRequest {
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub asset_id: u64,
    #[prost(uint64, tag = "4")]
    pub locker_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCloseLockerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockerRewardCalcRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub locker_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockerRewardCalcResponse {}
include!("comdex.locker.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
