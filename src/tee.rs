#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitEnclaveRequest {
    /// Sign algorithm of TEE
    #[prost(string, tag = "1")]
    pub tee_cert_sign_alg: ::prost::alloc::string::String,
    /// Encrypt algorithm of TEE
    #[prost(string, tag = "2")]
    pub tee_encrypt_alg: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitEnclaveResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub tee_report: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub tee_pubkey: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub tee_csr: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnclaveResponse {
    #[prost(message, optional, tag = "1")]
    pub enclave_response_payload: ::core::option::Option<EnclaveResponsePayload>,
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnclaveResponsePayload {
    #[prost(message, optional, tag = "1")]
    pub contract_result: ::core::option::Option<super::common::ContractResult>,
    #[prost(message, optional, tag = "2")]
    pub tx_rwset: ::core::option::Option<super::common::TxRwSet>,
    #[prost(message, optional, tag = "3")]
    pub tx_request: ::core::option::Option<super::common::TxRequest>,
    #[prost(string, tag = "4")]
    pub contract_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub contract_version: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub contract_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub report_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteAttestationRequest {
    #[prost(string, tag = "1")]
    pub challenge: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteAttestationResponse {
    #[prost(message, optional, tag = "1")]
    pub remote_attestation_payload: ::core::option::Option<RemoteAttestationPayload>,
    /// signature on challenge + report + tee_cert
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteAttestationPayload {
    #[prost(string, tag = "1")]
    pub challenge: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub report: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub tee_cert: ::prost::alloc::vec::Vec<u8>,
}
/// out call get
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutCallGetRequest {
    #[prost(string, tag = "1")]
    pub contract_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
/// out call put
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutCallPutRequest {
    #[prost(string, tag = "1")]
    pub contract_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
