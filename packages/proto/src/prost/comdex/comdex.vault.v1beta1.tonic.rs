// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    ///
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///
        pub async fn query_vault(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVaultRequest>,
        ) -> Result<tonic::Response<super::QueryVaultResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.vault.v1beta1.Query/QueryVault");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_user_extended_pair_total_data(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUserExtendedPairTotalDataRequest>,
        ) -> Result<tonic::Response<super::QueryUserExtendedPairTotalDataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryUserExtendedPairTotalData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_vault_info_by_vault_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVaultInfoByVaultIdRequest>,
        ) -> Result<tonic::Response<super::QueryVaultInfoByVaultIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryVaultInfoByVaultID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_vault_info_of_owner_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVaultInfoOfOwnerByAppRequest>,
        ) -> Result<tonic::Response<super::QueryVaultInfoOfOwnerByAppResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryVaultInfoOfOwnerByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_vaults(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllVaultsRequest>,
        ) -> Result<tonic::Response<super::QueryAllVaultsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.vault.v1beta1.Query/QueryAllVaults");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_vaults_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllVaultsByAppRequest>,
        ) -> Result<tonic::Response<super::QueryAllVaultsByAppResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryAllVaultsByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_vaults_by_app_and_extended_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllVaultsByAppAndExtendedPairRequest>,
        ) -> Result<tonic::Response<super::QueryAllVaultsByAppAndExtendedPairResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryAllVaultsByAppAndExtendedPair",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_vault_id_of_owner_by_extended_pair_and_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVaultIdOfOwnerByExtendedPairAndAppRequest>,
        ) -> Result<
            tonic::Response<super::QueryVaultIdOfOwnerByExtendedPairAndAppResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryVaultIDOfOwnerByExtendedPairAndApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_vault_ids_by_app_in_all_extended_pairs(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVaultIdsByAppInAllExtendedPairsRequest>,
        ) -> Result<
            tonic::Response<super::QueryVaultIdsByAppInAllExtendedPairsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryVaultIdsByAppInAllExtendedPairs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_vault_ids_by_an_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllVaultIdsByAnOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryAllVaultIdsByAnOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryAllVaultIdsByAnOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_token_minted_by_app_and_extended_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTokenMintedByAppAndExtendedPairRequest>,
        ) -> Result<
            tonic::Response<super::QueryTokenMintedByAppAndExtendedPairResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryTokenMintedByAppAndExtendedPair",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_token_minted_asset_wise_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTokenMintedAssetWiseByAppRequest>,
        ) -> Result<tonic::Response<super::QueryTokenMintedAssetWiseByAppResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryTokenMintedAssetWiseByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_vault_count_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVaultCountByAppRequest>,
        ) -> Result<tonic::Response<super::QueryVaultCountByAppResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryVaultCountByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_vault_count_by_app_and_extended_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVaultCountByAppAndExtendedPairRequest>,
        ) -> Result<
            tonic::Response<super::QueryVaultCountByAppAndExtendedPairResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryVaultCountByAppAndExtendedPair",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_total_value_locked_by_app_and_extended_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTotalValueLockedByAppAndExtendedPairRequest>,
        ) -> Result<
            tonic::Response<super::QueryTotalValueLockedByAppAndExtendedPairResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryTotalValueLockedByAppAndExtendedPair",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_extended_pair_i_ds_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExtendedPairIDsByAppRequest>,
        ) -> Result<tonic::Response<super::QueryExtendedPairIDsByAppResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryExtendedPairIDsByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_stable_vault_by_vault_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryStableVaultByVaultIdRequest>,
        ) -> Result<tonic::Response<super::QueryStableVaultByVaultIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryStableVaultByVaultID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_stable_vault_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryStableVaultByAppRequest>,
        ) -> Result<tonic::Response<super::QueryStableVaultByAppResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryStableVaultByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_stable_vault_by_app_and_extended_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryStableVaultByAppAndExtendedPairRequest>,
        ) -> Result<
            tonic::Response<super::QueryStableVaultByAppAndExtendedPairResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryStableVaultByAppAndExtendedPair",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_extended_pair_vault_mapping_by_app_and_extended_pair(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryExtendedPairVaultMappingByAppAndExtendedPairRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryExtendedPairVaultMappingByAppAndExtendedPairResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryExtendedPairVaultMappingByAppAndExtendedPair",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_extended_pair_vault_mapping_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExtendedPairVaultMappingByAppRequest>,
        ) -> Result<tonic::Response<super::QueryExtendedPairVaultMappingByAppResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryExtendedPairVaultMappingByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_tvl_by_app_of_all_extended_pairs(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTvlByAppOfAllExtendedPairsRequest>,
        ) -> Result<tonic::Response<super::QueryTvlByAppOfAllExtendedPairsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryTVLByAppOfAllExtendedPairs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_tvl_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTvlByAppRequest>,
        ) -> Result<tonic::Response<super::QueryTvlByAppResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.vault.v1beta1.Query/QueryTVLByApp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_user_my_position_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUserMyPositionByAppRequest>,
        ) -> Result<tonic::Response<super::QueryUserMyPositionByAppResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryUserMyPositionByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_pairs_locked_and_minted_statistic_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPairsLockedAndMintedStatisticByAppRequest>,
        ) -> Result<
            tonic::Response<super::QueryPairsLockedAndMintedStatisticByAppResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryPairsLockedAndMintedStatisticByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_stable_mint_vault_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllStableMintVaultRewardsRequest>,
        ) -> Result<tonic::Response<super::QueryAllStableMintVaultRewardsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Query/QueryAllStableMintVaultRewards",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        ///
        async fn query_vault(
            &self,
            request: tonic::Request<super::QueryVaultRequest>,
        ) -> Result<tonic::Response<super::QueryVaultResponse>, tonic::Status>;
        ///
        async fn query_user_extended_pair_total_data(
            &self,
            request: tonic::Request<super::QueryUserExtendedPairTotalDataRequest>,
        ) -> Result<tonic::Response<super::QueryUserExtendedPairTotalDataResponse>, tonic::Status>;
        ///
        async fn query_vault_info_by_vault_id(
            &self,
            request: tonic::Request<super::QueryVaultInfoByVaultIdRequest>,
        ) -> Result<tonic::Response<super::QueryVaultInfoByVaultIdResponse>, tonic::Status>;
        ///
        async fn query_vault_info_of_owner_by_app(
            &self,
            request: tonic::Request<super::QueryVaultInfoOfOwnerByAppRequest>,
        ) -> Result<tonic::Response<super::QueryVaultInfoOfOwnerByAppResponse>, tonic::Status>;
        ///
        async fn query_all_vaults(
            &self,
            request: tonic::Request<super::QueryAllVaultsRequest>,
        ) -> Result<tonic::Response<super::QueryAllVaultsResponse>, tonic::Status>;
        ///
        async fn query_all_vaults_by_app(
            &self,
            request: tonic::Request<super::QueryAllVaultsByAppRequest>,
        ) -> Result<tonic::Response<super::QueryAllVaultsByAppResponse>, tonic::Status>;
        ///
        async fn query_all_vaults_by_app_and_extended_pair(
            &self,
            request: tonic::Request<super::QueryAllVaultsByAppAndExtendedPairRequest>,
        ) -> Result<tonic::Response<super::QueryAllVaultsByAppAndExtendedPairResponse>, tonic::Status>;
        ///
        async fn query_vault_id_of_owner_by_extended_pair_and_app(
            &self,
            request: tonic::Request<super::QueryVaultIdOfOwnerByExtendedPairAndAppRequest>,
        ) -> Result<
            tonic::Response<super::QueryVaultIdOfOwnerByExtendedPairAndAppResponse>,
            tonic::Status,
        >;
        ///
        async fn query_vault_ids_by_app_in_all_extended_pairs(
            &self,
            request: tonic::Request<super::QueryVaultIdsByAppInAllExtendedPairsRequest>,
        ) -> Result<
            tonic::Response<super::QueryVaultIdsByAppInAllExtendedPairsResponse>,
            tonic::Status,
        >;
        ///
        async fn query_all_vault_ids_by_an_owner(
            &self,
            request: tonic::Request<super::QueryAllVaultIdsByAnOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryAllVaultIdsByAnOwnerResponse>, tonic::Status>;
        ///
        async fn query_token_minted_by_app_and_extended_pair(
            &self,
            request: tonic::Request<super::QueryTokenMintedByAppAndExtendedPairRequest>,
        ) -> Result<
            tonic::Response<super::QueryTokenMintedByAppAndExtendedPairResponse>,
            tonic::Status,
        >;
        ///
        async fn query_token_minted_asset_wise_by_app(
            &self,
            request: tonic::Request<super::QueryTokenMintedAssetWiseByAppRequest>,
        ) -> Result<tonic::Response<super::QueryTokenMintedAssetWiseByAppResponse>, tonic::Status>;
        ///
        async fn query_vault_count_by_app(
            &self,
            request: tonic::Request<super::QueryVaultCountByAppRequest>,
        ) -> Result<tonic::Response<super::QueryVaultCountByAppResponse>, tonic::Status>;
        ///
        async fn query_vault_count_by_app_and_extended_pair(
            &self,
            request: tonic::Request<super::QueryVaultCountByAppAndExtendedPairRequest>,
        ) -> Result<
            tonic::Response<super::QueryVaultCountByAppAndExtendedPairResponse>,
            tonic::Status,
        >;
        ///
        async fn query_total_value_locked_by_app_and_extended_pair(
            &self,
            request: tonic::Request<super::QueryTotalValueLockedByAppAndExtendedPairRequest>,
        ) -> Result<
            tonic::Response<super::QueryTotalValueLockedByAppAndExtendedPairResponse>,
            tonic::Status,
        >;
        ///
        async fn query_extended_pair_i_ds_by_app(
            &self,
            request: tonic::Request<super::QueryExtendedPairIDsByAppRequest>,
        ) -> Result<tonic::Response<super::QueryExtendedPairIDsByAppResponse>, tonic::Status>;
        ///
        async fn query_stable_vault_by_vault_id(
            &self,
            request: tonic::Request<super::QueryStableVaultByVaultIdRequest>,
        ) -> Result<tonic::Response<super::QueryStableVaultByVaultIdResponse>, tonic::Status>;
        ///
        async fn query_stable_vault_by_app(
            &self,
            request: tonic::Request<super::QueryStableVaultByAppRequest>,
        ) -> Result<tonic::Response<super::QueryStableVaultByAppResponse>, tonic::Status>;
        ///
        async fn query_stable_vault_by_app_and_extended_pair(
            &self,
            request: tonic::Request<super::QueryStableVaultByAppAndExtendedPairRequest>,
        ) -> Result<
            tonic::Response<super::QueryStableVaultByAppAndExtendedPairResponse>,
            tonic::Status,
        >;
        ///
        async fn query_extended_pair_vault_mapping_by_app_and_extended_pair(
            &self,
            request: tonic::Request<
                super::QueryExtendedPairVaultMappingByAppAndExtendedPairRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryExtendedPairVaultMappingByAppAndExtendedPairResponse>,
            tonic::Status,
        >;
        ///
        async fn query_extended_pair_vault_mapping_by_app(
            &self,
            request: tonic::Request<super::QueryExtendedPairVaultMappingByAppRequest>,
        ) -> Result<tonic::Response<super::QueryExtendedPairVaultMappingByAppResponse>, tonic::Status>;
        ///
        async fn query_tvl_by_app_of_all_extended_pairs(
            &self,
            request: tonic::Request<super::QueryTvlByAppOfAllExtendedPairsRequest>,
        ) -> Result<tonic::Response<super::QueryTvlByAppOfAllExtendedPairsResponse>, tonic::Status>;
        ///
        async fn query_tvl_by_app(
            &self,
            request: tonic::Request<super::QueryTvlByAppRequest>,
        ) -> Result<tonic::Response<super::QueryTvlByAppResponse>, tonic::Status>;
        ///
        async fn query_user_my_position_by_app(
            &self,
            request: tonic::Request<super::QueryUserMyPositionByAppRequest>,
        ) -> Result<tonic::Response<super::QueryUserMyPositionByAppResponse>, tonic::Status>;
        ///
        async fn query_pairs_locked_and_minted_statistic_by_app(
            &self,
            request: tonic::Request<super::QueryPairsLockedAndMintedStatisticByAppRequest>,
        ) -> Result<
            tonic::Response<super::QueryPairsLockedAndMintedStatisticByAppResponse>,
            tonic::Status,
        >;
        ///
        async fn query_all_stable_mint_vault_rewards(
            &self,
            request: tonic::Request<super::QueryAllStableMintVaultRewardsRequest>,
        ) -> Result<tonic::Response<super::QueryAllStableMintVaultRewardsResponse>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/comdex.vault.v1beta1.Query/QueryVault" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVaultSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryVaultRequest> for QueryVaultSvc<T> {
                        type Response = super::QueryVaultResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVaultRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_vault(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVaultSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryUserExtendedPairTotalData" => {
                    #[allow(non_camel_case_types)]
                    struct QueryUserExtendedPairTotalDataSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryUserExtendedPairTotalDataRequest>
                        for QueryUserExtendedPairTotalDataSvc<T>
                    {
                        type Response = super::QueryUserExtendedPairTotalDataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryUserExtendedPairTotalDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_user_extended_pair_total_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryUserExtendedPairTotalDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryVaultInfoByVaultID" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVaultInfoByVaultIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryVaultInfoByVaultIdRequest>
                        for QueryVaultInfoByVaultIDSvc<T>
                    {
                        type Response = super::QueryVaultInfoByVaultIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVaultInfoByVaultIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_vault_info_by_vault_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVaultInfoByVaultIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryVaultInfoOfOwnerByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVaultInfoOfOwnerByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryVaultInfoOfOwnerByAppRequest>
                        for QueryVaultInfoOfOwnerByAppSvc<T>
                    {
                        type Response = super::QueryVaultInfoOfOwnerByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVaultInfoOfOwnerByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_vault_info_of_owner_by_app(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVaultInfoOfOwnerByAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryAllVaults" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllVaultsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllVaultsRequest> for QueryAllVaultsSvc<T> {
                        type Response = super::QueryAllVaultsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllVaultsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_all_vaults(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllVaultsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryAllVaultsByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllVaultsByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllVaultsByAppRequest>
                        for QueryAllVaultsByAppSvc<T>
                    {
                        type Response = super::QueryAllVaultsByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllVaultsByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_all_vaults_by_app(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllVaultsByAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryAllVaultsByAppAndExtendedPair" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllVaultsByAppAndExtendedPairSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryAllVaultsByAppAndExtendedPairRequest,
                        > for QueryAllVaultsByAppAndExtendedPairSvc<T>
                    {
                        type Response = super::QueryAllVaultsByAppAndExtendedPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryAllVaultsByAppAndExtendedPairRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_all_vaults_by_app_and_extended_pair(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllVaultsByAppAndExtendedPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryVaultIDOfOwnerByExtendedPairAndApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVaultIDOfOwnerByExtendedPairAndAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryVaultIdOfOwnerByExtendedPairAndAppRequest,
                        > for QueryVaultIDOfOwnerByExtendedPairAndAppSvc<T>
                    {
                        type Response = super::QueryVaultIdOfOwnerByExtendedPairAndAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryVaultIdOfOwnerByExtendedPairAndAppRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_vault_id_of_owner_by_extended_pair_and_app(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVaultIDOfOwnerByExtendedPairAndAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryVaultIdsByAppInAllExtendedPairs" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVaultIdsByAppInAllExtendedPairsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryVaultIdsByAppInAllExtendedPairsRequest,
                        > for QueryVaultIdsByAppInAllExtendedPairsSvc<T>
                    {
                        type Response = super::QueryVaultIdsByAppInAllExtendedPairsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryVaultIdsByAppInAllExtendedPairsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_vault_ids_by_app_in_all_extended_pairs(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVaultIdsByAppInAllExtendedPairsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryAllVaultIdsByAnOwner" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllVaultIdsByAnOwnerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAllVaultIdsByAnOwnerRequest>
                        for QueryAllVaultIdsByAnOwnerSvc<T>
                    {
                        type Response = super::QueryAllVaultIdsByAnOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllVaultIdsByAnOwnerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_all_vault_ids_by_an_owner(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllVaultIdsByAnOwnerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryTokenMintedByAppAndExtendedPair" => {
                    #[allow(non_camel_case_types)]
                    struct QueryTokenMintedByAppAndExtendedPairSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryTokenMintedByAppAndExtendedPairRequest,
                        > for QueryTokenMintedByAppAndExtendedPairSvc<T>
                    {
                        type Response = super::QueryTokenMintedByAppAndExtendedPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTokenMintedByAppAndExtendedPairRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_token_minted_by_app_and_extended_pair(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryTokenMintedByAppAndExtendedPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryTokenMintedAssetWiseByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryTokenMintedAssetWiseByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryTokenMintedAssetWiseByAppRequest>
                        for QueryTokenMintedAssetWiseByAppSvc<T>
                    {
                        type Response = super::QueryTokenMintedAssetWiseByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTokenMintedAssetWiseByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_token_minted_asset_wise_by_app(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryTokenMintedAssetWiseByAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryVaultCountByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVaultCountByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryVaultCountByAppRequest>
                        for QueryVaultCountByAppSvc<T>
                    {
                        type Response = super::QueryVaultCountByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVaultCountByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_vault_count_by_app(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVaultCountByAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryVaultCountByAppAndExtendedPair" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVaultCountByAppAndExtendedPairSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryVaultCountByAppAndExtendedPairRequest,
                        > for QueryVaultCountByAppAndExtendedPairSvc<T>
                    {
                        type Response = super::QueryVaultCountByAppAndExtendedPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryVaultCountByAppAndExtendedPairRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_vault_count_by_app_and_extended_pair(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVaultCountByAppAndExtendedPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryTotalValueLockedByAppAndExtendedPair" => {
                    #[allow(non_camel_case_types)]
                    struct QueryTotalValueLockedByAppAndExtendedPairSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryTotalValueLockedByAppAndExtendedPairRequest,
                        > for QueryTotalValueLockedByAppAndExtendedPairSvc<T>
                    {
                        type Response = super::QueryTotalValueLockedByAppAndExtendedPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTotalValueLockedByAppAndExtendedPairRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_total_value_locked_by_app_and_extended_pair(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryTotalValueLockedByAppAndExtendedPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryExtendedPairIDsByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryExtendedPairIDsByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryExtendedPairIDsByAppRequest>
                        for QueryExtendedPairIDsByAppSvc<T>
                    {
                        type Response = super::QueryExtendedPairIDsByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryExtendedPairIDsByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_extended_pair_i_ds_by_app(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryExtendedPairIDsByAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryStableVaultByVaultID" => {
                    #[allow(non_camel_case_types)]
                    struct QueryStableVaultByVaultIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryStableVaultByVaultIdRequest>
                        for QueryStableVaultByVaultIDSvc<T>
                    {
                        type Response = super::QueryStableVaultByVaultIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryStableVaultByVaultIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_stable_vault_by_vault_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryStableVaultByVaultIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryStableVaultByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryStableVaultByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryStableVaultByAppRequest>
                        for QueryStableVaultByAppSvc<T>
                    {
                        type Response = super::QueryStableVaultByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryStableVaultByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_stable_vault_by_app(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryStableVaultByAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryStableVaultByAppAndExtendedPair" => {
                    #[allow(non_camel_case_types)]
                    struct QueryStableVaultByAppAndExtendedPairSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryStableVaultByAppAndExtendedPairRequest,
                        > for QueryStableVaultByAppAndExtendedPairSvc<T>
                    {
                        type Response = super::QueryStableVaultByAppAndExtendedPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryStableVaultByAppAndExtendedPairRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_stable_vault_by_app_and_extended_pair(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryStableVaultByAppAndExtendedPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryExtendedPairVaultMappingByAppAndExtendedPair" => {
                    #[allow(non_camel_case_types)]
                    struct QueryExtendedPairVaultMappingByAppAndExtendedPairSvc<T: Query>(
                        pub Arc<T>,
                    );
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryExtendedPairVaultMappingByAppAndExtendedPairRequest,
                        > for QueryExtendedPairVaultMappingByAppAndExtendedPairSvc<T>
                    {
                        type Response =
                            super::QueryExtendedPairVaultMappingByAppAndExtendedPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryExtendedPairVaultMappingByAppAndExtendedPairRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_extended_pair_vault_mapping_by_app_and_extended_pair(
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryExtendedPairVaultMappingByAppAndExtendedPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryExtendedPairVaultMappingByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryExtendedPairVaultMappingByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryExtendedPairVaultMappingByAppRequest,
                        > for QueryExtendedPairVaultMappingByAppSvc<T>
                    {
                        type Response = super::QueryExtendedPairVaultMappingByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryExtendedPairVaultMappingByAppRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_extended_pair_vault_mapping_by_app(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryExtendedPairVaultMappingByAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryTVLByAppOfAllExtendedPairs" => {
                    #[allow(non_camel_case_types)]
                    struct QueryTVLByAppOfAllExtendedPairsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryTvlByAppOfAllExtendedPairsRequest>
                        for QueryTVLByAppOfAllExtendedPairsSvc<T>
                    {
                        type Response = super::QueryTvlByAppOfAllExtendedPairsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTvlByAppOfAllExtendedPairsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_tvl_by_app_of_all_extended_pairs(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryTVLByAppOfAllExtendedPairsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryTVLByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryTVLByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTvlByAppRequest> for QueryTVLByAppSvc<T> {
                        type Response = super::QueryTvlByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTvlByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_tvl_by_app(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryTVLByAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryUserMyPositionByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryUserMyPositionByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryUserMyPositionByAppRequest>
                        for QueryUserMyPositionByAppSvc<T>
                    {
                        type Response = super::QueryUserMyPositionByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryUserMyPositionByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_user_my_position_by_app(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryUserMyPositionByAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryPairsLockedAndMintedStatisticByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryPairsLockedAndMintedStatisticByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryPairsLockedAndMintedStatisticByAppRequest,
                        > for QueryPairsLockedAndMintedStatisticByAppSvc<T>
                    {
                        type Response = super::QueryPairsLockedAndMintedStatisticByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryPairsLockedAndMintedStatisticByAppRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_pairs_locked_and_minted_statistic_by_app(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryPairsLockedAndMintedStatisticByAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Query/QueryAllStableMintVaultRewards" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllStableMintVaultRewardsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAllStableMintVaultRewardsRequest>
                        for QueryAllStableMintVaultRewardsSvc<T>
                    {
                        type Response = super::QueryAllStableMintVaultRewardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllStableMintVaultRewardsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_all_stable_mint_vault_rewards(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllStableMintVaultRewardsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "comdex.vault.v1beta1.Query";
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    ///
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///
        pub async fn msg_create(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateRequest>,
        ) -> Result<tonic::Response<super::MsgCreateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.vault.v1beta1.Msg/MsgCreate");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_deposit(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositRequest>,
        ) -> Result<tonic::Response<super::MsgDepositResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.vault.v1beta1.Msg/MsgDeposit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawRequest>,
        ) -> Result<tonic::Response<super::MsgWithdrawResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.vault.v1beta1.Msg/MsgWithdraw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_draw(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDrawRequest>,
        ) -> Result<tonic::Response<super::MsgDrawResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.vault.v1beta1.Msg/MsgDraw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_repay(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRepayRequest>,
        ) -> Result<tonic::Response<super::MsgRepayResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.vault.v1beta1.Msg/MsgRepay");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_close(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCloseRequest>,
        ) -> Result<tonic::Response<super::MsgCloseResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.vault.v1beta1.Msg/MsgClose");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_deposit_and_draw(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositAndDrawRequest>,
        ) -> Result<tonic::Response<super::MsgDepositAndDrawResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.vault.v1beta1.Msg/MsgDepositAndDraw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_create_stable_mint(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateStableMintRequest>,
        ) -> Result<tonic::Response<super::MsgCreateStableMintResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Msg/MsgCreateStableMint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_deposit_stable_mint(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositStableMintRequest>,
        ) -> Result<tonic::Response<super::MsgDepositStableMintResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Msg/MsgDepositStableMint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_withdraw_stable_mint(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawStableMintRequest>,
        ) -> Result<tonic::Response<super::MsgWithdrawStableMintResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Msg/MsgWithdrawStableMint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_vault_interest_calc(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgVaultInterestCalcRequest>,
        ) -> Result<tonic::Response<super::MsgVaultInterestCalcResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.vault.v1beta1.Msg/MsgVaultInterestCalc",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        ///
        async fn msg_create(
            &self,
            request: tonic::Request<super::MsgCreateRequest>,
        ) -> Result<tonic::Response<super::MsgCreateResponse>, tonic::Status>;
        ///
        async fn msg_deposit(
            &self,
            request: tonic::Request<super::MsgDepositRequest>,
        ) -> Result<tonic::Response<super::MsgDepositResponse>, tonic::Status>;
        ///
        async fn msg_withdraw(
            &self,
            request: tonic::Request<super::MsgWithdrawRequest>,
        ) -> Result<tonic::Response<super::MsgWithdrawResponse>, tonic::Status>;
        ///
        async fn msg_draw(
            &self,
            request: tonic::Request<super::MsgDrawRequest>,
        ) -> Result<tonic::Response<super::MsgDrawResponse>, tonic::Status>;
        ///
        async fn msg_repay(
            &self,
            request: tonic::Request<super::MsgRepayRequest>,
        ) -> Result<tonic::Response<super::MsgRepayResponse>, tonic::Status>;
        ///
        async fn msg_close(
            &self,
            request: tonic::Request<super::MsgCloseRequest>,
        ) -> Result<tonic::Response<super::MsgCloseResponse>, tonic::Status>;
        ///
        async fn msg_deposit_and_draw(
            &self,
            request: tonic::Request<super::MsgDepositAndDrawRequest>,
        ) -> Result<tonic::Response<super::MsgDepositAndDrawResponse>, tonic::Status>;
        ///
        async fn msg_create_stable_mint(
            &self,
            request: tonic::Request<super::MsgCreateStableMintRequest>,
        ) -> Result<tonic::Response<super::MsgCreateStableMintResponse>, tonic::Status>;
        ///
        async fn msg_deposit_stable_mint(
            &self,
            request: tonic::Request<super::MsgDepositStableMintRequest>,
        ) -> Result<tonic::Response<super::MsgDepositStableMintResponse>, tonic::Status>;
        ///
        async fn msg_withdraw_stable_mint(
            &self,
            request: tonic::Request<super::MsgWithdrawStableMintRequest>,
        ) -> Result<tonic::Response<super::MsgWithdrawStableMintResponse>, tonic::Status>;
        ///
        async fn msg_vault_interest_calc(
            &self,
            request: tonic::Request<super::MsgVaultInterestCalcRequest>,
        ) -> Result<tonic::Response<super::MsgVaultInterestCalcResponse>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/comdex.vault.v1beta1.Msg/MsgCreate" => {
                    #[allow(non_camel_case_types)]
                    struct MsgCreateSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateRequest> for MsgCreateSvc<T> {
                        type Response = super::MsgCreateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Msg/MsgDeposit" => {
                    #[allow(non_camel_case_types)]
                    struct MsgDepositSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDepositRequest> for MsgDepositSvc<T> {
                        type Response = super::MsgDepositResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDepositRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_deposit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgDepositSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Msg/MsgWithdraw" => {
                    #[allow(non_camel_case_types)]
                    struct MsgWithdrawSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawRequest> for MsgWithdrawSvc<T> {
                        type Response = super::MsgWithdrawResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_withdraw(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgWithdrawSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Msg/MsgDraw" => {
                    #[allow(non_camel_case_types)]
                    struct MsgDrawSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDrawRequest> for MsgDrawSvc<T> {
                        type Response = super::MsgDrawResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDrawRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_draw(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgDrawSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Msg/MsgRepay" => {
                    #[allow(non_camel_case_types)]
                    struct MsgRepaySvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRepayRequest> for MsgRepaySvc<T> {
                        type Response = super::MsgRepayResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRepayRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_repay(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgRepaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Msg/MsgClose" => {
                    #[allow(non_camel_case_types)]
                    struct MsgCloseSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCloseRequest> for MsgCloseSvc<T> {
                        type Response = super::MsgCloseResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCloseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_close(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgCloseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Msg/MsgDepositAndDraw" => {
                    #[allow(non_camel_case_types)]
                    struct MsgDepositAndDrawSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDepositAndDrawRequest>
                        for MsgDepositAndDrawSvc<T>
                    {
                        type Response = super::MsgDepositAndDrawResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDepositAndDrawRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_deposit_and_draw(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgDepositAndDrawSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Msg/MsgCreateStableMint" => {
                    #[allow(non_camel_case_types)]
                    struct MsgCreateStableMintSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateStableMintRequest>
                        for MsgCreateStableMintSvc<T>
                    {
                        type Response = super::MsgCreateStableMintResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateStableMintRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_create_stable_mint(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgCreateStableMintSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Msg/MsgDepositStableMint" => {
                    #[allow(non_camel_case_types)]
                    struct MsgDepositStableMintSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDepositStableMintRequest>
                        for MsgDepositStableMintSvc<T>
                    {
                        type Response = super::MsgDepositStableMintResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDepositStableMintRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).msg_deposit_stable_mint(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgDepositStableMintSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Msg/MsgWithdrawStableMint" => {
                    #[allow(non_camel_case_types)]
                    struct MsgWithdrawStableMintSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawStableMintRequest>
                        for MsgWithdrawStableMintSvc<T>
                    {
                        type Response = super::MsgWithdrawStableMintResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawStableMintRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).msg_withdraw_stable_mint(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgWithdrawStableMintSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/comdex.vault.v1beta1.Msg/MsgVaultInterestCalc" => {
                    #[allow(non_camel_case_types)]
                    struct MsgVaultInterestCalcSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgVaultInterestCalcRequest>
                        for MsgVaultInterestCalcSvc<T>
                    {
                        type Response = super::MsgVaultInterestCalcResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgVaultInterestCalcRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).msg_vault_interest_calc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgVaultInterestCalcSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "comdex.vault.v1beta1.Msg";
    }
}
