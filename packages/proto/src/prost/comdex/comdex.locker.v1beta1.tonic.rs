// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
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
        pub async fn query_locker_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockerInfoRequest>,
        ) -> Result<tonic::Response<super::QueryLockerInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryLockerInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_lockers_by_app_to_asset_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockersByAppToAssetIdRequest>,
        ) -> Result<tonic::Response<super::QueryLockersByAppToAssetIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryLockersByAppToAssetID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_locker_info_by_app_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockerInfoByAppIdRequest>,
        ) -> Result<tonic::Response<super::QueryLockerInfoByAppIdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryLockerInfoByAppID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_total_deposit_by_app_and_asset_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTotalDepositByAppAndAssetIdRequest>,
        ) -> Result<tonic::Response<super::QueryTotalDepositByAppAndAssetIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryTotalDepositByAppAndAssetID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_owner_locker_by_app_i_dby_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOwnerLockerByAppIDbyOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryOwnerLockerByAppIDbyOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryOwnerLockerByAppIDbyOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_owner_locker_of_all_apps_by_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOwnerLockerOfAllAppsByOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryOwnerLockerOfAllAppsByOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryOwnerLockerOfAllAppsByOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_owner_tx_details_locker_of_app_by_owner_by_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOwnerTxDetailsLockerOfAppByOwnerByAssetRequest>,
        ) -> Result<
            tonic::Response<super::QueryOwnerTxDetailsLockerOfAppByOwnerByAssetResponse>,
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
                "/comdex.locker.v1beta1.Query/QueryOwnerTxDetailsLockerOfAppByOwnerByAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_owner_locker_by_app_to_asset_i_dby_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOwnerLockerByAppToAssetIDbyOwnerRequest>,
        ) -> Result<
            tonic::Response<super::QueryOwnerLockerByAppToAssetIDbyOwnerResponse>,
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
                "/comdex.locker.v1beta1.Query/QueryOwnerLockerByAppToAssetIDbyOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_locker_by_app_by_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockerByAppByOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryLockerByAppByOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryLockerByAppByOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_locker_count_by_app_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockerCountByAppIdRequest>,
        ) -> Result<tonic::Response<super::QueryLockerCountByAppIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryLockerCountByAppID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_locker_count_by_app_to_asset_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockerCountByAppToAssetIdRequest>,
        ) -> Result<tonic::Response<super::QueryLockerCountByAppToAssetIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryLockerCountByAppToAssetID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_white_listed_asset_i_ds_by_app_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWhiteListedAssetIDsByAppIdRequest>,
        ) -> Result<tonic::Response<super::QueryWhiteListedAssetIDsByAppIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryWhiteListedAssetIDsByAppID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_white_listed_asset_by_all_apps(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWhiteListedAssetByAllAppsRequest>,
        ) -> Result<tonic::Response<super::QueryWhiteListedAssetByAllAppsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryWhiteListedAssetByAllApps",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_locker_lookup_table_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockerLookupTableByAppRequest>,
        ) -> Result<tonic::Response<super::QueryLockerLookupTableByAppResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryLockerLookupTableByApp",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_locker_lookup_table_by_app_and_asset_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockerLookupTableByAppAndAssetIdRequest>,
        ) -> Result<
            tonic::Response<super::QueryLockerLookupTableByAppAndAssetIdResponse>,
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
                "/comdex.locker.v1beta1.Query/QueryLockerLookupTableByAppAndAssetID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_locker_total_rewards_by_asset_app_wise(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockerTotalRewardsByAssetAppWiseRequest>,
        ) -> Result<
            tonic::Response<super::QueryLockerTotalRewardsByAssetAppWiseResponse>,
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
                "/comdex.locker.v1beta1.Query/QueryLockerTotalRewardsByAssetAppWise",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_locker_total_deposited_by_app(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockerTotalDepositedByAppRequest>,
        ) -> Result<tonic::Response<super::QueryLockerTotalDepositedByAppResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Query/QueryLockerTotalDepositedByApp",
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
        async fn query_locker_info(
            &self,
            request: tonic::Request<super::QueryLockerInfoRequest>,
        ) -> Result<tonic::Response<super::QueryLockerInfoResponse>, tonic::Status>;
        async fn query_lockers_by_app_to_asset_id(
            &self,
            request: tonic::Request<super::QueryLockersByAppToAssetIdRequest>,
        ) -> Result<tonic::Response<super::QueryLockersByAppToAssetIdResponse>, tonic::Status>;
        async fn query_locker_info_by_app_id(
            &self,
            request: tonic::Request<super::QueryLockerInfoByAppIdRequest>,
        ) -> Result<tonic::Response<super::QueryLockerInfoByAppIdResponse>, tonic::Status>;
        async fn query_total_deposit_by_app_and_asset_id(
            &self,
            request: tonic::Request<super::QueryTotalDepositByAppAndAssetIdRequest>,
        ) -> Result<tonic::Response<super::QueryTotalDepositByAppAndAssetIdResponse>, tonic::Status>;
        async fn query_owner_locker_by_app_i_dby_owner(
            &self,
            request: tonic::Request<super::QueryOwnerLockerByAppIDbyOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryOwnerLockerByAppIDbyOwnerResponse>, tonic::Status>;
        async fn query_owner_locker_of_all_apps_by_owner(
            &self,
            request: tonic::Request<super::QueryOwnerLockerOfAllAppsByOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryOwnerLockerOfAllAppsByOwnerResponse>, tonic::Status>;
        async fn query_owner_tx_details_locker_of_app_by_owner_by_asset(
            &self,
            request: tonic::Request<super::QueryOwnerTxDetailsLockerOfAppByOwnerByAssetRequest>,
        ) -> Result<
            tonic::Response<super::QueryOwnerTxDetailsLockerOfAppByOwnerByAssetResponse>,
            tonic::Status,
        >;
        async fn query_owner_locker_by_app_to_asset_i_dby_owner(
            &self,
            request: tonic::Request<super::QueryOwnerLockerByAppToAssetIDbyOwnerRequest>,
        ) -> Result<
            tonic::Response<super::QueryOwnerLockerByAppToAssetIDbyOwnerResponse>,
            tonic::Status,
        >;
        async fn query_locker_by_app_by_owner(
            &self,
            request: tonic::Request<super::QueryLockerByAppByOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryLockerByAppByOwnerResponse>, tonic::Status>;
        async fn query_locker_count_by_app_id(
            &self,
            request: tonic::Request<super::QueryLockerCountByAppIdRequest>,
        ) -> Result<tonic::Response<super::QueryLockerCountByAppIdResponse>, tonic::Status>;
        async fn query_locker_count_by_app_to_asset_id(
            &self,
            request: tonic::Request<super::QueryLockerCountByAppToAssetIdRequest>,
        ) -> Result<tonic::Response<super::QueryLockerCountByAppToAssetIdResponse>, tonic::Status>;
        async fn query_white_listed_asset_i_ds_by_app_id(
            &self,
            request: tonic::Request<super::QueryWhiteListedAssetIDsByAppIdRequest>,
        ) -> Result<tonic::Response<super::QueryWhiteListedAssetIDsByAppIdResponse>, tonic::Status>;
        async fn query_white_listed_asset_by_all_apps(
            &self,
            request: tonic::Request<super::QueryWhiteListedAssetByAllAppsRequest>,
        ) -> Result<tonic::Response<super::QueryWhiteListedAssetByAllAppsResponse>, tonic::Status>;
        async fn query_locker_lookup_table_by_app(
            &self,
            request: tonic::Request<super::QueryLockerLookupTableByAppRequest>,
        ) -> Result<tonic::Response<super::QueryLockerLookupTableByAppResponse>, tonic::Status>;
        async fn query_locker_lookup_table_by_app_and_asset_id(
            &self,
            request: tonic::Request<super::QueryLockerLookupTableByAppAndAssetIdRequest>,
        ) -> Result<
            tonic::Response<super::QueryLockerLookupTableByAppAndAssetIdResponse>,
            tonic::Status,
        >;
        async fn query_locker_total_rewards_by_asset_app_wise(
            &self,
            request: tonic::Request<super::QueryLockerTotalRewardsByAssetAppWiseRequest>,
        ) -> Result<
            tonic::Response<super::QueryLockerTotalRewardsByAssetAppWiseResponse>,
            tonic::Status,
        >;
        async fn query_locker_total_deposited_by_app(
            &self,
            request: tonic::Request<super::QueryLockerTotalDepositedByAppRequest>,
        ) -> Result<tonic::Response<super::QueryLockerTotalDepositedByAppResponse>, tonic::Status>;
    }
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
                "/comdex.locker.v1beta1.Query/QueryLockerInfo" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLockerInfoSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryLockerInfoRequest>
                        for QueryLockerInfoSvc<T>
                    {
                        type Response = super::QueryLockerInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLockerInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_locker_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLockerInfoSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryLockersByAppToAssetID" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLockersByAppToAssetIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryLockersByAppToAssetIdRequest>
                        for QueryLockersByAppToAssetIDSvc<T>
                    {
                        type Response = super::QueryLockersByAppToAssetIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLockersByAppToAssetIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_lockers_by_app_to_asset_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLockersByAppToAssetIDSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryLockerInfoByAppID" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLockerInfoByAppIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryLockerInfoByAppIdRequest>
                        for QueryLockerInfoByAppIDSvc<T>
                    {
                        type Response = super::QueryLockerInfoByAppIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLockerInfoByAppIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_locker_info_by_app_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLockerInfoByAppIDSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryTotalDepositByAppAndAssetID" => {
                    #[allow(non_camel_case_types)]
                    struct QueryTotalDepositByAppAndAssetIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryTotalDepositByAppAndAssetIdRequest>
                        for QueryTotalDepositByAppAndAssetIDSvc<T>
                    {
                        type Response = super::QueryTotalDepositByAppAndAssetIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTotalDepositByAppAndAssetIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_total_deposit_by_app_and_asset_id(request)
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
                        let method = QueryTotalDepositByAppAndAssetIDSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryOwnerLockerByAppIDbyOwner" => {
                    #[allow(non_camel_case_types)]
                    struct QueryOwnerLockerByAppIDbyOwnerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryOwnerLockerByAppIDbyOwnerRequest>
                        for QueryOwnerLockerByAppIDbyOwnerSvc<T>
                    {
                        type Response = super::QueryOwnerLockerByAppIDbyOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryOwnerLockerByAppIDbyOwnerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_owner_locker_by_app_i_dby_owner(request)
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
                        let method = QueryOwnerLockerByAppIDbyOwnerSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryOwnerLockerOfAllAppsByOwner" => {
                    #[allow(non_camel_case_types)]
                    struct QueryOwnerLockerOfAllAppsByOwnerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryOwnerLockerOfAllAppsByOwnerRequest>
                        for QueryOwnerLockerOfAllAppsByOwnerSvc<T>
                    {
                        type Response = super::QueryOwnerLockerOfAllAppsByOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryOwnerLockerOfAllAppsByOwnerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_owner_locker_of_all_apps_by_owner(request)
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
                        let method = QueryOwnerLockerOfAllAppsByOwnerSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryOwnerTxDetailsLockerOfAppByOwnerByAsset" => {
                    #[allow(non_camel_case_types)]
                    struct QueryOwnerTxDetailsLockerOfAppByOwnerByAssetSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryOwnerTxDetailsLockerOfAppByOwnerByAssetRequest,
                        > for QueryOwnerTxDetailsLockerOfAppByOwnerByAssetSvc<T>
                    {
                        type Response = super::QueryOwnerTxDetailsLockerOfAppByOwnerByAssetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryOwnerTxDetailsLockerOfAppByOwnerByAssetRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_owner_tx_details_locker_of_app_by_owner_by_asset(request)
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
                        let method = QueryOwnerTxDetailsLockerOfAppByOwnerByAssetSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryOwnerLockerByAppToAssetIDbyOwner" => {
                    #[allow(non_camel_case_types)]
                    struct QueryOwnerLockerByAppToAssetIDbyOwnerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryOwnerLockerByAppToAssetIDbyOwnerRequest,
                        > for QueryOwnerLockerByAppToAssetIDbyOwnerSvc<T>
                    {
                        type Response = super::QueryOwnerLockerByAppToAssetIDbyOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryOwnerLockerByAppToAssetIDbyOwnerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_owner_locker_by_app_to_asset_i_dby_owner(request)
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
                        let method = QueryOwnerLockerByAppToAssetIDbyOwnerSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryLockerByAppByOwner" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLockerByAppByOwnerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryLockerByAppByOwnerRequest>
                        for QueryLockerByAppByOwnerSvc<T>
                    {
                        type Response = super::QueryLockerByAppByOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLockerByAppByOwnerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_locker_by_app_by_owner(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLockerByAppByOwnerSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryLockerCountByAppID" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLockerCountByAppIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryLockerCountByAppIdRequest>
                        for QueryLockerCountByAppIDSvc<T>
                    {
                        type Response = super::QueryLockerCountByAppIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLockerCountByAppIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_locker_count_by_app_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLockerCountByAppIDSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryLockerCountByAppToAssetID" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLockerCountByAppToAssetIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryLockerCountByAppToAssetIdRequest>
                        for QueryLockerCountByAppToAssetIDSvc<T>
                    {
                        type Response = super::QueryLockerCountByAppToAssetIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLockerCountByAppToAssetIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_locker_count_by_app_to_asset_id(request)
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
                        let method = QueryLockerCountByAppToAssetIDSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryWhiteListedAssetIDsByAppID" => {
                    #[allow(non_camel_case_types)]
                    struct QueryWhiteListedAssetIDsByAppIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryWhiteListedAssetIDsByAppIdRequest>
                        for QueryWhiteListedAssetIDsByAppIDSvc<T>
                    {
                        type Response = super::QueryWhiteListedAssetIDsByAppIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWhiteListedAssetIDsByAppIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_white_listed_asset_i_ds_by_app_id(request)
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
                        let method = QueryWhiteListedAssetIDsByAppIDSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryWhiteListedAssetByAllApps" => {
                    #[allow(non_camel_case_types)]
                    struct QueryWhiteListedAssetByAllAppsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryWhiteListedAssetByAllAppsRequest>
                        for QueryWhiteListedAssetByAllAppsSvc<T>
                    {
                        type Response = super::QueryWhiteListedAssetByAllAppsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWhiteListedAssetByAllAppsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_white_listed_asset_by_all_apps(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryWhiteListedAssetByAllAppsSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryLockerLookupTableByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLockerLookupTableByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryLockerLookupTableByAppRequest>
                        for QueryLockerLookupTableByAppSvc<T>
                    {
                        type Response = super::QueryLockerLookupTableByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLockerLookupTableByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_locker_lookup_table_by_app(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLockerLookupTableByAppSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryLockerLookupTableByAppAndAssetID" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLockerLookupTableByAppAndAssetIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryLockerLookupTableByAppAndAssetIdRequest,
                        > for QueryLockerLookupTableByAppAndAssetIDSvc<T>
                    {
                        type Response = super::QueryLockerLookupTableByAppAndAssetIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryLockerLookupTableByAppAndAssetIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_locker_lookup_table_by_app_and_asset_id(request)
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
                        let method = QueryLockerLookupTableByAppAndAssetIDSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryLockerTotalRewardsByAssetAppWise" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLockerTotalRewardsByAssetAppWiseSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryLockerTotalRewardsByAssetAppWiseRequest,
                        > for QueryLockerTotalRewardsByAssetAppWiseSvc<T>
                    {
                        type Response = super::QueryLockerTotalRewardsByAssetAppWiseResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryLockerTotalRewardsByAssetAppWiseRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_locker_total_rewards_by_asset_app_wise(request)
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
                        let method = QueryLockerTotalRewardsByAssetAppWiseSvc(inner);
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
                "/comdex.locker.v1beta1.Query/QueryLockerTotalDepositedByApp" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLockerTotalDepositedByAppSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryLockerTotalDepositedByAppRequest>
                        for QueryLockerTotalDepositedByAppSvc<T>
                    {
                        type Response = super::QueryLockerTotalDepositedByAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLockerTotalDepositedByAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_locker_total_deposited_by_app(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLockerTotalDepositedByAppSvc(inner);
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
        const NAME: &'static str = "comdex.locker.v1beta1.Query";
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
        pub async fn msg_create_locker(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateLockerRequest>,
        ) -> Result<tonic::Response<super::MsgCreateLockerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.locker.v1beta1.Msg/MsgCreateLocker");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn msg_deposit_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositAssetRequest>,
        ) -> Result<tonic::Response<super::MsgDepositAssetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.locker.v1beta1.Msg/MsgDepositAsset");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn msg_withdraw_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawAssetRequest>,
        ) -> Result<tonic::Response<super::MsgWithdrawAssetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.locker.v1beta1.Msg/MsgWithdrawAsset");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn msg_close_locker(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCloseLockerRequest>,
        ) -> Result<tonic::Response<super::MsgCloseLockerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/comdex.locker.v1beta1.Msg/MsgCloseLocker");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn msg_locker_reward_calc(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLockerRewardCalcRequest>,
        ) -> Result<tonic::Response<super::MsgLockerRewardCalcResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/comdex.locker.v1beta1.Msg/MsgLockerRewardCalc",
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
        async fn msg_create_locker(
            &self,
            request: tonic::Request<super::MsgCreateLockerRequest>,
        ) -> Result<tonic::Response<super::MsgCreateLockerResponse>, tonic::Status>;
        async fn msg_deposit_asset(
            &self,
            request: tonic::Request<super::MsgDepositAssetRequest>,
        ) -> Result<tonic::Response<super::MsgDepositAssetResponse>, tonic::Status>;
        async fn msg_withdraw_asset(
            &self,
            request: tonic::Request<super::MsgWithdrawAssetRequest>,
        ) -> Result<tonic::Response<super::MsgWithdrawAssetResponse>, tonic::Status>;
        async fn msg_close_locker(
            &self,
            request: tonic::Request<super::MsgCloseLockerRequest>,
        ) -> Result<tonic::Response<super::MsgCloseLockerResponse>, tonic::Status>;
        async fn msg_locker_reward_calc(
            &self,
            request: tonic::Request<super::MsgLockerRewardCalcRequest>,
        ) -> Result<tonic::Response<super::MsgLockerRewardCalcResponse>, tonic::Status>;
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
                "/comdex.locker.v1beta1.Msg/MsgCreateLocker" => {
                    #[allow(non_camel_case_types)]
                    struct MsgCreateLockerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateLockerRequest> for MsgCreateLockerSvc<T> {
                        type Response = super::MsgCreateLockerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateLockerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_create_locker(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgCreateLockerSvc(inner);
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
                "/comdex.locker.v1beta1.Msg/MsgDepositAsset" => {
                    #[allow(non_camel_case_types)]
                    struct MsgDepositAssetSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDepositAssetRequest> for MsgDepositAssetSvc<T> {
                        type Response = super::MsgDepositAssetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDepositAssetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_deposit_asset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgDepositAssetSvc(inner);
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
                "/comdex.locker.v1beta1.Msg/MsgWithdrawAsset" => {
                    #[allow(non_camel_case_types)]
                    struct MsgWithdrawAssetSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawAssetRequest>
                        for MsgWithdrawAssetSvc<T>
                    {
                        type Response = super::MsgWithdrawAssetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawAssetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_withdraw_asset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgWithdrawAssetSvc(inner);
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
                "/comdex.locker.v1beta1.Msg/MsgCloseLocker" => {
                    #[allow(non_camel_case_types)]
                    struct MsgCloseLockerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCloseLockerRequest> for MsgCloseLockerSvc<T> {
                        type Response = super::MsgCloseLockerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCloseLockerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_close_locker(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgCloseLockerSvc(inner);
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
                "/comdex.locker.v1beta1.Msg/MsgLockerRewardCalc" => {
                    #[allow(non_camel_case_types)]
                    struct MsgLockerRewardCalcSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgLockerRewardCalcRequest>
                        for MsgLockerRewardCalcSvc<T>
                    {
                        type Response = super::MsgLockerRewardCalcResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgLockerRewardCalcRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).msg_locker_reward_calc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MsgLockerRewardCalcSvc(inner);
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
        const NAME: &'static str = "comdex.locker.v1beta1.Msg";
    }
}
