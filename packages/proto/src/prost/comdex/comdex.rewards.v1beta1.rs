// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochInfo {
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(int64, tag = "4")]
    pub current_epoch: i64,
    #[prost(message, optional, tag = "5")]
    pub current_epoch_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag = "6")]
    pub current_epoch_start_height: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidtyGaugeMetaData {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(bool, tag = "2")]
    pub is_master_pool: bool,
    #[prost(uint64, repeated, tag = "3")]
    pub child_pool_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gauge {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "5")]
    pub gauge_type_id: u64,
    #[prost(message, optional, tag = "6")]
    pub trigger_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "7")]
    pub deposit_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "8")]
    pub total_triggers: u64,
    #[prost(uint64, tag = "9")]
    pub triggered_count: u64,
    #[prost(message, optional, tag = "10")]
    pub distributed_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(bool, tag = "11")]
    pub is_active: bool,
    #[prost(bool, tag = "12")]
    pub for_swap_fee: bool,
    #[prost(uint64, tag = "14")]
    pub app_id: u64,
    #[prost(oneof = "gauge::Kind", tags = "13")]
    pub kind: ::core::option::Option<gauge::Kind>,
}
/// Nested message and enum types in `Gauge`.
pub mod gauge {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "13")]
        LiquidityMetaData(super::LiquidtyGaugeMetaData),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByTriggerDuration {
    #[prost(message, optional, tag = "1")]
    pub trigger_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint64, repeated, tag = "2")]
    pub gauge_ids: ::prost::alloc::vec::Vec<u64>,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalRewards {
    #[prost(uint64, tag = "1")]
    pub app_mapping_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockerRewardsTracker {
    #[prost(uint64, tag = "1")]
    pub locker_id: u64,
    #[prost(uint64, tag = "2")]
    pub app_mapping_id: u64,
    #[prost(string, tag = "3")]
    pub rewards_accumulated: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultInterestTracker {
    #[prost(uint64, tag = "1")]
    pub vault_id: u64,
    #[prost(uint64, tag = "2")]
    pub app_mapping_id: u64,
    #[prost(string, tag = "3")]
    pub interest_accumulated: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockerExternalRewards {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub app_mapping_id: u64,
    #[prost(uint64, tag = "3")]
    pub asset_id: u64,
    #[prost(message, optional, tag = "4")]
    pub total_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "5")]
    pub duration_days: i64,
    #[prost(bool, tag = "6")]
    pub is_active: bool,
    #[prost(message, optional, tag = "7")]
    pub available_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "8")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub start_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "10")]
    pub end_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag = "11")]
    pub min_lockup_time_seconds: i64,
    #[prost(uint64, tag = "12")]
    pub epoch_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultExternalRewards {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub app_mapping_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_id: u64,
    #[prost(message, optional, tag = "4")]
    pub total_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "5")]
    pub duration_days: i64,
    #[prost(bool, tag = "6")]
    pub is_active: bool,
    #[prost(message, optional, tag = "7")]
    pub available_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "8")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub start_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "10")]
    pub end_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag = "11")]
    pub min_lockup_time_seconds: i64,
    #[prost(uint64, tag = "12")]
    pub epoch_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochTime {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub app_mapping_id: u64,
    #[prost(int64, tag = "3")]
    pub starting_time: i64,
    #[prost(uint64, tag = "4")]
    pub count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendExternalRewards {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub app_mapping_id: u64,
    #[prost(message, optional, tag = "3")]
    pub rewards_asset_pool_data: ::core::option::Option<RewardsAssetPoolData>,
    #[prost(message, optional, tag = "4")]
    pub total_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "5")]
    pub master_pool_id: i64,
    #[prost(int64, tag = "6")]
    pub duration_days: i64,
    #[prost(bool, tag = "7")]
    pub is_active: bool,
    #[prost(message, optional, tag = "8")]
    pub available_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "9")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "10")]
    pub start_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "11")]
    pub end_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag = "12")]
    pub min_lockup_time_seconds: i64,
    #[prost(uint64, tag = "13")]
    pub epoch_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsAssetPoolData {
    #[prost(uint64, tag = "1")]
    pub c_pool_id: u64,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub asset_id: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag = "3")]
    pub c_swap_app_id: u64,
    #[prost(uint64, tag = "4")]
    pub c_swap_min_lock_amount: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StableVaultExternalRewards {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub cswap_app_id: u64,
    #[prost(uint64, tag = "4")]
    pub commodo_app_id: u64,
    #[prost(message, optional, tag = "5")]
    pub total_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "6")]
    pub duration_days: i64,
    #[prost(bool, tag = "7")]
    pub is_active: bool,
    #[prost(message, optional, tag = "8")]
    pub available_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "9")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "10")]
    pub start_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "11")]
    pub end_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag = "12")]
    pub accepted_block_height: i64,
    #[prost(uint64, tag = "13")]
    pub epoch_id: u64,
}
/// GenesisState defines the rewards module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub internal_rewards: ::prost::alloc::vec::Vec<InternalRewards>,
    #[prost(message, repeated, tag = "2")]
    pub locker_rewards_tracker: ::prost::alloc::vec::Vec<LockerRewardsTracker>,
    #[prost(message, repeated, tag = "3")]
    pub vault_interest_tracker: ::prost::alloc::vec::Vec<VaultInterestTracker>,
    #[prost(message, repeated, tag = "4")]
    pub locker_external_rewards: ::prost::alloc::vec::Vec<LockerExternalRewards>,
    #[prost(message, repeated, tag = "5")]
    pub vault_external_rewards: ::prost::alloc::vec::Vec<VaultExternalRewards>,
    #[prost(uint64, repeated, packed = "false", tag = "6")]
    pub app_i_ds: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag = "7")]
    pub epoch_info: ::prost::alloc::vec::Vec<EpochInfo>,
    #[prost(message, repeated, tag = "8")]
    pub gauge: ::prost::alloc::vec::Vec<Gauge>,
    #[prost(message, repeated, tag = "9")]
    pub gauge_by_trigger_duration: ::prost::alloc::vec::Vec<GaugeByTriggerDuration>,
    #[prost(message, optional, tag = "10")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "11")]
    pub lend_external_rewards: ::prost::alloc::vec::Vec<LendExternalRewards>,
}
/// PARAMS
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// ALL EPOCHES
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEpochsInfoRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEpochsInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// EPOCH BY DURATION
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochInfoByDurationRequest {
    #[prost(uint64, tag = "1")]
    pub duration_seconds: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochInfoByDurationResponse {
    #[prost(message, optional, tag = "1")]
    pub epoch: ::core::option::Option<EpochInfo>,
}
/// ALL GAUGES
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllGaugesRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// GAUGE BY ID
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGaugeByIdRequest {
    #[prost(uint64, tag = "1")]
    pub gauge_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGaugeByIdResponse {
    #[prost(message, optional, tag = "1")]
    pub gauge: ::core::option::Option<Gauge>,
}
/// GAUGE BY TRIIGER DURATION
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGaugesByDurationRequest {
    #[prost(uint64, tag = "1")]
    pub duration_seconds: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGaugeByDurationResponse {
    #[prost(message, repeated, tag = "1")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsResponse {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<InternalRewards>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardResponse {
    #[prost(message, repeated, tag = "1")]
    pub reward: ::prost::alloc::vec::Vec<InternalRewards>,
}
// all ExternalRewardsLockers

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalRewardsLockersRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalRewardsLockersResponse {
    #[prost(message, repeated, tag = "1")]
    pub locker_external_rewards: ::prost::alloc::vec::Vec<LockerExternalRewards>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalRewardVaultsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalRewardVaultsResponse {
    #[prost(message, repeated, tag = "1")]
    pub vault_external_rewards: ::prost::alloc::vec::Vec<VaultExternalRewards>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWhitelistedAppIdsVaultRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWhitelistedAppIdsVaultResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub whitelisted_app_ids_vault: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalRewardLendsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalRewardLendsResponse {
    #[prost(message, repeated, tag = "1")]
    pub lend_external_rewards: ::prost::alloc::vec::Vec<LendExternalRewards>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalRewardStableMintRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalRewardStableMintResponse {
    #[prost(message, repeated, tag = "1")]
    pub stable_mint_external_rewards: ::prost::alloc::vec::Vec<StableVaultExternalRewards>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochTimeRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochTimeResponse {
    #[prost(message, repeated, tag = "1")]
    pub epoch_time: ::prost::alloc::vec::Vec<EpochTime>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtLendRewardsAprRequest {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(uint64, tag = "2")]
    pub c_pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtLendRewardsAprResponse {
    #[prost(string, tag = "1")]
    pub apr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGauge {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub gauge_type_id: u64,
    #[prost(message, optional, tag = "3")]
    pub trigger_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "4")]
    pub deposit_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "5")]
    pub total_triggers: u64,
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "8")]
    pub app_id: u64,
    #[prost(oneof = "msg_create_gauge::Kind", tags = "7")]
    pub kind: ::core::option::Option<msg_create_gauge::Kind>,
}
/// Nested message and enum types in `MsgCreateGauge`.
pub mod msg_create_gauge {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "7")]
        LiquidityMetaData(super::LiquidtyGaugeMetaData),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGaugeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhitelistAsset {
    #[prost(uint64, tag = "1")]
    pub app_mapping_id: u64,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWhitelistAsset {
    #[prost(uint64, tag = "1")]
    pub app_mapping_id: u64,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWhitelistAssetResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveWhitelistAssetResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhitelistAppIdVault {
    #[prost(uint64, tag = "1")]
    pub app_mapping_id: u64,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWhitelistAppIdVault {
    #[prost(uint64, tag = "1")]
    pub app_mapping_id: u64,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWhitelistAppIdVaultResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveWhitelistAppIdVaultResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateExternalRewardsLockers {
    #[prost(uint64, tag = "1")]
    pub app_mapping_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(message, optional, tag = "3")]
    pub total_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "4")]
    pub duration_days: i64,
    #[prost(string, tag = "5")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub min_lockup_time_seconds: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateExternalRewardsLockersResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateExternalRewardsVault {
    #[prost(uint64, tag = "1")]
    pub app_mapping_id: u64,
    #[prost(uint64, tag = "2")]
    pub extended_pair_id: u64,
    #[prost(message, optional, tag = "3")]
    pub total_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "4")]
    pub duration_days: i64,
    #[prost(string, tag = "5")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub min_lockup_time_seconds: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateExternalRewardsVaultResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateExternalRewardsLend {
    #[prost(uint64, tag = "1")]
    pub app_mapping_id: u64,
    #[prost(uint64, tag = "2")]
    pub c_pool_id: u64,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub asset_id: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag = "4")]
    pub c_swap_app_id: u64,
    #[prost(uint64, tag = "5")]
    pub c_swap_min_lock_amount: u64,
    #[prost(message, optional, tag = "6")]
    pub total_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "7")]
    pub master_pool_id: i64,
    #[prost(int64, tag = "8")]
    pub duration_days: i64,
    #[prost(int64, tag = "9")]
    pub min_lockup_time_seconds: i64,
    #[prost(string, tag = "10")]
    pub depositor: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateExternalRewardsLendResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateExternalRewardsStableMint {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub cswap_app_id: u64,
    #[prost(uint64, tag = "3")]
    pub commodo_app_id: u64,
    #[prost(message, optional, tag = "4")]
    pub total_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "5")]
    pub duration_days: i64,
    #[prost(string, tag = "6")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub accepted_block_height: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateExternalRewardsStableMintResponse {}
include!("comdex.rewards.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
