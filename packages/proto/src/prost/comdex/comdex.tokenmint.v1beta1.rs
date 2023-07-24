// @generated
/// app_vault_type_id will be the key for  the KVStore for this value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenMint {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub minted_tokens: ::prost::alloc::vec::Vec<MintedTokens>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintedTokens {
    #[prost(uint64, tag = "1")]
    pub asset_id: u64,
    #[prost(string, tag = "2")]
    pub genesis_supply: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub current_supply: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub token_mint: ::prost::alloc::vec::Vec<TokenMint>,
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllTokenMintedForAllAppsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllTokenMintedForAllAppsResponse {
    #[prost(message, repeated, tag = "1")]
    pub token_mint: ::prost::alloc::vec::Vec<TokenMint>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenMintedByAppRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenMintedByAppResponse {
    #[prost(message, optional, tag = "1")]
    pub token_mint: ::core::option::Option<TokenMint>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenMintedByAppAndAssetRequest {
    #[prost(uint64, tag = "1")]
    pub app_id: u64,
    #[prost(uint64, tag = "2")]
    pub asset_id: u64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenMintedByAppAndAssetResponse {
    #[prost(message, optional, tag = "1")]
    pub minted_tokens: ::core::option::Option<MintedTokens>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// Will become governance proposal- will trigger token minting & sending
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintNewTokensRequest {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub app_id: u64,
    #[prost(uint64, tag = "3")]
    pub asset_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintNewTokensResponse {}
include!("comdex.tokenmint.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
