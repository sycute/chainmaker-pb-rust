#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusMsg {
    #[prost(enumeration = "MessageType", tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalData {
    /// block info of the proposal
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<super::common::Block>,
    /// consensus view of the proposal
    #[prost(uint64, tag = "2")]
    pub view: u64,
    #[prost(string, tag = "3")]
    pub proposer: ::prost::alloc::string::String,
    /// the qc info of the parent's block in the proposal
    #[prost(message, optional, tag = "4")]
    pub justify_qc: ::core::option::Option<QuorumCert>,
    #[prost(uint64, tag = "5")]
    pub epoch_id: u64,
    #[prost(message, repeated, tag = "6")]
    pub tx_rw_set: ::prost::alloc::vec::Vec<super::common::TxRwSet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuorumCert {
    /// votes in qc
    #[prost(message, repeated, tag = "1")]
    pub votes: ::prost::alloc::vec::Vec<VoteData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteData {
    /// block id of the vote block
    #[prost(bytes = "vec", tag = "1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    /// block height of the vote block
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// consensus view of the vote block
    #[prost(uint64, tag = "3")]
    pub view: u64,
    /// voter of the voteInfo
    #[prost(bytes = "vec", tag = "4")]
    pub author: ::prost::alloc::vec::Vec<u8>,
    /// the id of the epoch
    #[prost(uint64, tag = "5")]
    pub epoch_id: u64,
    /// signature of the vote
    #[prost(message, optional, tag = "6")]
    pub signature: ::core::option::Option<super::common::EndorsementEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewData {
    #[prost(uint64, tag = "1")]
    pub view: u64,
    #[prost(uint64, tag = "2")]
    pub epoch_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalFetchMsg {
    /// block id of the request proposal
    #[prost(bytes = "vec", tag = "1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    /// block height of the request block
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// consensus view of the request block
    #[prost(uint64, tag = "3")]
    pub view: u64,
    /// identify of the requester
    #[prost(bytes = "vec", tag = "4")]
    pub requester: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalRespMsg {
    /// proposal in response
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<ProposalData>,
    /// identify of the responser
    #[prost(bytes = "vec", tag = "2")]
    pub responser: ::prost::alloc::vec::Vec<u8>,
    /// qc of the proposal
    #[prost(message, optional, tag = "3")]
    pub qc: ::core::option::Option<QuorumCert>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildProposal {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint64, tag = "2")]
    pub view: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub pre_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalEntry {
    #[prost(bytes = "vec", tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "MessageType", tag = "2")]
    pub msg_type: i32,
    #[prost(uint64, tag = "3")]
    pub last_snapshot_index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GovernanceContract {
    /// current epoch id
    #[prost(uint64, tag = "1")]
    pub epoch_id: u64,
    /// the last view or block height in the epoch
    #[prost(uint64, tag = "2")]
    pub end_view: u64,
    /// list of consensus nodeIds
    #[prost(string, repeated, tag = "3")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// chain config version witch the epoch was based on
    #[prost(uint64, tag = "4")]
    pub config_sequence: u64,
    /// chain config
    #[prost(message, optional, tag = "5")]
    pub chain_config: ::core::option::Option<super::config::ChainConfig>,
    /// cert frozen list
    #[prost(bytes = "vec", tag = "6")]
    pub cert_frozen_list: ::prost::alloc::vec::Vec<u8>,
    /// CRL
    #[prost(bytes = "vec", tag = "7")]
    pub crl: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeStatus {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint64, tag = "2")]
    pub view: u64,
    #[prost(uint64, tag = "3")]
    pub epoch: u64,
    #[prost(string, tag = "4")]
    pub node_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposeBlock {
    #[prost(bool, tag = "1")]
    pub is_propose: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageType {
    ProposalMessage = 0,
    VoteMessage = 1,
    ProposalFetchMessage = 2,
    ProposalRespMessage = 3,
    NewViewMessage = 4,
}
impl MessageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageType::ProposalMessage => "PROPOSAL_MESSAGE",
            MessageType::VoteMessage => "VOTE_MESSAGE",
            MessageType::ProposalFetchMessage => "PROPOSAL_FETCH_MESSAGE",
            MessageType::ProposalRespMessage => "PROPOSAL_RESP_MESSAGE",
            MessageType::NewViewMessage => "NEW_VIEW_MESSAGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROPOSAL_MESSAGE" => Some(Self::ProposalMessage),
            "VOTE_MESSAGE" => Some(Self::VoteMessage),
            "PROPOSAL_FETCH_MESSAGE" => Some(Self::ProposalFetchMessage),
            "PROPOSAL_RESP_MESSAGE" => Some(Self::ProposalRespMessage),
            "NEW_VIEW_MESSAGE" => Some(Self::NewViewMessage),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConsStateType {
    /// The collection phase of the votes
    VoteCollect = 0,
    /// pacemaker of the consensus
    Pacemaker = 1,
}
impl ConsStateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConsStateType::VoteCollect => "VOTE_COLLECT",
            ConsStateType::Pacemaker => "PACEMAKER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOTE_COLLECT" => Some(Self::VoteCollect),
            "PACEMAKER" => Some(Self::Pacemaker),
            _ => None,
        }
    }
}