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
        pub async fn query_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAssetsRequest>,
        ) -> Result<tonic::Response<super::QueryAssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.asset.v1beta1.Query/QueryAssets");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAssetRequest>,
        ) -> Result<tonic::Response<super::QueryAssetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.asset.v1beta1.Query/QueryAsset");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_asset_pairs(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAssetPairsRequest>,
        ) -> Result<tonic::Response<super::QueryAssetPairsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.asset.v1beta1.Query/QueryAssetPairs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_asset_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAssetPairRequest>,
        ) -> Result<tonic::Response<super::QueryAssetPairResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.asset.v1beta1.Query/QueryAssetPair");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_apps(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAppsRequest>,
        ) -> Result<tonic::Response<super::QueryAppsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.asset.v1beta1.Query/QueryApps");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAppRequest>,
        ) -> Result<tonic::Response<super::QueryAppResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/comdex.asset.v1beta1.Query/QueryApp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_extended_pair_vault(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExtendedPairVaultRequest>,
        ) -> Result<tonic::Response<super::QueryExtendedPairVaultResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.asset.v1beta1.Query/QueryExtendedPairVault",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_extended_pair_vaults(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllExtendedPairVaultsRequest>,
        ) -> Result<tonic::Response<super::QueryAllExtendedPairVaultsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.asset.v1beta1.Query/QueryAllExtendedPairVaults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_extended_pair_vaults_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllExtendedPairVaultsByAppRequest>,
        ) -> Result<tonic::Response<super::QueryAllExtendedPairVaultsByAppResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.asset.v1beta1.Query/QueryAllExtendedPairVaultsByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_extended_pair_stable_vaults_id_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllExtendedPairStableVaultsIdByAppRequest>,
        ) -> Result<
            tonic::Response<super::QueryAllExtendedPairStableVaultsIdByAppResponse>,
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
                "/comdex.asset.v1beta1.Query/QueryAllExtendedPairStableVaultsIDByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_gov_token_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGovTokenByAppRequest>,
        ) -> Result<tonic::Response<super::QueryGovTokenByAppResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.asset.v1beta1.Query/QueryGovTokenByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_all_extended_pair_stable_vaults_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllExtendedPairStableVaultsByAppRequest>,
        ) -> Result<
            tonic::Response<super::QueryAllExtendedPairStableVaultsByAppResponse>,
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
                "/comdex.asset.v1beta1.Query/QueryAllExtendedPairStableVaultsByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn query_extended_pair_vaults_by_app_without_stable(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExtendedPairVaultsByAppWithoutStableRequest>,
        ) -> Result<
            tonic::Response<super::QueryExtendedPairVaultsByAppWithoutStableResponse>,
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
                "/comdex.asset.v1beta1.Query/QueryExtendedPairVaultsByAppWithoutStable",
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
        async fn query_assets(
            &self,
            request: tonic::Request<super::QueryAssetsRequest>,
        ) -> Result<tonic::Response<super::QueryAssetsResponse>, tonic::Status>;
        ///
        async fn query_asset(
            &self,
            request: tonic::Request<super::QueryAssetRequest>,
        ) -> Result<tonic::Response<super::QueryAssetResponse>, tonic::Status>;
        ///
        async fn query_asset_pairs(
            &self,
            request: tonic::Request<super::QueryAssetPairsRequest>,
        ) -> Result<tonic::Response<super::QueryAssetPairsResponse>, tonic::Status>;
        ///
        async fn query_asset_pair(
            &self,
            request: tonic::Request<super::QueryAssetPairRequest>,
        ) -> Result<tonic::Response<super::QueryAssetPairResponse>, tonic::Status>;
        ///
        async fn query_apps(
            &self,
            request: tonic::Request<super::QueryAppsRequest>,
        ) -> Result<tonic::Response<super::QueryAppsResponse>, tonic::Status>;
        ///
        async fn query_app(
            &self,
            request: tonic::Request<super::QueryAppRequest>,
        ) -> Result<tonic::Response<super::QueryAppResponse>, tonic::Status>;
        ///
        async fn query_extended_pair_vault(
            &self,
            request: tonic::Request<super::QueryExtendedPairVaultRequest>,
        ) -> Result<tonic::Response<super::QueryExtendedPairVaultResponse>, tonic::Status>;
        ///
        async fn query_all_extended_pair_vaults(
            &self,
            request: tonic::Request<super::QueryAllExtendedPairVaultsRequest>,
        ) -> Result<tonic::Response<super::QueryAllExtendedPairVaultsResponse>, tonic::Status>;
        ///
        async fn query_all_extended_pair_vaults_by_app(
            &self,
            request: tonic::Request<super::QueryAllExtendedPairVaultsByAppRequest>,
        ) -> Result<tonic::Response<super::QueryAllExtendedPairVaultsByAppResponse>, tonic::Status>;
        ///
        async fn query_all_extended_pair_stable_vaults_id_by_app(
            &self,
            request: tonic::Request<super::QueryAllExtendedPairStableVaultsIdByAppRequest>,
        ) -> Result<
            tonic::Response<super::QueryAllExtendedPairStableVaultsIdByAppResponse>,
            tonic::Status,
        >;
        ///
        async fn query_gov_token_by_app(
            &self,
            request: tonic::Request<super::QueryGovTokenByAppRequest>,
        ) -> Result<tonic::Response<super::QueryGovTokenByAppResponse>, tonic::Status>;
        ///
        async fn query_all_extended_pair_stable_vaults_by_app(
            &self,
            request: tonic::Request<super::QueryAllExtendedPairStableVaultsByAppRequest>,
        ) -> Result<
            tonic::Response<super::QueryAllExtendedPairStableVaultsByAppResponse>,
            tonic::Status,
        >;
        ///
        async fn query_extended_pair_vaults_by_app_without_stable(
            &self,
            request: tonic::Request<super::QueryExtendedPairVaultsByAppWithoutStableRequest>,
        ) -> Result<
            tonic::Response<super::QueryExtendedPairVaultsByAppWithoutStableResponse>,
            tonic::Status,
        >;
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
                "/comdex.asset.v1beta1.Query/QueryAssets" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAssetsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAssetsRequest> for QueryAssetsSvc<T> {
                        type Response = super::QueryAssetsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAssetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_assets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAssetsSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryAsset" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAssetSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAssetRequest> for QueryAssetSvc<T> {
                        type Response = super::QueryAssetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAssetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_asset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAssetSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryAssetPairs" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAssetPairsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAssetPairsRequest>
                        for QueryAssetPairsSvc<T>
                    {
                        type Response = super::QueryAssetPairsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAssetPairsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_asset_pairs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAssetPairsSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryAssetPair" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAssetPairSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAssetPairRequest> for QueryAssetPairSvc<T> {
                        type Response = super::QueryAssetPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAssetPairRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_asset_pair(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAssetPairSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryApps" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAppsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAppsRequest> for QueryAppsSvc<T> {
                        type Response = super::QueryAppsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAppsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_apps(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAppsSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAppRequest> for QueryAppSvc<T> {
                        type Response = super::QueryAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_app(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAppSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryExtendedPairVault" => {
                    #[allow(non_camel_case_types)]
                    struct QueryExtendedPairVaultSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryExtendedPairVaultRequest>
                        for QueryExtendedPairVaultSvc<T>
                    {
                        type Response = super::QueryExtendedPairVaultResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryExtendedPairVaultRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_extended_pair_vault(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryExtendedPairVaultSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryAllExtendedPairVaults" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllExtendedPairVaultsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAllExtendedPairVaultsRequest>
                        for QueryAllExtendedPairVaultsSvc<T>
                    {
                        type Response = super::QueryAllExtendedPairVaultsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllExtendedPairVaultsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_all_extended_pair_vaults(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllExtendedPairVaultsSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryAllExtendedPairVaultsByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllExtendedPairVaultsByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAllExtendedPairVaultsByAppRequest>
                        for QueryAllExtendedPairVaultsByAppSvc<T>
                    {
                        type Response = super::QueryAllExtendedPairVaultsByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllExtendedPairVaultsByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_all_extended_pair_vaults_by_app(request)
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
                        let method = QueryAllExtendedPairVaultsByAppSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryAllExtendedPairStableVaultsIDByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllExtendedPairStableVaultsIDByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryAllExtendedPairStableVaultsIdByAppRequest,
                        > for QueryAllExtendedPairStableVaultsIDByAppSvc<T>
                    {
                        type Response = super::QueryAllExtendedPairStableVaultsIdByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryAllExtendedPairStableVaultsIdByAppRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_all_extended_pair_stable_vaults_id_by_app(request)
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
                        let method = QueryAllExtendedPairStableVaultsIDByAppSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryGovTokenByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryGovTokenByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGovTokenByAppRequest>
                        for QueryGovTokenByAppSvc<T>
                    {
                        type Response = super::QueryGovTokenByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGovTokenByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_gov_token_by_app(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryGovTokenByAppSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryAllExtendedPairStableVaultsByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllExtendedPairStableVaultsByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryAllExtendedPairStableVaultsByAppRequest,
                        > for QueryAllExtendedPairStableVaultsByAppSvc<T>
                    {
                        type Response = super::QueryAllExtendedPairStableVaultsByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryAllExtendedPairStableVaultsByAppRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_all_extended_pair_stable_vaults_by_app(request)
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
                        let method = QueryAllExtendedPairStableVaultsByAppSvc(inner);
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
                "/comdex.asset.v1beta1.Query/QueryExtendedPairVaultsByAppWithoutStable" => {
                    #[allow(non_camel_case_types)]
                    struct QueryExtendedPairVaultsByAppWithoutStableSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryExtendedPairVaultsByAppWithoutStableRequest,
                        > for QueryExtendedPairVaultsByAppWithoutStableSvc<T>
                    {
                        type Response = super::QueryExtendedPairVaultsByAppWithoutStableResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryExtendedPairVaultsByAppWithoutStableRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_extended_pair_vaults_by_app_without_stable(request)
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
                        let method = QueryExtendedPairVaultsByAppWithoutStableSvc(inner);
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
        const NAME: &'static str = "comdex.asset.v1beta1.Query";
    }
}
