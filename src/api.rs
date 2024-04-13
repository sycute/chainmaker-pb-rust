/// Generated client implementations.
pub mod rpc_node_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// rpcNnode is the server API for
    #[derive(Debug, Clone)]
    pub struct RpcNodeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RpcNodeClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RpcNodeClient<T>
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
        ) -> RpcNodeClient<InterceptedService<T, F>>
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
            RpcNodeClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// processing transaction message requests
        pub async fn send_request(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::TxRequest>,
        ) -> std::result::Result<tonic::Response<super::super::common::TxResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.RpcNode/SendRequest");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "SendRequest"));
            self.inner.unary(req, path, codec).await
        }
        /// processing transaction message requests in sync mode.
        /// returns tx execution result
        pub async fn send_request_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::TxRequest>,
        ) -> std::result::Result<tonic::Response<super::super::common::TxResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.RpcNode/SendRequestSync");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "SendRequestSync"));
            self.inner.unary(req, path, codec).await
        }
        /// processing requests for message subscription
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::TxRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::super::common::SubscribeResult>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.RpcNode/Subscribe");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "Subscribe"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// processing requests for message subscription by websocket
        pub async fn subscribe_ws(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::RawTxRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::super::common::SubscribeResult>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.RpcNode/SubscribeWS");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "SubscribeWS"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// update debug status (development debugging)
        pub async fn update_debug_config(
            &mut self,
            request: impl tonic::IntoRequest<super::super::config::DebugConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::config::DebugConfigResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.RpcNode/UpdateDebugConfig");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "UpdateDebugConfig"));
            self.inner.unary(req, path, codec).await
        }
        /// refreshLogLevelsConfig
        pub async fn refresh_log_levels_config(
            &mut self,
            request: impl tonic::IntoRequest<super::super::config::LogLevelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::config::LogLevelsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.RpcNode/RefreshLogLevelsConfig");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "RefreshLogLevelsConfig"));
            self.inner.unary(req, path, codec).await
        }
        /// get chainmaker version
        pub async fn get_chain_maker_version(
            &mut self,
            request: impl tonic::IntoRequest<super::super::config::ChainMakerVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::config::ChainMakerVersionResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.RpcNode/GetChainMakerVersion");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "GetChainMakerVersion"));
            self.inner.unary(req, path, codec).await
        }
        /// check chain configuration and load new chain dynamically
        pub async fn check_new_block_chain_config(
            &mut self,
            request: impl tonic::IntoRequest<super::super::config::CheckNewBlockChainConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::config::CheckNewBlockChainConfigResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.RpcNode/CheckNewBlockChainConfig");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "CheckNewBlockChainConfig"));
            self.inner.unary(req, path, codec).await
        }
        /// GetPoolStatus Returns the max size of config transaction pool and common transaction pool,
        /// the num of config transaction in queue and pendingCache,
        /// and the the num of common transaction in queue and pendingCache.
        pub async fn get_pool_status(
            &mut self,
            request: impl tonic::IntoRequest<super::super::txpool::GetPoolStatusRequest>,
        ) -> std::result::Result<tonic::Response<super::super::txpool::TxPoolStatus>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.RpcNode/GetPoolStatus");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "GetPoolStatus"));
            self.inner.unary(req, path, codec).await
        }
        /// GetTxIdsByTypeAndStage Returns config or common txIds in different stage.
        /// TxType may be TxType_CONFIG_TX, TxType_COMMON_TX, (TxType_CONFIG_TX|TxType_COMMON_TX)
        /// TxStage may be TxStage_IN_QUEUE, TxStage_IN_PENDING, (TxStage_IN_QUEUE|TxStage_IN_PENDING)
        pub async fn get_tx_ids_by_type_and_stage(
            &mut self,
            request: impl tonic::IntoRequest<super::super::txpool::GetTxIdsByTypeAndStageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::txpool::GetTxIdsByTypeAndStageResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.RpcNode/GetTxIdsByTypeAndStage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "GetTxIdsByTypeAndStage"));
            self.inner.unary(req, path, codec).await
        }
        /// GetTxsInPoolByTxIds Retrieve the transactions by the txIds from the txPool,
        /// return transactions in the txPool and txIds not in txPool.
        /// default query upper limit is 1w transaction, and error is returned if the limit is exceeded.
        pub async fn get_txs_in_pool_by_tx_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::super::txpool::GetTxsInPoolByTxIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::txpool::GetTxsInPoolByTxIdsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.RpcNode/GetTxsInPoolByTxIds");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.RpcNode", "GetTxsInPoolByTxIds"));
            self.inner.unary(req, path, codec).await
        }
    }
}
