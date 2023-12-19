#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePromoCodeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag = "2")]
    pub total_promo_code: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "3")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag = "4")]
    pub quantity_per_customer: u32,
    #[prost(double, optional, tag = "5")]
    pub total_money_discount: ::core::option::Option<f64>,
    #[prost(uint32, optional, tag = "6")]
    pub percentage_discount: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "7")]
    pub date_activate_promo_code: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "8")]
    pub free_shipping: bool,
    #[prost(string, repeated, tag = "9")]
    pub items_promotion: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "10")]
    pub rule: ::core::option::Option<CreatePromoCodeRuleRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePromoCodeRuleRequest {
    #[prost(bool, optional, tag = "1")]
    pub new_user: ::core::option::Option<bool>,
    #[prost(double, optional, tag = "2")]
    pub min_money: ::core::option::Option<f64>,
    #[prost(uint32, optional, tag = "3")]
    pub min_items: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "4")]
    pub create_promo_code_rule_item_request: ::core::option::Option<
        CreatePromoCodeRuleItemRequest,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePromoCodeRuleItemRequest {
    #[prost(uint32, tag = "1")]
    pub min_total_item: u32,
    #[prost(bool, tag = "2")]
    pub is_valid_min_item_just_items_inside_promotion: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePromoCodeResponse {
    #[prost(string, tag = "1")]
    pub id_promotion_created: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag = "3")]
    pub total_promo_code: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "4")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag = "5")]
    pub quantity_per_customer: u32,
    #[prost(float, optional, tag = "6")]
    pub total_money_discount: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "7")]
    pub percentage_discount: ::core::option::Option<u32>,
    #[prost(bool, tag = "8")]
    pub free_shipping: bool,
    #[prost(message, optional, tag = "9")]
    pub date_activate_promo_code: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPromoCodeRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPromoCodeResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserPromotionsRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserPromotionsResponse {
    #[prost(string, tag = "1")]
    pub id_promotion_created: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag = "4")]
    pub quantity_per_customer: u32,
    #[prost(double, optional, tag = "5")]
    pub total_money_discount: ::core::option::Option<f64>,
    #[prost(uint32, optional, tag = "6")]
    pub percentage_discount: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGeneralPromotionsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGeneralPromotionsResponse {
    #[prost(string, tag = "1")]
    pub id_promotion_created: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag = "4")]
    pub quantity_per_customer: u32,
    #[prost(double, optional, tag = "5")]
    pub total_money_discount: ::core::option::Option<f64>,
    #[prost(uint32, optional, tag = "6")]
    pub percentage_discount: ::core::option::Option<u32>,
}
/// Generated server implementations.
pub mod promotion_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PromotionServiceServer.
    #[async_trait]
    pub trait PromotionService: Send + Sync + 'static {
        async fn create_promo_code(
            &self,
            request: tonic::Request<super::CreatePromoCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePromoCodeResponse>,
            tonic::Status,
        >;
        async fn apply_promo_code(
            &self,
            request: tonic::Request<super::ApplyPromoCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApplyPromoCodeResponse>,
            tonic::Status,
        >;
        async fn get_user_promotions(
            &self,
            request: tonic::Request<super::GetUserPromotionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserPromotionsResponse>,
            tonic::Status,
        >;
        async fn get_general_promotions(
            &self,
            request: tonic::Request<super::GetGeneralPromotionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGeneralPromotionsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PromotionServiceServer<T: PromotionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PromotionService> PromotionServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PromotionServiceServer<T>
    where
        T: PromotionService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/promotions_service_proto.PromotionService/CreatePromoCode" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePromoCodeSvc<T: PromotionService>(pub Arc<T>);
                    impl<
                        T: PromotionService,
                    > tonic::server::UnaryService<super::CreatePromoCodeRequest>
                    for CreatePromoCodeSvc<T> {
                        type Response = super::CreatePromoCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePromoCodeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PromotionService>::create_promo_code(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreatePromoCodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/promotions_service_proto.PromotionService/ApplyPromoCode" => {
                    #[allow(non_camel_case_types)]
                    struct ApplyPromoCodeSvc<T: PromotionService>(pub Arc<T>);
                    impl<
                        T: PromotionService,
                    > tonic::server::UnaryService<super::ApplyPromoCodeRequest>
                    for ApplyPromoCodeSvc<T> {
                        type Response = super::ApplyPromoCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ApplyPromoCodeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PromotionService>::apply_promo_code(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ApplyPromoCodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/promotions_service_proto.PromotionService/GetUserPromotions" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserPromotionsSvc<T: PromotionService>(pub Arc<T>);
                    impl<
                        T: PromotionService,
                    > tonic::server::UnaryService<super::GetUserPromotionsRequest>
                    for GetUserPromotionsSvc<T> {
                        type Response = super::GetUserPromotionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserPromotionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PromotionService>::get_user_promotions(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserPromotionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/promotions_service_proto.PromotionService/GetGeneralPromotions" => {
                    #[allow(non_camel_case_types)]
                    struct GetGeneralPromotionsSvc<T: PromotionService>(pub Arc<T>);
                    impl<
                        T: PromotionService,
                    > tonic::server::UnaryService<super::GetGeneralPromotionsRequest>
                    for GetGeneralPromotionsSvc<T> {
                        type Response = super::GetGeneralPromotionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGeneralPromotionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PromotionService>::get_general_promotions(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGeneralPromotionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PromotionService> Clone for PromotionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: PromotionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PromotionService> tonic::server::NamedService for PromotionServiceServer<T> {
        const NAME: &'static str = "promotions_service_proto.PromotionService";
    }
}
