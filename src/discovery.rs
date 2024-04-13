/// information of a blockchain
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainInfo {
    /// block height
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
    /// node list
    #[prost(message, repeated, tag = "2")]
    pub node_list: ::prost::alloc::vec::Vec<Node>,
}
/// information of a blockchain node
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// node identifier
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    /// node address
    #[prost(string, tag = "2")]
    pub node_address: ::prost::alloc::string::String,
    /// TLS certificate of the node
    #[prost(bytes = "vec", tag = "3")]
    pub node_tls_cert: ::prost::alloc::vec::Vec<u8>,
}
/// chain_id_list
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainList {
    #[prost(string, repeated, tag = "1")]
    pub chain_id_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
