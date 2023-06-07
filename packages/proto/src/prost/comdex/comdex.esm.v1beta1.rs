// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EsmTriggerParams {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub target_value: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "3")]
    pub cool_off_period: u64,
    #[prost(message, repeated, tag = "4")]
    pub assets_rates: ::prost::alloc::vec::Vec<DebtAssetsRates>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentDepositStats {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EsmStatus {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub executor: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub status: bool,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "6")]
    pub vault_redemption_status: bool,
    #[prost(bool, tag = "7")]
    pub snapshot_status: bool,
    #[prost(bool, tag = "8")]
    pub stable_vault_redemption_status: bool,
    #[prost(bool, tag = "9")]
    pub collector_transaction: bool,
    #[prost(bool, tag = "10")]
    pub share_calculation: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KillSwitchParams {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(bool, tag = "2")]
    pub breaker_enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsersDepositMapping {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub deposits: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataAfterCoolOff {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub collateral_total_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub debt_total_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetToAmount {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub share: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub debt_token_worth: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub is_collateral: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebtAssetsRates {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(uint64, tag = "2")]
    pub rates: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, repeated, tag = "1")]
    pub admin: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub e_sm_trigger_params: ::prost::alloc::vec::Vec<EsmTriggerParams>,
    #[prost(message, repeated, tag = "2")]
    pub current_deposit_stats: ::prost::alloc::vec::Vec<CurrentDepositStats>,
    #[prost(message, repeated, tag = "3")]
    pub e_sm_status: ::prost::alloc::vec::Vec<EsmStatus>,
    #[prost(message, repeated, tag = "4")]
    pub kill_switch_params: ::prost::alloc::vec::Vec<KillSwitchParams>,
    #[prost(message, repeated, tag = "5")]
    pub users_deposit_mapping: ::prost::alloc::vec::Vec<UsersDepositMapping>,
    #[prost(message, repeated, tag = "7")]
    pub data_after_cool_off: ::prost::alloc::vec::Vec<DataAfterCoolOff>,
    #[prost(message, optional, tag = "10")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEsmTriggerParamsRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEsmTriggerParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub esm_trigger_params: ::core::option::Option<EsmTriggerParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEsmStatusRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEsmStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub esm_status: ::core::option::Option<EsmStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentDepositStatsRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentDepositStatsResponse {
    #[prost(message, optional, tag = "1")]
    pub current_deposit_stats: ::core::option::Option<CurrentDepositStats>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUsersDepositMappingRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUsersDepositMappingResponse {
    #[prost(message, optional, tag = "1")]
    pub users_deposit_mapping: ::core::option::Option<UsersDepositMapping>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDataAfterCoolOffRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDataAfterCoolOffResponse {
    #[prost(message, optional, tag = "1")]
    pub data_after_cool_off: ::core::option::Option<DataAfterCoolOff>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySnapshotPriceRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySnapshotPriceResponse {
    #[prost(uint64, tag = "1")]
    pub price: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetDataAfterCoolOffRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetDataAfterCoolOffResponse {
    #[prost(message, repeated, tag = "1")]
    pub asset_to_amount: ::prost::alloc::vec::Vec<AssetToAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositEsm {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteEsm {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgKillRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub kill_switch_params: ::core::option::Option<KillSwitchParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCollateralRedemptionRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositEsmResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteEsmResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgKillResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCollateralRedemptionResponse {}
include!("comdex.esm.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
