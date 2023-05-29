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
        pub async fn query_surplus_auction(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySurplusAuctionRequest>,
        ) -> Result<tonic::Response<super::QuerySurplusAuctionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QuerySurplusAuction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_surplus_auctions(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySurplusAuctionsRequest>,
        ) -> Result<tonic::Response<super::QuerySurplusAuctionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QuerySurplusAuctions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_surplus_biddings(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySurplusBiddingsRequest>,
        ) -> Result<tonic::Response<super::QuerySurplusBiddingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QuerySurplusBiddings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_debt_auction(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDebtAuctionRequest>,
        ) -> Result<tonic::Response<super::QueryDebtAuctionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryDebtAuction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_debt_auctions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDebtAuctionsRequest>,
        ) -> Result<tonic::Response<super::QueryDebtAuctionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryDebtAuctions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_debt_biddings(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDebtBiddingsRequest>,
        ) -> Result<tonic::Response<super::QueryDebtBiddingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryDebtBiddings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_dutch_auction(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDutchAuctionRequest>,
        ) -> Result<tonic::Response<super::QueryDutchAuctionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryDutchAuction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_dutch_auctions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDutchAuctionsRequest>,
        ) -> Result<tonic::Response<super::QueryDutchAuctionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryDutchAuctions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_dutch_biddings(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDutchBiddingsRequest>,
        ) -> Result<tonic::Response<super::QueryDutchBiddingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryDutchBiddings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_protocol_statistics(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryProtocolStatisticsRequest>,
        ) -> Result<tonic::Response<super::QueryProtocolStatisticsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryProtocolStatistics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_generic_auction_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGenericAuctionParamRequest>,
        ) -> Result<tonic::Response<super::QueryGenericAuctionParamResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryGenericAuctionParams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_dutch_lend_auction(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDutchLendAuctionRequest>,
        ) -> Result<tonic::Response<super::QueryDutchLendAuctionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryDutchLendAuction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_dutch_lend_auctions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDutchLendAuctionsRequest>,
        ) -> Result<tonic::Response<super::QueryDutchLendAuctionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryDutchLendAuctions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_dutch_lend_biddings(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDutchLendBiddingsRequest>,
        ) -> Result<tonic::Response<super::QueryDutchLendBiddingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryDutchLendBiddings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_filter_dutch_auctions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFilterDutchAuctionsRequest>,
        ) -> Result<tonic::Response<super::QueryFilterDutchAuctionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Query/QueryFilterDutchAuctions",
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
        async fn query_surplus_auction(
            &self,
            request: tonic::Request<super::QuerySurplusAuctionRequest>,
        ) -> Result<tonic::Response<super::QuerySurplusAuctionResponse>, tonic::Status>;
        ///
        async fn query_surplus_auctions(
            &self,
            request: tonic::Request<super::QuerySurplusAuctionsRequest>,
        ) -> Result<tonic::Response<super::QuerySurplusAuctionsResponse>, tonic::Status>;
        ///
        async fn query_surplus_biddings(
            &self,
            request: tonic::Request<super::QuerySurplusBiddingsRequest>,
        ) -> Result<tonic::Response<super::QuerySurplusBiddingsResponse>, tonic::Status>;
        ///
        async fn query_debt_auction(
            &self,
            request: tonic::Request<super::QueryDebtAuctionRequest>,
        ) -> Result<tonic::Response<super::QueryDebtAuctionResponse>, tonic::Status>;
        ///
        async fn query_debt_auctions(
            &self,
            request: tonic::Request<super::QueryDebtAuctionsRequest>,
        ) -> Result<tonic::Response<super::QueryDebtAuctionsResponse>, tonic::Status>;
        ///
        async fn query_debt_biddings(
            &self,
            request: tonic::Request<super::QueryDebtBiddingsRequest>,
        ) -> Result<tonic::Response<super::QueryDebtBiddingsResponse>, tonic::Status>;
        ///
        async fn query_dutch_auction(
            &self,
            request: tonic::Request<super::QueryDutchAuctionRequest>,
        ) -> Result<tonic::Response<super::QueryDutchAuctionResponse>, tonic::Status>;
        ///
        async fn query_dutch_auctions(
            &self,
            request: tonic::Request<super::QueryDutchAuctionsRequest>,
        ) -> Result<tonic::Response<super::QueryDutchAuctionsResponse>, tonic::Status>;
        ///
        async fn query_dutch_biddings(
            &self,
            request: tonic::Request<super::QueryDutchBiddingsRequest>,
        ) -> Result<tonic::Response<super::QueryDutchBiddingsResponse>, tonic::Status>;
        ///
        async fn query_protocol_statistics(
            &self,
            request: tonic::Request<super::QueryProtocolStatisticsRequest>,
        ) -> Result<tonic::Response<super::QueryProtocolStatisticsResponse>, tonic::Status>;
        ///
        async fn query_generic_auction_params(
            &self,
            request: tonic::Request<super::QueryGenericAuctionParamRequest>,
        ) -> Result<tonic::Response<super::QueryGenericAuctionParamResponse>, tonic::Status>;
        ///
        async fn query_dutch_lend_auction(
            &self,
            request: tonic::Request<super::QueryDutchLendAuctionRequest>,
        ) -> Result<tonic::Response<super::QueryDutchLendAuctionResponse>, tonic::Status>;
        ///
        async fn query_dutch_lend_auctions(
            &self,
            request: tonic::Request<super::QueryDutchLendAuctionsRequest>,
        ) -> Result<tonic::Response<super::QueryDutchLendAuctionsResponse>, tonic::Status>;
        ///
        async fn query_dutch_lend_biddings(
            &self,
            request: tonic::Request<super::QueryDutchLendBiddingsRequest>,
        ) -> Result<tonic::Response<super::QueryDutchLendBiddingsResponse>, tonic::Status>;
        ///
        async fn query_filter_dutch_auctions(
            &self,
            request: tonic::Request<super::QueryFilterDutchAuctionsRequest>,
        ) -> Result<tonic::Response<super::QueryFilterDutchAuctionsResponse>, tonic::Status>;
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
                "/comdex.auction.v1beta1.Query/QuerySurplusAuction" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySurplusAuctionSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QuerySurplusAuctionRequest>
                        for QuerySurplusAuctionSvc<T>
                    {
                        type Response = super::QuerySurplusAuctionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySurplusAuctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_surplus_auction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuerySurplusAuctionSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QuerySurplusAuctions" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySurplusAuctionsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QuerySurplusAuctionsRequest>
                        for QuerySurplusAuctionsSvc<T>
                    {
                        type Response = super::QuerySurplusAuctionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySurplusAuctionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_surplus_auctions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuerySurplusAuctionsSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QuerySurplusBiddings" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySurplusBiddingsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QuerySurplusBiddingsRequest>
                        for QuerySurplusBiddingsSvc<T>
                    {
                        type Response = super::QuerySurplusBiddingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySurplusBiddingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_surplus_biddings(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuerySurplusBiddingsSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryDebtAuction" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDebtAuctionSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDebtAuctionRequest>
                        for QueryDebtAuctionSvc<T>
                    {
                        type Response = super::QueryDebtAuctionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDebtAuctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_debt_auction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDebtAuctionSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryDebtAuctions" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDebtAuctionsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDebtAuctionsRequest>
                        for QueryDebtAuctionsSvc<T>
                    {
                        type Response = super::QueryDebtAuctionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDebtAuctionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_debt_auctions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDebtAuctionsSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryDebtBiddings" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDebtBiddingsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDebtBiddingsRequest>
                        for QueryDebtBiddingsSvc<T>
                    {
                        type Response = super::QueryDebtBiddingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDebtBiddingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_debt_biddings(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDebtBiddingsSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryDutchAuction" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDutchAuctionSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDutchAuctionRequest>
                        for QueryDutchAuctionSvc<T>
                    {
                        type Response = super::QueryDutchAuctionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDutchAuctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_dutch_auction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDutchAuctionSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryDutchAuctions" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDutchAuctionsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDutchAuctionsRequest>
                        for QueryDutchAuctionsSvc<T>
                    {
                        type Response = super::QueryDutchAuctionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDutchAuctionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_dutch_auctions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDutchAuctionsSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryDutchBiddings" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDutchBiddingsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDutchBiddingsRequest>
                        for QueryDutchBiddingsSvc<T>
                    {
                        type Response = super::QueryDutchBiddingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDutchBiddingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_dutch_biddings(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDutchBiddingsSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryProtocolStatistics" => {
                    #[allow(non_camel_case_types)]
                    struct QueryProtocolStatisticsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryProtocolStatisticsRequest>
                        for QueryProtocolStatisticsSvc<T>
                    {
                        type Response = super::QueryProtocolStatisticsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryProtocolStatisticsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_protocol_statistics(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryProtocolStatisticsSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryGenericAuctionParams" => {
                    #[allow(non_camel_case_types)]
                    struct QueryGenericAuctionParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGenericAuctionParamRequest>
                        for QueryGenericAuctionParamsSvc<T>
                    {
                        type Response = super::QueryGenericAuctionParamResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGenericAuctionParamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_generic_auction_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryGenericAuctionParamsSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryDutchLendAuction" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDutchLendAuctionSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDutchLendAuctionRequest>
                        for QueryDutchLendAuctionSvc<T>
                    {
                        type Response = super::QueryDutchLendAuctionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDutchLendAuctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_dutch_lend_auction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDutchLendAuctionSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryDutchLendAuctions" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDutchLendAuctionsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDutchLendAuctionsRequest>
                        for QueryDutchLendAuctionsSvc<T>
                    {
                        type Response = super::QueryDutchLendAuctionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDutchLendAuctionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_dutch_lend_auctions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDutchLendAuctionsSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryDutchLendBiddings" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDutchLendBiddingsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDutchLendBiddingsRequest>
                        for QueryDutchLendBiddingsSvc<T>
                    {
                        type Response = super::QueryDutchLendBiddingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDutchLendBiddingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_dutch_lend_biddings(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDutchLendBiddingsSvc(inner);
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
                "/comdex.auction.v1beta1.Query/QueryFilterDutchAuctions" => {
                    #[allow(non_camel_case_types)]
                    struct QueryFilterDutchAuctionsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryFilterDutchAuctionsRequest>
                        for QueryFilterDutchAuctionsSvc<T>
                    {
                        type Response = super::QueryFilterDutchAuctionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFilterDutchAuctionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_filter_dutch_auctions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryFilterDutchAuctionsSvc(inner);
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
        const NAME: &'static str = "comdex.auction.v1beta1.Query";
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
        pub async fn msg_place_surplus_bid(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPlaceSurplusBidRequest>,
        ) -> Result<tonic::Response<super::MsgPlaceSurplusBidResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Msg/MsgPlaceSurplusBid",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_place_debt_bid(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPlaceDebtBidRequest>,
        ) -> Result<tonic::Response<super::MsgPlaceDebtBidResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.auction.v1beta1.Msg/MsgPlaceDebtBid");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_place_dutch_bid(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPlaceDutchBidRequest>,
        ) -> Result<tonic::Response<super::MsgPlaceDutchBidResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Msg/MsgPlaceDutchBid",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn msg_place_dutch_lend_bid(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPlaceDutchLendBidRequest>,
        ) -> Result<tonic::Response<super::MsgPlaceDutchLendBidResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.auction.v1beta1.Msg/MsgPlaceDutchLendBid",
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
        async fn msg_place_surplus_bid(
            &self,
            request: tonic::Request<super::MsgPlaceSurplusBidRequest>,
        ) -> Result<tonic::Response<super::MsgPlaceSurplusBidResponse>, tonic::Status>;
        ///
        async fn msg_place_debt_bid(
            &self,
            request: tonic::Request<super::MsgPlaceDebtBidRequest>,
        ) -> Result<tonic::Response<super::MsgPlaceDebtBidResponse>, tonic::Status>;
        ///
        async fn msg_place_dutch_bid(
            &self,
            request: tonic::Request<super::MsgPlaceDutchBidRequest>,
        ) -> Result<tonic::Response<super::MsgPlaceDutchBidResponse>, tonic::Status>;
        ///
        async fn msg_place_dutch_lend_bid(
            &self,
            request: tonic::Request<super::MsgPlaceDutchLendBidRequest>,
        ) -> Result<tonic::Response<super::MsgPlaceDutchLendBidResponse>, tonic::Status>;
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
                "/comdex.auction.v1beta1.Msg/MsgPlaceSurplusBid" => {
                    #[allow(non_camel_case_types)]
                    struct MsgPlaceSurplusBidSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgPlaceSurplusBidRequest>
                        for MsgPlaceSurplusBidSvc<T>
                    {
                        type Response = super::MsgPlaceSurplusBidResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgPlaceSurplusBidRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_place_surplus_bid(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgPlaceSurplusBidSvc(inner);
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
                "/comdex.auction.v1beta1.Msg/MsgPlaceDebtBid" => {
                    #[allow(non_camel_case_types)]
                    struct MsgPlaceDebtBidSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgPlaceDebtBidRequest> for MsgPlaceDebtBidSvc<T> {
                        type Response = super::MsgPlaceDebtBidResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgPlaceDebtBidRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_place_debt_bid(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgPlaceDebtBidSvc(inner);
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
                "/comdex.auction.v1beta1.Msg/MsgPlaceDutchBid" => {
                    #[allow(non_camel_case_types)]
                    struct MsgPlaceDutchBidSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgPlaceDutchBidRequest>
                        for MsgPlaceDutchBidSvc<T>
                    {
                        type Response = super::MsgPlaceDutchBidResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgPlaceDutchBidRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_place_dutch_bid(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgPlaceDutchBidSvc(inner);
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
                "/comdex.auction.v1beta1.Msg/MsgPlaceDutchLendBid" => {
                    #[allow(non_camel_case_types)]
                    struct MsgPlaceDutchLendBidSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgPlaceDutchLendBidRequest>
                        for MsgPlaceDutchLendBidSvc<T>
                    {
                        type Response = super::MsgPlaceDutchLendBidResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgPlaceDutchLendBidRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).msg_place_dutch_lend_bid(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgPlaceDutchLendBidSvc(inner);
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
        const NAME: &'static str = "comdex.auction.v1beta1.Msg";
    }
}
