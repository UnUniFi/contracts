// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /** Query defines the gRPC querier service.
    */
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
        /** Params returns parameters of the module.
        */
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
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** GenericParams returns app parameters of the module.
        */
        pub async fn generic_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGenericParamsRequest>,
        ) -> Result<tonic::Response<super::QueryGenericParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/GenericParams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Pools returns all liquidity pools.
        */
        pub async fn pools(
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
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Query/Pools");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Pool returns the specific liquidity pool.
        */
        pub async fn pool(
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
            let path = http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Query/Pool");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** PoolByReserveAddress returns all pools that correspond to the reserve account.
        */
        pub async fn pool_by_reserve_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolByReserveAddressRequest>,
        ) -> Result<tonic::Response<super::QueryPoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/PoolByReserveAddress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** PoolByPoolCoinDenom returns all pools that correspond to the pool coin denom.
        */
        pub async fn pool_by_pool_coin_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolByPoolCoinDenomRequest>,
        ) -> Result<tonic::Response<super::QueryPoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/PoolByPoolCoinDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Pairs returns all liquidity pairs.
        */
        pub async fn pairs(
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
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Query/Pairs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Pair returns the specific pair.
        */
        pub async fn pair(
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
            let path = http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Query/Pair");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** DepositRequests returns all deposit requests.
        */
        pub async fn deposit_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDepositRequestsRequest>,
        ) -> Result<tonic::Response<super::QueryDepositRequestsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/DepositRequests",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** DepositRequest returns the specific deposit request.
        */
        pub async fn deposit_request(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDepositRequestRequest>,
        ) -> Result<tonic::Response<super::QueryDepositRequestResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/DepositRequest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** WithdrawRequests returns all withdraw requests.
        */
        pub async fn withdraw_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWithdrawRequestsRequest>,
        ) -> Result<tonic::Response<super::QueryWithdrawRequestsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/WithdrawRequests",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** WithdrawRequest returns the specific withdraw request.
        */
        pub async fn withdraw_request(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWithdrawRequestRequest>,
        ) -> Result<tonic::Response<super::QueryWithdrawRequestResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/WithdrawRequest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Orders returns all orders within the pair.
        */
        pub async fn orders(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOrdersRequest>,
        ) -> Result<tonic::Response<super::QueryOrdersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Query/Orders");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Order returns the specific order.
        */
        pub async fn order(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOrderRequest>,
        ) -> Result<tonic::Response<super::QueryOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Query/Order");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** OrdersByOrderer returns orders made by an orderer.
        */
        pub async fn orders_by_orderer(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOrdersByOrdererRequest>,
        ) -> Result<tonic::Response<super::QueryOrdersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/OrdersByOrderer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Farmer returns deposited poolcoin for farming .
        */
        pub async fn farmer(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFarmerRequest>,
        ) -> Result<tonic::Response<super::QueryFarmerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Query/Farmer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** DeserializePoolCoin splits poolcoin into the actual provided pool assets .
        */
        pub async fn deserialize_pool_coin(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDeserializePoolCoinRequest>,
        ) -> Result<tonic::Response<super::QueryDeserializePoolCoinResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/DeserializePoolCoin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** PoolIncentives provides insights about available pool incentives.
        */
        pub async fn pool_incentives(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolsIncentivesRequest>,
        ) -> Result<tonic::Response<super::QueryPoolIncentivesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/PoolIncentives",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** FarmedPoolCoin returns the total farmed pool coins.
        */
        pub async fn farmed_pool_coin(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFarmedPoolCoinRequest>,
        ) -> Result<tonic::Response<super::QueryFarmedPoolCoinResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/FarmedPoolCoin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** TotalActiveAndQueuedPoolCoin returns the total number of active and queued farmed pool coins in each pool.
        */
        pub async fn total_active_and_queued_pool_coin(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllFarmedPoolCoinsRequest>,
        ) -> Result<tonic::Response<super::QueryAllFarmedPoolCoinsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Query/TotalActiveAndQueuedPoolCoin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn order_books(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOrderBooksRequest>,
        ) -> Result<tonic::Response<super::QueryOrderBooksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Query/OrderBooks");
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
        /** Params returns parameters of the module.
        */
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        /** GenericParams returns app parameters of the module.
        */
        async fn generic_params(
            &self,
            request: tonic::Request<super::QueryGenericParamsRequest>,
        ) -> Result<tonic::Response<super::QueryGenericParamsResponse>, tonic::Status>;
        /** Pools returns all liquidity pools.
        */
        async fn pools(
            &self,
            request: tonic::Request<super::QueryPoolsRequest>,
        ) -> Result<tonic::Response<super::QueryPoolsResponse>, tonic::Status>;
        /** Pool returns the specific liquidity pool.
        */
        async fn pool(
            &self,
            request: tonic::Request<super::QueryPoolRequest>,
        ) -> Result<tonic::Response<super::QueryPoolResponse>, tonic::Status>;
        /** PoolByReserveAddress returns all pools that correspond to the reserve account.
        */
        async fn pool_by_reserve_address(
            &self,
            request: tonic::Request<super::QueryPoolByReserveAddressRequest>,
        ) -> Result<tonic::Response<super::QueryPoolResponse>, tonic::Status>;
        /** PoolByPoolCoinDenom returns all pools that correspond to the pool coin denom.
        */
        async fn pool_by_pool_coin_denom(
            &self,
            request: tonic::Request<super::QueryPoolByPoolCoinDenomRequest>,
        ) -> Result<tonic::Response<super::QueryPoolResponse>, tonic::Status>;
        /** Pairs returns all liquidity pairs.
        */
        async fn pairs(
            &self,
            request: tonic::Request<super::QueryPairsRequest>,
        ) -> Result<tonic::Response<super::QueryPairsResponse>, tonic::Status>;
        /** Pair returns the specific pair.
        */
        async fn pair(
            &self,
            request: tonic::Request<super::QueryPairRequest>,
        ) -> Result<tonic::Response<super::QueryPairResponse>, tonic::Status>;
        /** DepositRequests returns all deposit requests.
        */
        async fn deposit_requests(
            &self,
            request: tonic::Request<super::QueryDepositRequestsRequest>,
        ) -> Result<tonic::Response<super::QueryDepositRequestsResponse>, tonic::Status>;
        /** DepositRequest returns the specific deposit request.
        */
        async fn deposit_request(
            &self,
            request: tonic::Request<super::QueryDepositRequestRequest>,
        ) -> Result<tonic::Response<super::QueryDepositRequestResponse>, tonic::Status>;
        /** WithdrawRequests returns all withdraw requests.
        */
        async fn withdraw_requests(
            &self,
            request: tonic::Request<super::QueryWithdrawRequestsRequest>,
        ) -> Result<tonic::Response<super::QueryWithdrawRequestsResponse>, tonic::Status>;
        /** WithdrawRequest returns the specific withdraw request.
        */
        async fn withdraw_request(
            &self,
            request: tonic::Request<super::QueryWithdrawRequestRequest>,
        ) -> Result<tonic::Response<super::QueryWithdrawRequestResponse>, tonic::Status>;
        /** Orders returns all orders within the pair.
        */
        async fn orders(
            &self,
            request: tonic::Request<super::QueryOrdersRequest>,
        ) -> Result<tonic::Response<super::QueryOrdersResponse>, tonic::Status>;
        /** Order returns the specific order.
        */
        async fn order(
            &self,
            request: tonic::Request<super::QueryOrderRequest>,
        ) -> Result<tonic::Response<super::QueryOrderResponse>, tonic::Status>;
        /** OrdersByOrderer returns orders made by an orderer.
        */
        async fn orders_by_orderer(
            &self,
            request: tonic::Request<super::QueryOrdersByOrdererRequest>,
        ) -> Result<tonic::Response<super::QueryOrdersResponse>, tonic::Status>;
        /** Farmer returns deposited poolcoin for farming .
        */
        async fn farmer(
            &self,
            request: tonic::Request<super::QueryFarmerRequest>,
        ) -> Result<tonic::Response<super::QueryFarmerResponse>, tonic::Status>;
        /** DeserializePoolCoin splits poolcoin into the actual provided pool assets .
        */
        async fn deserialize_pool_coin(
            &self,
            request: tonic::Request<super::QueryDeserializePoolCoinRequest>,
        ) -> Result<tonic::Response<super::QueryDeserializePoolCoinResponse>, tonic::Status>;
        /** PoolIncentives provides insights about available pool incentives.
        */
        async fn pool_incentives(
            &self,
            request: tonic::Request<super::QueryPoolsIncentivesRequest>,
        ) -> Result<tonic::Response<super::QueryPoolIncentivesResponse>, tonic::Status>;
        /** FarmedPoolCoin returns the total farmed pool coins.
        */
        async fn farmed_pool_coin(
            &self,
            request: tonic::Request<super::QueryFarmedPoolCoinRequest>,
        ) -> Result<tonic::Response<super::QueryFarmedPoolCoinResponse>, tonic::Status>;
        /** TotalActiveAndQueuedPoolCoin returns the total number of active and queued farmed pool coins in each pool.
        */
        async fn total_active_and_queued_pool_coin(
            &self,
            request: tonic::Request<super::QueryAllFarmedPoolCoinsRequest>,
        ) -> Result<tonic::Response<super::QueryAllFarmedPoolCoinsResponse>, tonic::Status>;
        ///
        async fn order_books(
            &self,
            request: tonic::Request<super::QueryOrderBooksRequest>,
        ) -> Result<tonic::Response<super::QueryOrderBooksResponse>, tonic::Status>;
    }
    /** Query defines the gRPC querier service.
    */
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
                "/comdex.liquidity.v1beta1.Query/Params" => {
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
                "/comdex.liquidity.v1beta1.Query/GenericParams" => {
                    #[allow(non_camel_case_types)]
                    struct GenericParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGenericParamsRequest>
                        for GenericParamsSvc<T>
                    {
                        type Response = super::QueryGenericParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGenericParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).generic_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GenericParamsSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/Pools" => {
                    #[allow(non_camel_case_types)]
                    struct PoolsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPoolsRequest> for PoolsSvc<T> {
                        type Response = super::QueryPoolsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pools(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PoolsSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/Pool" => {
                    #[allow(non_camel_case_types)]
                    struct PoolSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPoolRequest> for PoolSvc<T> {
                        type Response = super::QueryPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PoolSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/PoolByReserveAddress" => {
                    #[allow(non_camel_case_types)]
                    struct PoolByReserveAddressSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryPoolByReserveAddressRequest>
                        for PoolByReserveAddressSvc<T>
                    {
                        type Response = super::QueryPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolByReserveAddressRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).pool_by_reserve_address(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PoolByReserveAddressSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/PoolByPoolCoinDenom" => {
                    #[allow(non_camel_case_types)]
                    struct PoolByPoolCoinDenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryPoolByPoolCoinDenomRequest>
                        for PoolByPoolCoinDenomSvc<T>
                    {
                        type Response = super::QueryPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolByPoolCoinDenomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).pool_by_pool_coin_denom(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PoolByPoolCoinDenomSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/Pairs" => {
                    #[allow(non_camel_case_types)]
                    struct PairsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPairsRequest> for PairsSvc<T> {
                        type Response = super::QueryPairsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPairsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pairs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PairsSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/Pair" => {
                    #[allow(non_camel_case_types)]
                    struct PairSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPairRequest> for PairSvc<T> {
                        type Response = super::QueryPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPairRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pair(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PairSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/DepositRequests" => {
                    #[allow(non_camel_case_types)]
                    struct DepositRequestsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDepositRequestsRequest>
                        for DepositRequestsSvc<T>
                    {
                        type Response = super::QueryDepositRequestsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDepositRequestsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).deposit_requests(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DepositRequestsSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/DepositRequest" => {
                    #[allow(non_camel_case_types)]
                    struct DepositRequestSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDepositRequestRequest>
                        for DepositRequestSvc<T>
                    {
                        type Response = super::QueryDepositRequestResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDepositRequestRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).deposit_request(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DepositRequestSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/WithdrawRequests" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawRequestsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryWithdrawRequestsRequest>
                        for WithdrawRequestsSvc<T>
                    {
                        type Response = super::QueryWithdrawRequestsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWithdrawRequestsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).withdraw_requests(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawRequestsSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/WithdrawRequest" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawRequestSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryWithdrawRequestRequest>
                        for WithdrawRequestSvc<T>
                    {
                        type Response = super::QueryWithdrawRequestResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWithdrawRequestRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).withdraw_request(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawRequestSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/Orders" => {
                    #[allow(non_camel_case_types)]
                    struct OrdersSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryOrdersRequest> for OrdersSvc<T> {
                        type Response = super::QueryOrdersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryOrdersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).orders(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrdersSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/Order" => {
                    #[allow(non_camel_case_types)]
                    struct OrderSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryOrderRequest> for OrderSvc<T> {
                        type Response = super::QueryOrderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryOrderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/OrdersByOrderer" => {
                    #[allow(non_camel_case_types)]
                    struct OrdersByOrdererSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryOrdersByOrdererRequest>
                        for OrdersByOrdererSvc<T>
                    {
                        type Response = super::QueryOrdersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryOrdersByOrdererRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).orders_by_orderer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrdersByOrdererSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/Farmer" => {
                    #[allow(non_camel_case_types)]
                    struct FarmerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryFarmerRequest> for FarmerSvc<T> {
                        type Response = super::QueryFarmerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFarmerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).farmer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FarmerSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/DeserializePoolCoin" => {
                    #[allow(non_camel_case_types)]
                    struct DeserializePoolCoinSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryDeserializePoolCoinRequest>
                        for DeserializePoolCoinSvc<T>
                    {
                        type Response = super::QueryDeserializePoolCoinResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDeserializePoolCoinRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).deserialize_pool_coin(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeserializePoolCoinSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/PoolIncentives" => {
                    #[allow(non_camel_case_types)]
                    struct PoolIncentivesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPoolsIncentivesRequest>
                        for PoolIncentivesSvc<T>
                    {
                        type Response = super::QueryPoolIncentivesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolsIncentivesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pool_incentives(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PoolIncentivesSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/FarmedPoolCoin" => {
                    #[allow(non_camel_case_types)]
                    struct FarmedPoolCoinSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryFarmedPoolCoinRequest>
                        for FarmedPoolCoinSvc<T>
                    {
                        type Response = super::QueryFarmedPoolCoinResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFarmedPoolCoinRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).farmed_pool_coin(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FarmedPoolCoinSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/TotalActiveAndQueuedPoolCoin" => {
                    #[allow(non_camel_case_types)]
                    struct TotalActiveAndQueuedPoolCoinSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAllFarmedPoolCoinsRequest>
                        for TotalActiveAndQueuedPoolCoinSvc<T>
                    {
                        type Response = super::QueryAllFarmedPoolCoinsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllFarmedPoolCoinsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).total_active_and_queued_pool_coin(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TotalActiveAndQueuedPoolCoinSvc(inner);
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
                "/comdex.liquidity.v1beta1.Query/OrderBooks" => {
                    #[allow(non_camel_case_types)]
                    struct OrderBooksSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryOrderBooksRequest> for OrderBooksSvc<T> {
                        type Response = super::QueryOrderBooksResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryOrderBooksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).order_books(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderBooksSvc(inner);
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
        const NAME: &'static str = "comdex.liquidity.v1beta1.Query";
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
        pub async fn create_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreatePair>,
        ) -> Result<tonic::Response<super::MsgCreatePairResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/CreatePair");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreatePool>,
        ) -> Result<tonic::Response<super::MsgCreatePoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/CreatePool");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_ranged_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateRangedPool>,
        ) -> Result<tonic::Response<super::MsgCreateRangedPoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Msg/CreateRangedPool",
            );
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
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/Deposit");
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
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/Withdraw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn limit_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLimitOrder>,
        ) -> Result<tonic::Response<super::MsgLimitOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/LimitOrder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn market_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMarketOrder>,
        ) -> Result<tonic::Response<super::MsgMarketOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/MarketOrder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn mm_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMmOrder>,
        ) -> Result<tonic::Response<super::MsgMmOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/MMOrder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelOrder>,
        ) -> Result<tonic::Response<super::MsgCancelOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/CancelOrder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cancel_all_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelAllOrders>,
        ) -> Result<tonic::Response<super::MsgCancelAllOrdersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.liquidity.v1beta1.Msg/CancelAllOrders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cancel_mm_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelMmOrder>,
        ) -> Result<tonic::Response<super::MsgCancelMmOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/CancelMMOrder");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn farm(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFarm>,
        ) -> Result<tonic::Response<super::MsgFarmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/Farm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn unfarm(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnfarm>,
        ) -> Result<tonic::Response<super::MsgUnfarmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.liquidity.v1beta1.Msg/Unfarm");
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
        async fn create_pair(
            &self,
            request: tonic::Request<super::MsgCreatePair>,
        ) -> Result<tonic::Response<super::MsgCreatePairResponse>, tonic::Status>;
        async fn create_pool(
            &self,
            request: tonic::Request<super::MsgCreatePool>,
        ) -> Result<tonic::Response<super::MsgCreatePoolResponse>, tonic::Status>;
        async fn create_ranged_pool(
            &self,
            request: tonic::Request<super::MsgCreateRangedPool>,
        ) -> Result<tonic::Response<super::MsgCreateRangedPoolResponse>, tonic::Status>;
        async fn deposit(
            &self,
            request: tonic::Request<super::MsgDeposit>,
        ) -> Result<tonic::Response<super::MsgDepositResponse>, tonic::Status>;
        async fn withdraw(
            &self,
            request: tonic::Request<super::MsgWithdraw>,
        ) -> Result<tonic::Response<super::MsgWithdrawResponse>, tonic::Status>;
        async fn limit_order(
            &self,
            request: tonic::Request<super::MsgLimitOrder>,
        ) -> Result<tonic::Response<super::MsgLimitOrderResponse>, tonic::Status>;
        async fn market_order(
            &self,
            request: tonic::Request<super::MsgMarketOrder>,
        ) -> Result<tonic::Response<super::MsgMarketOrderResponse>, tonic::Status>;
        async fn mm_order(
            &self,
            request: tonic::Request<super::MsgMmOrder>,
        ) -> Result<tonic::Response<super::MsgMmOrderResponse>, tonic::Status>;
        async fn cancel_order(
            &self,
            request: tonic::Request<super::MsgCancelOrder>,
        ) -> Result<tonic::Response<super::MsgCancelOrderResponse>, tonic::Status>;
        async fn cancel_all_orders(
            &self,
            request: tonic::Request<super::MsgCancelAllOrders>,
        ) -> Result<tonic::Response<super::MsgCancelAllOrdersResponse>, tonic::Status>;
        async fn cancel_mm_order(
            &self,
            request: tonic::Request<super::MsgCancelMmOrder>,
        ) -> Result<tonic::Response<super::MsgCancelMmOrderResponse>, tonic::Status>;
        async fn farm(
            &self,
            request: tonic::Request<super::MsgFarm>,
        ) -> Result<tonic::Response<super::MsgFarmResponse>, tonic::Status>;
        async fn unfarm(
            &self,
            request: tonic::Request<super::MsgUnfarm>,
        ) -> Result<tonic::Response<super::MsgUnfarmResponse>, tonic::Status>;
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
                "/comdex.liquidity.v1beta1.Msg/CreatePair" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePairSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreatePair> for CreatePairSvc<T> {
                        type Response = super::MsgCreatePairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreatePair>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_pair(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreatePairSvc(inner);
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
                "/comdex.liquidity.v1beta1.Msg/CreatePool" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePoolSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreatePool> for CreatePoolSvc<T> {
                        type Response = super::MsgCreatePoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreatePool>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreatePoolSvc(inner);
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
                "/comdex.liquidity.v1beta1.Msg/CreateRangedPool" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRangedPoolSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateRangedPool> for CreateRangedPoolSvc<T> {
                        type Response = super::MsgCreateRangedPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateRangedPool>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_ranged_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateRangedPoolSvc(inner);
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
                "/comdex.liquidity.v1beta1.Msg/Deposit" => {
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
                "/comdex.liquidity.v1beta1.Msg/Withdraw" => {
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
                "/comdex.liquidity.v1beta1.Msg/LimitOrder" => {
                    #[allow(non_camel_case_types)]
                    struct LimitOrderSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgLimitOrder> for LimitOrderSvc<T> {
                        type Response = super::MsgLimitOrderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgLimitOrder>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).limit_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LimitOrderSvc(inner);
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
                "/comdex.liquidity.v1beta1.Msg/MarketOrder" => {
                    #[allow(non_camel_case_types)]
                    struct MarketOrderSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMarketOrder> for MarketOrderSvc<T> {
                        type Response = super::MsgMarketOrderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMarketOrder>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).market_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarketOrderSvc(inner);
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
                "/comdex.liquidity.v1beta1.Msg/MMOrder" => {
                    #[allow(non_camel_case_types)]
                    struct MMOrderSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMmOrder> for MMOrderSvc<T> {
                        type Response = super::MsgMmOrderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMmOrder>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mm_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MMOrderSvc(inner);
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
                "/comdex.liquidity.v1beta1.Msg/CancelOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CancelOrderSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCancelOrder> for CancelOrderSvc<T> {
                        type Response = super::MsgCancelOrderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelOrder>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelOrderSvc(inner);
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
                "/comdex.liquidity.v1beta1.Msg/CancelAllOrders" => {
                    #[allow(non_camel_case_types)]
                    struct CancelAllOrdersSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCancelAllOrders> for CancelAllOrdersSvc<T> {
                        type Response = super::MsgCancelAllOrdersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelAllOrders>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel_all_orders(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelAllOrdersSvc(inner);
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
                "/comdex.liquidity.v1beta1.Msg/CancelMMOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CancelMMOrderSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCancelMmOrder> for CancelMMOrderSvc<T> {
                        type Response = super::MsgCancelMmOrderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelMmOrder>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel_mm_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelMMOrderSvc(inner);
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
                "/comdex.liquidity.v1beta1.Msg/Farm" => {
                    #[allow(non_camel_case_types)]
                    struct FarmSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgFarm> for FarmSvc<T> {
                        type Response = super::MsgFarmResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgFarm>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).farm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FarmSvc(inner);
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
                "/comdex.liquidity.v1beta1.Msg/Unfarm" => {
                    #[allow(non_camel_case_types)]
                    struct UnfarmSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUnfarm> for UnfarmSvc<T> {
                        type Response = super::MsgUnfarmResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUnfarm>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).unfarm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnfarmSvc(inner);
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
        const NAME: &'static str = "comdex.liquidity.v1beta1.Msg";
    }
}
