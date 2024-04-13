/// ValidatorSet represents the set of validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSet {
    #[prost(string, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TbftMsg {
    #[prost(enumeration = "TbftMsgType", tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// Proposal defined a consesensus proposal which can
/// be gossiped to other node and can be serilized
/// for persistent store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(string, tag = "1")]
    pub voter: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(int32, tag = "3")]
    pub round: i32,
    #[prost(int32, tag = "4")]
    pub pol_round: i32,
    #[prost(message, optional, tag = "5")]
    pub block: ::core::option::Option<super::common::Block>,
    #[prost(message, optional, tag = "6")]
    pub endorsement: ::core::option::Option<super::common::EndorsementEntry>,
    #[prost(map = "string, message", tag = "7")]
    pub txs_rw_set:
        ::std::collections::HashMap<::prost::alloc::string::String, super::common::TxRwSet>,
    #[prost(message, repeated, tag = "8")]
    pub qc: ::prost::alloc::vec::Vec<Vote>,
}
/// Vote represents a tbft vote
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    #[prost(enumeration = "VoteType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub height: u64,
    #[prost(int32, tag = "4")]
    pub round: i32,
    #[prost(bytes = "vec", tag = "5")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub endorsement: ::core::option::Option<super::common::EndorsementEntry>,
    #[prost(string, repeated, tag = "7")]
    pub invalid_txs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// BlockVotes represents votes as key-value form
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockVotes {
    #[prost(map = "string, message", tag = "1")]
    pub votes: ::std::collections::HashMap<::prost::alloc::string::String, Vote>,
    #[prost(uint64, tag = "2")]
    pub sum: u64,
}
/// VoteSet represents a set of vote at `height` and `round`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteSet {
    #[prost(enumeration = "VoteType", tag = "1")]
    pub r#type: i32,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(int32, tag = "3")]
    pub round: i32,
    #[prost(uint64, tag = "4")]
    pub sum: u64,
    #[prost(bytes = "vec", tag = "5")]
    pub maj23: ::prost::alloc::vec::Vec<u8>,
    #[prost(map = "string, message", tag = "6")]
    pub votes: ::std::collections::HashMap<::prost::alloc::string::String, Vote>,
    #[prost(map = "string, message", tag = "7")]
    pub votes_by_block: ::std::collections::HashMap<::prost::alloc::string::String, BlockVotes>,
}
/// RoundVoteSet represents voteSet of a `round`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundVoteSet {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(int32, tag = "2")]
    pub round: i32,
    #[prost(message, optional, tag = "3")]
    pub prevotes: ::core::option::Option<VoteSet>,
    #[prost(message, optional, tag = "4")]
    pub precommits: ::core::option::Option<VoteSet>,
}
/// HeightRoundVoteSet represents voteSet of a `height`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeightRoundVoteSet {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    /// max round
    #[prost(int32, tag = "2")]
    pub round: i32,
    #[prost(map = "int32, message", tag = "3")]
    pub round_vote_sets: ::std::collections::HashMap<i32, RoundVoteSet>,
}
/// ConsensusState represents the state of tbft instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// max round
    #[prost(int32, tag = "3")]
    pub round: i32,
    /// step
    #[prost(enumeration = "Step", tag = "4")]
    pub step: i32,
    #[prost(message, optional, tag = "5")]
    pub proposal: ::core::option::Option<Proposal>,
    #[prost(message, optional, tag = "6")]
    pub verifing_proposal: ::core::option::Option<Proposal>,
    #[prost(message, optional, tag = "7")]
    pub locked_proposal: ::core::option::Option<Proposal>,
    #[prost(message, optional, tag = "8")]
    pub valid_proposal: ::core::option::Option<Proposal>,
    #[prost(message, optional, tag = "9")]
    pub height_round_vote_set: ::core::option::Option<HeightRoundVoteSet>,
}
/// FetchRoundQC defined a fetch roundQC request which can
/// be gossiped to other node
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRoundQc {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(int32, tag = "3")]
    pub round: i32,
}
/// RoundQC represents the max round of qc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundQc {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(int32, tag = "3")]
    pub round: i32,
    #[prost(message, optional, tag = "4")]
    pub qc: ::core::option::Option<VoteSet>,
}
/// GossipState represents the state of tbft instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GossipState {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// max round
    #[prost(int32, tag = "3")]
    pub round: i32,
    /// step
    #[prost(enumeration = "Step", tag = "4")]
    pub step: i32,
    #[prost(bytes = "vec", tag = "5")]
    pub proposal: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub verifing_proposal: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "7")]
    pub round_vote_set: ::core::option::Option<RoundVoteSet>,
}
/// TimeoutInfo represents the timeout event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeoutInfo {
    #[prost(int64, tag = "1")]
    pub duration: i64,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(int32, tag = "3")]
    pub round: i32,
    #[prost(enumeration = "Step", tag = "4")]
    pub step: i32,
}
/// WalEntry represents the log entry in Wal
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalEntry {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    /// log entry type
    #[prost(enumeration = "WalEntryType", tag = "2")]
    pub r#type: i32,
    /// data of entry
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// TBFTMsgType defines different type message in tbft
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TbftMsgType {
    MsgPropose = 0,
    MsgPrevote = 1,
    MsgPrecommit = 2,
    MsgState = 3,
    MsgFetchRoundqc = 4,
    MsgSendRoundQc = 5,
}
impl TbftMsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TbftMsgType::MsgPropose => "MSG_PROPOSE",
            TbftMsgType::MsgPrevote => "MSG_PREVOTE",
            TbftMsgType::MsgPrecommit => "MSG_PRECOMMIT",
            TbftMsgType::MsgState => "MSG_STATE",
            TbftMsgType::MsgFetchRoundqc => "MSG_FETCH_ROUNDQC",
            TbftMsgType::MsgSendRoundQc => "MSG_SEND_ROUND_QC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MSG_PROPOSE" => Some(Self::MsgPropose),
            "MSG_PREVOTE" => Some(Self::MsgPrevote),
            "MSG_PRECOMMIT" => Some(Self::MsgPrecommit),
            "MSG_STATE" => Some(Self::MsgState),
            "MSG_FETCH_ROUNDQC" => Some(Self::MsgFetchRoundqc),
            "MSG_SEND_ROUND_QC" => Some(Self::MsgSendRoundQc),
            _ => None,
        }
    }
}
/// VoteType represents the type of vote
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoteType {
    VotePrevote = 0,
    VotePrecommit = 1,
}
impl VoteType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoteType::VotePrevote => "VOTE_PREVOTE",
            VoteType::VotePrecommit => "VOTE_PRECOMMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOTE_PREVOTE" => Some(Self::VotePrevote),
            "VOTE_PRECOMMIT" => Some(Self::VotePrecommit),
            _ => None,
        }
    }
}
/// Step represents the step in a round
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Step {
    NewHeight = 0,
    NewRound = 1,
    Propose = 2,
    Prevote = 3,
    PrevoteWait = 4,
    Precommit = 5,
    PrecommitWait = 6,
    Commit = 7,
}
impl Step {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Step::NewHeight => "NEW_HEIGHT",
            Step::NewRound => "NEW_ROUND",
            Step::Propose => "PROPOSE",
            Step::Prevote => "PREVOTE",
            Step::PrevoteWait => "PREVOTE_WAIT",
            Step::Precommit => "PRECOMMIT",
            Step::PrecommitWait => "PRECOMMIT_WAIT",
            Step::Commit => "COMMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NEW_HEIGHT" => Some(Self::NewHeight),
            "NEW_ROUND" => Some(Self::NewRound),
            "PROPOSE" => Some(Self::Propose),
            "PREVOTE" => Some(Self::Prevote),
            "PREVOTE_WAIT" => Some(Self::PrevoteWait),
            "PRECOMMIT" => Some(Self::Precommit),
            "PRECOMMIT_WAIT" => Some(Self::PrecommitWait),
            "COMMIT" => Some(Self::Commit),
            _ => None,
        }
    }
}
/// WalEntryType represents different types of entries in Wal
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WalEntryType {
    TimeoutEntry = 0,
    ProposalEntry = 1,
    VoteEntry = 2,
    ProposalVoteEntry = 3,
}
impl WalEntryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WalEntryType::TimeoutEntry => "TIMEOUT_ENTRY",
            WalEntryType::ProposalEntry => "PROPOSAL_ENTRY",
            WalEntryType::VoteEntry => "VOTE_ENTRY",
            WalEntryType::ProposalVoteEntry => "PROPOSAL_VOTE_ENTRY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TIMEOUT_ENTRY" => Some(Self::TimeoutEntry),
            "PROPOSAL_ENTRY" => Some(Self::ProposalEntry),
            "VOTE_ENTRY" => Some(Self::VoteEntry),
            "PROPOSAL_VOTE_ENTRY" => Some(Self::ProposalVoteEntry),
            _ => None,
        }
    }
}
