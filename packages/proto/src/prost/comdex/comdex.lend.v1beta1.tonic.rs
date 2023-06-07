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
        pub async fn query_lends(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLendsRequest>,
        ) -> Result<tonic::Response<super::QueryLendsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Query/QueryLends");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_lend(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLendRequest>,
        ) -> Result<tonic::Response<super::QueryLendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Query/QueryLend");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_lend_by_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllLendByOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryAllLendByOwnerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryAllLendByOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_lend_by_owner_and_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllLendByOwnerAndPoolRequest>,
        ) -> Result<tonic::Response<super::QueryAllLendByOwnerAndPoolResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryAllLendByOwnerAndPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_pairs(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPairsRequest>,
        ) -> Result<tonic::Response<super::QueryPairsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Query/QueryPairs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPairRequest>,
        ) -> Result<tonic::Response<super::QueryPairResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Query/QueryPair");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_asset_rates_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAssetRatesParamsRequest>,
        ) -> Result<tonic::Response<super::QueryAssetRatesParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryAssetRatesParams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_asset_rates_param(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAssetRatesParamRequest>,
        ) -> Result<tonic::Response<super::QueryAssetRatesParamResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryAssetRatesParam",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolsRequest>,
        ) -> Result<tonic::Response<super::QueryPoolsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Query/QueryPools");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolRequest>,
        ) -> Result<tonic::Response<super::QueryPoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Query/QueryPool");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_asset_to_pair_mappings(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAssetToPairMappingsRequest>,
        ) -> Result<tonic::Response<super::QueryAssetToPairMappingsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryAssetToPairMappings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_asset_to_pair_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAssetToPairMappingRequest>,
        ) -> Result<tonic::Response<super::QueryAssetToPairMappingResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryAssetToPairMapping",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_borrows(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBorrowsRequest>,
        ) -> Result<tonic::Response<super::QueryBorrowsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Query/QueryBorrows");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_borrow(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBorrowRequest>,
        ) -> Result<tonic::Response<super::QueryBorrowResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Query/QueryBorrow");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_borrow_by_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllBorrowByOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryAllBorrowByOwnerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryAllBorrowByOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_borrow_by_owner_and_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllBorrowByOwnerAndPoolRequest>,
        ) -> Result<tonic::Response<super::QueryAllBorrowByOwnerAndPoolResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryAllBorrowByOwnerAndPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_pool_asset_lb_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolAssetLbMappingRequest>,
        ) -> Result<tonic::Response<super::QueryPoolAssetLbMappingResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryPoolAssetLBMapping",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_reserve_buyback_asset_data(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryReserveBuybackAssetDataRequest>,
        ) -> Result<tonic::Response<super::QueryReserveBuybackAssetDataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryReserveBuybackAssetData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_auction_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAuctionParamRequest>,
        ) -> Result<tonic::Response<super::QueryAuctionParamResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryAuctionParams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_module_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryModuleBalanceRequest>,
        ) -> Result<tonic::Response<super::QueryModuleBalanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryModuleBalance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_fund_mod_bal(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFundModBalRequest>,
        ) -> Result<tonic::Response<super::QueryFundModBalResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Query/QueryFundModBal");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_fund_reserve_bal(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFundReserveBalRequest>,
        ) -> Result<tonic::Response<super::QueryFundReserveBalResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryFundReserveBal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_reserve_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllReserveStatsRequest>,
        ) -> Result<tonic::Response<super::QueryAllReserveStatsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryAllReserveStats",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_fund_mod_bal_by_asset_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFundModBalByAssetPoolRequest>,
        ) -> Result<tonic::Response<super::QueryFundModBalByAssetPoolResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryFundModBalByAssetPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_lend_interest(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLendInterestRequest>,
        ) -> Result<tonic::Response<super::QueryLendInterestResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryLendInterest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_borrow_interest(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBorrowInterestRequest>,
        ) -> Result<tonic::Response<super::QueryBorrowInterestResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Query/QueryBorrowInterest",
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
        async fn query_lends(
            &self,
            request: tonic::Request<super::QueryLendsRequest>,
        ) -> Result<tonic::Response<super::QueryLendsResponse>, tonic::Status>;
        ///
        async fn query_lend(
            &self,
            request: tonic::Request<super::QueryLendRequest>,
        ) -> Result<tonic::Response<super::QueryLendResponse>, tonic::Status>;
        ///
        async fn query_all_lend_by_owner(
            &self,
            request: tonic::Request<super::QueryAllLendByOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryAllLendByOwnerResponse>, tonic::Status>;
        ///
        async fn query_all_lend_by_owner_and_pool(
            &self,
            request: tonic::Request<super::QueryAllLendByOwnerAndPoolRequest>,
        ) -> Result<tonic::Response<super::QueryAllLendByOwnerAndPoolResponse>, tonic::Status>;
        ///
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        ///
        async fn query_pairs(
            &self,
            request: tonic::Request<super::QueryPairsRequest>,
        ) -> Result<tonic::Response<super::QueryPairsResponse>, tonic::Status>;
        ///
        async fn query_pair(
            &self,
            request: tonic::Request<super::QueryPairRequest>,
        ) -> Result<tonic::Response<super::QueryPairResponse>, tonic::Status>;
        ///
        async fn query_asset_rates_params(
            &self,
            request: tonic::Request<super::QueryAssetRatesParamsRequest>,
        ) -> Result<tonic::Response<super::QueryAssetRatesParamsResponse>, tonic::Status>;
        ///
        async fn query_asset_rates_param(
            &self,
            request: tonic::Request<super::QueryAssetRatesParamRequest>,
        ) -> Result<tonic::Response<super::QueryAssetRatesParamResponse>, tonic::Status>;
        ///
        async fn query_pools(
            &self,
            request: tonic::Request<super::QueryPoolsRequest>,
        ) -> Result<tonic::Response<super::QueryPoolsResponse>, tonic::Status>;
        ///
        async fn query_pool(
            &self,
            request: tonic::Request<super::QueryPoolRequest>,
        ) -> Result<tonic::Response<super::QueryPoolResponse>, tonic::Status>;
        ///
        async fn query_asset_to_pair_mappings(
            &self,
            request: tonic::Request<super::QueryAssetToPairMappingsRequest>,
        ) -> Result<tonic::Response<super::QueryAssetToPairMappingsResponse>, tonic::Status>;
        ///
        async fn query_asset_to_pair_mapping(
            &self,
            request: tonic::Request<super::QueryAssetToPairMappingRequest>,
        ) -> Result<tonic::Response<super::QueryAssetToPairMappingResponse>, tonic::Status>;
        ///
        async fn query_borrows(
            &self,
            request: tonic::Request<super::QueryBorrowsRequest>,
        ) -> Result<tonic::Response<super::QueryBorrowsResponse>, tonic::Status>;
        ///
        async fn query_borrow(
            &self,
            request: tonic::Request<super::QueryBorrowRequest>,
        ) -> Result<tonic::Response<super::QueryBorrowResponse>, tonic::Status>;
        ///
        async fn query_all_borrow_by_owner(
            &self,
            request: tonic::Request<super::QueryAllBorrowByOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryAllBorrowByOwnerResponse>, tonic::Status>;
        ///
        async fn query_all_borrow_by_owner_and_pool(
            &self,
            request: tonic::Request<super::QueryAllBorrowByOwnerAndPoolRequest>,
        ) -> Result<tonic::Response<super::QueryAllBorrowByOwnerAndPoolResponse>, tonic::Status>;
        ///
        async fn query_pool_asset_lb_mapping(
            &self,
            request: tonic::Request<super::QueryPoolAssetLbMappingRequest>,
        ) -> Result<tonic::Response<super::QueryPoolAssetLbMappingResponse>, tonic::Status>;
        ///
        async fn query_reserve_buyback_asset_data(
            &self,
            request: tonic::Request<super::QueryReserveBuybackAssetDataRequest>,
        ) -> Result<tonic::Response<super::QueryReserveBuybackAssetDataResponse>, tonic::Status>;
        ///
        async fn query_auction_params(
            &self,
            request: tonic::Request<super::QueryAuctionParamRequest>,
        ) -> Result<tonic::Response<super::QueryAuctionParamResponse>, tonic::Status>;
        ///
        async fn query_module_balance(
            &self,
            request: tonic::Request<super::QueryModuleBalanceRequest>,
        ) -> Result<tonic::Response<super::QueryModuleBalanceResponse>, tonic::Status>;
        ///
        async fn query_fund_mod_bal(
            &self,
            request: tonic::Request<super::QueryFundModBalRequest>,
        ) -> Result<tonic::Response<super::QueryFundModBalResponse>, tonic::Status>;
        ///
        async fn query_fund_reserve_bal(
            &self,
            request: tonic::Request<super::QueryFundReserveBalRequest>,
        ) -> Result<tonic::Response<super::QueryFundReserveBalResponse>, tonic::Status>;
        ///
        async fn query_all_reserve_stats(
            &self,
            request: tonic::Request<super::QueryAllReserveStatsRequest>,
        ) -> Result<tonic::Response<super::QueryAllReserveStatsResponse>, tonic::Status>;
        ///
        async fn query_fund_mod_bal_by_asset_pool(
            &self,
            request: tonic::Request<super::QueryFundModBalByAssetPoolRequest>,
        ) -> Result<tonic::Response<super::QueryFundModBalByAssetPoolResponse>, tonic::Status>;
        ///
        async fn query_lend_interest(
            &self,
            request: tonic::Request<super::QueryLendInterestRequest>,
        ) -> Result<tonic::Response<super::QueryLendInterestResponse>, tonic::Status>;
        ///
        async fn query_borrow_interest(
            &self,
            request: tonic::Request<super::QueryBorrowInterestRequest>,
        ) -> Result<tonic::Response<super::QueryBorrowInterestResponse>, tonic::Status>;
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
                "/comdex.lend.v1beta1.Query/QueryLends" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLendsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryLendsRequest> for QueryLendsSvc<T> {
                        type Response = super::QueryLendsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLendsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_lends(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLendsSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryLend" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLendSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryLendRequest> for QueryLendSvc<T> {
                        type Response = super::QueryLendResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLendRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_lend(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLendSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryAllLendByOwner" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllLendByOwnerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllLendByOwnerRequest>
                        for QueryAllLendByOwnerSvc<T>
                    {
                        type Response = super::QueryAllLendByOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllLendByOwnerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_all_lend_by_owner(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllLendByOwnerSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryAllLendByOwnerAndPool" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllLendByOwnerAndPoolSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAllLendByOwnerAndPoolRequest>
                        for QueryAllLendByOwnerAndPoolSvc<T>
                    {
                        type Response = super::QueryAllLendByOwnerAndPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllLendByOwnerAndPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_all_lend_by_owner_and_pool(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllLendByOwnerAndPoolSvc(inner);
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
                "/comdex.lend.v1beta1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryPairs" => {
                    #[allow(non_camel_case_types)]
                    struct QueryPairsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPairsRequest> for QueryPairsSvc<T> {
                        type Response = super::QueryPairsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPairsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_pairs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryPairsSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryPair" => {
                    #[allow(non_camel_case_types)]
                    struct QueryPairSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPairRequest> for QueryPairSvc<T> {
                        type Response = super::QueryPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPairRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_pair(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryPairSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryAssetRatesParams" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAssetRatesParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAssetRatesParamsRequest>
                        for QueryAssetRatesParamsSvc<T>
                    {
                        type Response = super::QueryAssetRatesParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAssetRatesParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_asset_rates_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAssetRatesParamsSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryAssetRatesParam" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAssetRatesParamSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAssetRatesParamRequest>
                        for QueryAssetRatesParamSvc<T>
                    {
                        type Response = super::QueryAssetRatesParamResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAssetRatesParamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_asset_rates_param(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAssetRatesParamSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryPools" => {
                    #[allow(non_camel_case_types)]
                    struct QueryPoolsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPoolsRequest> for QueryPoolsSvc<T> {
                        type Response = super::QueryPoolsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_pools(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryPoolsSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryPool" => {
                    #[allow(non_camel_case_types)]
                    struct QueryPoolSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPoolRequest> for QueryPoolSvc<T> {
                        type Response = super::QueryPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryPoolSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryAssetToPairMappings" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAssetToPairMappingsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAssetToPairMappingsRequest>
                        for QueryAssetToPairMappingsSvc<T>
                    {
                        type Response = super::QueryAssetToPairMappingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAssetToPairMappingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_asset_to_pair_mappings(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAssetToPairMappingsSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryAssetToPairMapping" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAssetToPairMappingSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAssetToPairMappingRequest>
                        for QueryAssetToPairMappingSvc<T>
                    {
                        type Response = super::QueryAssetToPairMappingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAssetToPairMappingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_asset_to_pair_mapping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAssetToPairMappingSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryBorrows" => {
                    #[allow(non_camel_case_types)]
                    struct QueryBorrowsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryBorrowsRequest> for QueryBorrowsSvc<T> {
                        type Response = super::QueryBorrowsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryBorrowsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_borrows(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryBorrowsSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryBorrow" => {
                    #[allow(non_camel_case_types)]
                    struct QueryBorrowSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryBorrowRequest> for QueryBorrowSvc<T> {
                        type Response = super::QueryBorrowResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryBorrowRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_borrow(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryBorrowSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryAllBorrowByOwner" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllBorrowByOwnerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllBorrowByOwnerRequest>
                        for QueryAllBorrowByOwnerSvc<T>
                    {
                        type Response = super::QueryAllBorrowByOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllBorrowByOwnerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_all_borrow_by_owner(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllBorrowByOwnerSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryAllBorrowByOwnerAndPool" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllBorrowByOwnerAndPoolSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAllBorrowByOwnerAndPoolRequest>
                        for QueryAllBorrowByOwnerAndPoolSvc<T>
                    {
                        type Response = super::QueryAllBorrowByOwnerAndPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllBorrowByOwnerAndPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_all_borrow_by_owner_and_pool(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllBorrowByOwnerAndPoolSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryPoolAssetLBMapping" => {
                    #[allow(non_camel_case_types)]
                    struct QueryPoolAssetLBMappingSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryPoolAssetLbMappingRequest>
                        for QueryPoolAssetLBMappingSvc<T>
                    {
                        type Response = super::QueryPoolAssetLbMappingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolAssetLbMappingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_pool_asset_lb_mapping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryPoolAssetLBMappingSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryReserveBuybackAssetData" => {
                    #[allow(non_camel_case_types)]
                    struct QueryReserveBuybackAssetDataSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryReserveBuybackAssetDataRequest>
                        for QueryReserveBuybackAssetDataSvc<T>
                    {
                        type Response = super::QueryReserveBuybackAssetDataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryReserveBuybackAssetDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_reserve_buyback_asset_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryReserveBuybackAssetDataSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryAuctionParams" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAuctionParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAuctionParamRequest>
                        for QueryAuctionParamsSvc<T>
                    {
                        type Response = super::QueryAuctionParamResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAuctionParamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_auction_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAuctionParamsSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryModuleBalance" => {
                    #[allow(non_camel_case_types)]
                    struct QueryModuleBalanceSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryModuleBalanceRequest>
                        for QueryModuleBalanceSvc<T>
                    {
                        type Response = super::QueryModuleBalanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryModuleBalanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_module_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryModuleBalanceSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryFundModBal" => {
                    #[allow(non_camel_case_types)]
                    struct QueryFundModBalSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryFundModBalRequest>
                        for QueryFundModBalSvc<T>
                    {
                        type Response = super::QueryFundModBalResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFundModBalRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_fund_mod_bal(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryFundModBalSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryFundReserveBal" => {
                    #[allow(non_camel_case_types)]
                    struct QueryFundReserveBalSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryFundReserveBalRequest>
                        for QueryFundReserveBalSvc<T>
                    {
                        type Response = super::QueryFundReserveBalResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFundReserveBalRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_fund_reserve_bal(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryFundReserveBalSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryAllReserveStats" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllReserveStatsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllReserveStatsRequest>
                        for QueryAllReserveStatsSvc<T>
                    {
                        type Response = super::QueryAllReserveStatsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllReserveStatsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_all_reserve_stats(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllReserveStatsSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryFundModBalByAssetPool" => {
                    #[allow(non_camel_case_types)]
                    struct QueryFundModBalByAssetPoolSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryFundModBalByAssetPoolRequest>
                        for QueryFundModBalByAssetPoolSvc<T>
                    {
                        type Response = super::QueryFundModBalByAssetPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFundModBalByAssetPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_fund_mod_bal_by_asset_pool(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryFundModBalByAssetPoolSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryLendInterest" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLendInterestSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryLendInterestRequest>
                        for QueryLendInterestSvc<T>
                    {
                        type Response = super::QueryLendInterestResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLendInterestRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_lend_interest(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLendInterestSvc(inner);
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
                "/comdex.lend.v1beta1.Query/QueryBorrowInterest" => {
                    #[allow(non_camel_case_types)]
                    struct QueryBorrowInterestSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryBorrowInterestRequest>
                        for QueryBorrowInterestSvc<T>
                    {
                        type Response = super::QueryBorrowInterestResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryBorrowInterestRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_borrow_interest(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryBorrowInterestSvc(inner);
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
        const NAME: &'static str = "comdex.lend.v1beta1.Query";
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
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
        pub async fn lend(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLend>,
        ) -> Result<tonic::Response<super::MsgLendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/Lend");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdraw>,
        ) -> Result<tonic::Response<super::MsgWithdrawResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/Withdraw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn deposit(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeposit>,
        ) -> Result<tonic::Response<super::MsgDepositResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/Deposit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn close_lend(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCloseLend>,
        ) -> Result<tonic::Response<super::MsgCloseLendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/CloseLend");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn borrow(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBorrow>,
        ) -> Result<tonic::Response<super::MsgBorrowResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/Borrow");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn repay(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRepay>,
        ) -> Result<tonic::Response<super::MsgRepayResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/Repay");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn deposit_borrow(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositBorrow>,
        ) -> Result<tonic::Response<super::MsgDepositBorrowResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/DepositBorrow");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn draw(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDraw>,
        ) -> Result<tonic::Response<super::MsgDrawResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/Draw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn close_borrow(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCloseBorrow>,
        ) -> Result<tonic::Response<super::MsgCloseBorrowResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/CloseBorrow");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn borrow_alternate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBorrowAlternate>,
        ) -> Result<tonic::Response<super::MsgBorrowAlternateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/BorrowAlternate");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fund_module_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFundModuleAccounts>,
        ) -> Result<tonic::Response<super::MsgFundModuleAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.lend.v1beta1.Msg/FundModuleAccounts");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn calculate_interest_and_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCalculateInterestAndRewards>,
        ) -> Result<tonic::Response<super::MsgCalculateInterestAndRewardsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Msg/CalculateInterestAndRewards",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fund_reserve_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFundReserveAccounts>,
        ) -> Result<tonic::Response<super::MsgFundReserveAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.lend.v1beta1.Msg/FundReserveAccounts",
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
        async fn lend(
            &self,
            request: tonic::Request<super::MsgLend>,
        ) -> Result<tonic::Response<super::MsgLendResponse>, tonic::Status>;
        async fn withdraw(
            &self,
            request: tonic::Request<super::MsgWithdraw>,
        ) -> Result<tonic::Response<super::MsgWithdrawResponse>, tonic::Status>;
        async fn deposit(
            &self,
            request: tonic::Request<super::MsgDeposit>,
        ) -> Result<tonic::Response<super::MsgDepositResponse>, tonic::Status>;
        async fn close_lend(
            &self,
            request: tonic::Request<super::MsgCloseLend>,
        ) -> Result<tonic::Response<super::MsgCloseLendResponse>, tonic::Status>;
        async fn borrow(
            &self,
            request: tonic::Request<super::MsgBorrow>,
        ) -> Result<tonic::Response<super::MsgBorrowResponse>, tonic::Status>;
        async fn repay(
            &self,
            request: tonic::Request<super::MsgRepay>,
        ) -> Result<tonic::Response<super::MsgRepayResponse>, tonic::Status>;
        async fn deposit_borrow(
            &self,
            request: tonic::Request<super::MsgDepositBorrow>,
        ) -> Result<tonic::Response<super::MsgDepositBorrowResponse>, tonic::Status>;
        async fn draw(
            &self,
            request: tonic::Request<super::MsgDraw>,
        ) -> Result<tonic::Response<super::MsgDrawResponse>, tonic::Status>;
        async fn close_borrow(
            &self,
            request: tonic::Request<super::MsgCloseBorrow>,
        ) -> Result<tonic::Response<super::MsgCloseBorrowResponse>, tonic::Status>;
        async fn borrow_alternate(
            &self,
            request: tonic::Request<super::MsgBorrowAlternate>,
        ) -> Result<tonic::Response<super::MsgBorrowAlternateResponse>, tonic::Status>;
        async fn fund_module_accounts(
            &self,
            request: tonic::Request<super::MsgFundModuleAccounts>,
        ) -> Result<tonic::Response<super::MsgFundModuleAccountsResponse>, tonic::Status>;
        async fn calculate_interest_and_rewards(
            &self,
            request: tonic::Request<super::MsgCalculateInterestAndRewards>,
        ) -> Result<tonic::Response<super::MsgCalculateInterestAndRewardsResponse>, tonic::Status>;
        async fn fund_reserve_accounts(
            &self,
            request: tonic::Request<super::MsgFundReserveAccounts>,
        ) -> Result<tonic::Response<super::MsgFundReserveAccountsResponse>, tonic::Status>;
    }
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
                "/comdex.lend.v1beta1.Msg/Lend" => {
                    #[allow(non_camel_case_types)]
                    struct LendSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgLend> for LendSvc<T> {
                        type Response = super::MsgLendResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgLend>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).lend(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LendSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/Withdraw" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdraw> for WithdrawSvc<T> {
                        type Response = super::MsgWithdrawResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdraw>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).withdraw(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/Deposit" => {
                    #[allow(non_camel_case_types)]
                    struct DepositSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeposit> for DepositSvc<T> {
                        type Response = super::MsgDepositResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeposit>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).deposit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DepositSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/CloseLend" => {
                    #[allow(non_camel_case_types)]
                    struct CloseLendSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCloseLend> for CloseLendSvc<T> {
                        type Response = super::MsgCloseLendResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCloseLend>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).close_lend(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloseLendSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/Borrow" => {
                    #[allow(non_camel_case_types)]
                    struct BorrowSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBorrow> for BorrowSvc<T> {
                        type Response = super::MsgBorrowResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBorrow>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).borrow(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BorrowSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/Repay" => {
                    #[allow(non_camel_case_types)]
                    struct RepaySvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRepay> for RepaySvc<T> {
                        type Response = super::MsgRepayResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRepay>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).repay(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RepaySvc(inner);
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
                "/comdex.lend.v1beta1.Msg/DepositBorrow" => {
                    #[allow(non_camel_case_types)]
                    struct DepositBorrowSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDepositBorrow> for DepositBorrowSvc<T> {
                        type Response = super::MsgDepositBorrowResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDepositBorrow>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).deposit_borrow(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DepositBorrowSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/Draw" => {
                    #[allow(non_camel_case_types)]
                    struct DrawSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDraw> for DrawSvc<T> {
                        type Response = super::MsgDrawResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDraw>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).draw(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DrawSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/CloseBorrow" => {
                    #[allow(non_camel_case_types)]
                    struct CloseBorrowSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCloseBorrow> for CloseBorrowSvc<T> {
                        type Response = super::MsgCloseBorrowResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCloseBorrow>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).close_borrow(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloseBorrowSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/BorrowAlternate" => {
                    #[allow(non_camel_case_types)]
                    struct BorrowAlternateSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBorrowAlternate> for BorrowAlternateSvc<T> {
                        type Response = super::MsgBorrowAlternateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBorrowAlternate>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).borrow_alternate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BorrowAlternateSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/FundModuleAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct FundModuleAccountsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgFundModuleAccounts>
                        for FundModuleAccountsSvc<T>
                    {
                        type Response = super::MsgFundModuleAccountsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgFundModuleAccounts>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fund_module_accounts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FundModuleAccountsSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/CalculateInterestAndRewards" => {
                    #[allow(non_camel_case_types)]
                    struct CalculateInterestAndRewardsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCalculateInterestAndRewards>
                        for CalculateInterestAndRewardsSvc<T>
                    {
                        type Response = super::MsgCalculateInterestAndRewardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCalculateInterestAndRewards>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).calculate_interest_and_rewards(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CalculateInterestAndRewardsSvc(inner);
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
                "/comdex.lend.v1beta1.Msg/FundReserveAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct FundReserveAccountsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgFundReserveAccounts>
                        for FundReserveAccountsSvc<T>
                    {
                        type Response = super::MsgFundReserveAccountsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgFundReserveAccounts>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fund_reserve_accounts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FundReserveAccountsSvc(inner);
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
        const NAME: &'static str = "comdex.lend.v1beta1.Msg";
    }
}
