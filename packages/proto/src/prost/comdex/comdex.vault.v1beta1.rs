// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateVault {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collateral_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDepositCollateral {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collateral_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub collateral: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventWithdrawCollateral {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collateral_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub collateral: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDrawDebt {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collateral_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub debt: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRepayDebt {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collateral_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub debt: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCloseVault {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collateral_type: ::prost::alloc::string::String,
}
/// app_vault_type_id will be the key for  the KVStore for this value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vault {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub amount_out: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "8")]
    pub interest_accumulated: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub closing_fee_accumulated: ::prost::alloc::string::String,
    #[prost(int64, tag = "10")]
    pub block_height: i64,
    #[prost(message, optional, tag = "11")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnerAppExtendedPairVaultMappingData {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_id: u64,
    #[prost(uint64, tag = "4")]
    pub vault_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppExtendedPairVaultMappingData {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub extended_pair_id: u64,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub vault_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag = "4")]
    pub token_minted_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub collateral_locked_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TvlLockedDataMap {
    #[prost(string, tag = "1")]
    pub asset_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collateral_locked_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintedDataMap {
    #[prost(string, tag = "1")]
    pub asset_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub minted_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StableMintVault {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount_out: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub app_id: u64,
    #[prost(uint64, tag = "5")]
    pub extended_pair_vault_id: u64,
    #[prost(message, optional, tag = "6")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairStatisticData {
    #[prost(string, tag = "1")]
    pub asset_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub asset_out_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub collateral_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub minted_amount: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub extended_pair_vault_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StableMintVaultRewards {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub stable_extended_pair_id: u64,
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub block_height: u64,
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub vaults: ::prost::alloc::vec::Vec<Vault>,
    #[prost(message, repeated, tag = "2")]
    pub stable_mint_vault: ::prost::alloc::vec::Vec<StableMintVault>,
    #[prost(message, repeated, tag = "3")]
    pub app_extended_pair_vault_mapping: ::prost::alloc::vec::Vec<AppExtendedPairVaultMappingData>,
    #[prost(message, repeated, tag = "4")]
    pub user_vault_asset_mapping: ::prost::alloc::vec::Vec<OwnerAppExtendedPairVaultMappingData>,
    #[prost(uint64, tag = "5")]
    pub length_of_vaults: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultInfo {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub extended_pair_id: u64,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub collateral: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub debt: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub collateralization_ratio: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub extended_pair_name: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub interest_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub asset_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub asset_out_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub min_cr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultResponse {
    #[prost(message, optional, tag = "1")]
    pub vault: ::core::option::Option<Vault>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultInfoByVaultIdRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultInfoByVaultIdResponse {
    #[prost(message, optional, tag = "1")]
    pub vaults_info: ::core::option::Option<VaultInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultInfoOfOwnerByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultInfoOfOwnerByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub vaults_info: ::prost::alloc::vec::Vec<VaultInfo>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllVaultsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllVaultsResponse {
    #[prost(message, repeated, tag = "1")]
    pub vault: ::prost::alloc::vec::Vec<Vault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllVaultsByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllVaultsByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub vault: ::prost::alloc::vec::Vec<Vault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllVaultsByAppAndExtendedPairRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub extended_pair_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllVaultsByAppAndExtendedPairResponse {
    #[prost(message, repeated, tag = "1")]
    pub vault: ::prost::alloc::vec::Vec<Vault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultIdOfOwnerByExtendedPairAndAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub extended_pair_id: u64,
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultIdOfOwnerByExtendedPairAndAppResponse {
    #[prost(uint64, tag = "1")]
    pub vault_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultIdsByAppInAllExtendedPairsRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultIdsByAppInAllExtendedPairsResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub vault_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllVaultIdsByAnOwnerRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllVaultIdsByAnOwnerResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub vault_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenMintedByAppAndExtendedPairRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub extended_pair_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenMintedByAppAndExtendedPairResponse {
    #[prost(string, tag = "1")]
    pub token_minted: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenMintedAssetWiseByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenMintedAssetWiseByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub minted_data: ::prost::alloc::vec::Vec<MintedDataMap>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultCountByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultCountByAppResponse {
    #[prost(uint64, tag = "1")]
    pub vault_count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultCountByAppAndExtendedPairRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub extended_pair_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVaultCountByAppAndExtendedPairResponse {
    #[prost(uint64, tag = "1")]
    pub vault_count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalValueLockedByAppAndExtendedPairRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub extended_pair_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalValueLockedByAppAndExtendedPairResponse {
    #[prost(string, tag = "1")]
    pub value_locked: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtendedPairIDsByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtendedPairIDsByAppResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub extended_pair_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStableVaultByVaultIdRequest {
    #[prost(uint64, tag = "1")]
    pub stable_vault_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStableVaultByVaultIdResponse {
    #[prost(message, optional, tag = "1")]
    pub stable_mint_vault: ::core::option::Option<StableMintVault>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStableVaultByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStableVaultByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub stable_mint_vault: ::prost::alloc::vec::Vec<StableMintVault>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStableVaultByAppAndExtendedPairRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub extended_pair_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStableVaultByAppAndExtendedPairResponse {
    #[prost(message, optional, tag = "1")]
    pub stable_mint_vault: ::core::option::Option<StableMintVault>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtendedPairVaultMappingByAppAndExtendedPairRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub extended_pair_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtendedPairVaultMappingByAppAndExtendedPairResponse {
    #[prost(message, optional, tag = "1")]
    pub extended_pair_vault_mapping: ::core::option::Option<AppExtendedPairVaultMappingData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtendedPairVaultMappingByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExtendedPairVaultMappingByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub extended_pair_vault_mapping: ::prost::alloc::vec::Vec<AppExtendedPairVaultMappingData>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTvlByAppOfAllExtendedPairsRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTvlByAppOfAllExtendedPairsResponse {
    #[prost(message, repeated, tag = "1")]
    pub tvldata: ::prost::alloc::vec::Vec<TvlLockedDataMap>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTvlByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTvlByAppResponse {
    #[prost(string, tag = "1")]
    pub collateral_locked: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserMyPositionByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserMyPositionByAppResponse {
    #[prost(string, tag = "1")]
    pub collateral_locked: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub total_due: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub available_to_borrow: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub average_cr_ratio: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserExtendedPairTotalDataRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserExtendedPairTotalDataResponse {
    #[prost(message, repeated, tag = "1")]
    pub user_total_data: ::prost::alloc::vec::Vec<OwnerAppExtendedPairVaultMappingData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPairsLockedAndMintedStatisticByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPairsLockedAndMintedStatisticByAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub pair_statistic_data: ::prost::alloc::vec::Vec<PairStatisticData>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllStableMintVaultRewardsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllStableMintVaultRewardsResponse {
    #[prost(message, repeated, tag = "1")]
    pub stable_mint_vault_rewards: ::prost::alloc::vec::Vec<StableMintVaultRewards>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(string, tag = "4")]
    pub amount_in: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub amount_out: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(uint64, tag = "4")]
    pub user_vault_id: u64,
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(uint64, tag = "4")]
    pub user_vault_id: u64,
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDrawRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(uint64, tag = "4")]
    pub user_vault_id: u64,
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDrawResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRepayRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(uint64, tag = "4")]
    pub user_vault_id: u64,
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRepayResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCloseRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(uint64, tag = "4")]
    pub user_vault_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCloseResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositAndDrawRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(uint64, tag = "4")]
    pub user_vault_id: u64,
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositAndDrawResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateStableMintRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateStableMintResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositStableMintRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub stable_vault_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositStableMintResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawStableMintRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub extended_pair_vault_id: u64,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub stable_vault_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawStableMintResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgVaultInterestCalcRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub user_vault_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgVaultInterestCalcResponse {}
include!("comdex.vault.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
