#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandidateInfo {
    /// Identification of the node
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
    /// The voting weight of the node
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
