// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppData {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub short_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub min_gov_deposit: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub gov_time_in_seconds: u64,
    #[prost(message, repeated, tag = "6")]
    pub genesis_token: ::prost::alloc::vec::Vec<MintGenesisToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintGenesisToken {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(string, tag = "2")]
    pub genesis_supply: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_gov_token: bool,
    #[prost(string, tag = "4")]
    pub recipient: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppAndGovTime {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub gov_time_in_seconds: u64,
    #[prost(string, tag = "3")]
    pub min_gov_deposit: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub decimals: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub is_on_chain: bool,
    #[prost(bool, tag = "6")]
    pub is_oracle_price_required: bool,
    #[prost(bool, tag = "7")]
    pub is_cdp_mintable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAddPair {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedPairVault {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub pair_id: u64,
    #[prost(string, tag = "4")]
    pub stability_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub closing_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub liquidation_penalty: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub draw_down_fee: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub is_vault_active: bool,
    #[prost(string, tag = "9")]
    pub debt_ceiling: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub debt_floor: ::prost::alloc::string::String,
    #[prost(bool, tag = "11")]
    pub is_stable_mint_vault: bool,
    #[prost(string, tag = "12")]
    pub min_cr: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub pair_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "14")]
    pub asset_out_oracle_price: bool,
    #[prost(uint64, tag = "15")]
    pub asset_out_price: u64,
    #[prost(uint64, tag = "16")]
    pub min_usd_value_left: u64,
    #[prost(int64, tag = "17")]
    pub block_height: i64,
    #[prost(message, optional, tag = "18")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_in: u64,
    #[prost(uint64, tag = "3")]
    pub asset_out: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairInfo {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_in: u64,
    #[prost(string, tag = "3")]
    pub denom_in: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub asset_out: u64,
    #[prost(string, tag = "5")]
    pub denom_out: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetPair {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub decimals: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub is_on_chain: bool,
    #[prost(bool, tag = "6")]
    pub is_oracle_price_required: bool,
    #[prost(bool, tag = "7")]
    pub is_cdp_mintable: bool,
    #[prost(uint64, tag = "8")]
    pub asset_out: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
    #[prost(message, repeated, tag = "3")]
    pub app_data: ::prost::alloc::vec::Vec<AppData>,
    #[prost(message, repeated, tag = "4")]
    pub extended_pair_vault: ::prost::alloc::vec::Vec<ExtendedPairVault>,
    #[prost(message, optional, tag = "5")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAssetsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<Asset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMultipleAssetsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMultiplePairsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssetProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub asset: ::core::option::Option<Asset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPairsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pairs: ::core::option::Option<Pair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePairProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pairs: ::core::option::Option<Pair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAppProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub app: ::core::option::Option<AppData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGovTimeInAppProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub gov_time: ::core::option::Option<AppAndGovTime>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAssetInAppProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub app: ::core::option::Option<AppData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMultipleAssetsPairsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub assets_pair: ::prost::alloc::vec::Vec<AssetPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetsResponse {
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetResponse {
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<Asset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetPairsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetPairsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pairs_info: ::prost::alloc::vec::Vec<PairInfo>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetPairRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetPairResponse {
    #[prost(message, optional, tag = "1")]
    pub pair_info: ::core::option::Option<PairInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppResponse {
    #[prost(message, optional, tag = "1")]
    pub app: ::core::option::Option<AppData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGovTokenByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGovTokenByAppResponse {
    #[prost(uint64, tag = "1")]
    pub gov_asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppsResponse {
    #[prost(message, repeated, tag = "1")]
    pub apps: ::prost::alloc::vec::Vec<AppData>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtendedPairVaultRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtendedPairVaultResponse {
    #[prost(message, optional, tag = "1")]
    pub pair_vault: ::core::option::Option<ExtendedPairVault>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllExtendedPairVaultsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllExtendedPairVaultsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pair_vault: ::prost::alloc::vec::Vec<ExtendedPairVault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllExtendedPairVaultsByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllExtendedPairVaultsByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub extended_pair: ::prost::alloc::vec::Vec<ExtendedPairVault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllExtendedPairStableVaultsIdByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllExtendedPairStableVaultsIdByAppResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub extended_pairs_id: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllExtendedPairStableVaultsByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllExtendedPairStableVaultsByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub extended_pair: ::prost::alloc::vec::Vec<ExtendedPairVault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtendedPairVaultsByAppWithoutStableRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtendedPairVaultsByAppWithoutStableResponse {
    #[prost(message, repeated, tag = "1")]
    pub extended_pair: ::prost::alloc::vec::Vec<ExtendedPairVault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
include!("comdex.asset.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
