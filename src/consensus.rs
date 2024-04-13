#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyResult {
    #[prost(message, optional, tag = "1")]
    pub verified_block: ::core::option::Option<super::common::Block>,
    #[prost(enumeration = "verify_result::Code", tag = "2")]
    pub code: i32,
    #[prost(string, tag = "3")]
    pub msg: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "4")]
    pub txs_rw_set:
        ::std::collections::HashMap<::prost::alloc::string::String, super::common::TxRwSet>,
    #[prost(message, optional, tag = "5")]
    pub rw_set_verify_fail_txs: ::core::option::Option<RwSetVerifyFailTxs>,
}
/// Nested message and enum types in `VerifyResult`.
pub mod verify_result {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        Success = 0,
        Fail = 1,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::Success => "SUCCESS",
                Code::Fail => "FAIL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUCCESS" => Some(Self::Success),
                "FAIL" => Some(Self::Fail),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RwSetVerifyFailTxs {
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
    #[prost(string, repeated, tag = "2")]
    pub tx_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalBlock {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<super::common::Block>,
    #[prost(map = "string, message", tag = "2")]
    pub txs_rw_set:
        ::std::collections::HashMap<::prost::alloc::string::String, super::common::TxRwSet>,
    #[prost(message, optional, tag = "3")]
    pub cut_block: ::core::option::Option<super::common::Block>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeaderConsensusArgs {
    #[prost(int64, tag = "1")]
    pub consensus_type: i64,
    /// status of the round in tbft
    #[prost(uint64, tag = "2")]
    pub round: u64,
    /// status of the view in maxbft
    #[prost(uint64, tag = "3")]
    pub view: u64,
    #[prost(message, optional, tag = "4")]
    pub consensus_data: ::core::option::Option<super::common::TxRwSet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GovernanceMember {
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub index: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GovernanceContract {
    /// epoch id，increase by epoch switch
    #[prost(uint64, tag = "1")]
    pub epoch_id: u64,
    /// consensus type
    #[prost(enumeration = "ConsensusType", tag = "2")]
    pub r#type: i32,
    /// current index to be assigned
    #[prost(int64, tag = "3")]
    pub cur_max_index: i64,
    #[prost(bool, tag = "4")]
    pub skip_timeout_commit: bool,
    /// bool is_config_chg = 4;   //is the configuration changed
    /// bool is_validator_chg = 5;    //is the validator changed
    ///
    /// config sequence, check whether the configuration is changed
    #[prost(uint64, tag = "6")]
    pub config_sequence: u64,
    /// number of nodes participating in the consensus
    #[prost(uint64, tag = "7")]
    pub n: u64,
    /// the minimum number of consensus nodes that need to survive
    #[prost(uint64, tag = "8")]
    pub min_quorum_for_qc: u64,
    #[prost(uint64, tag = "9")]
    pub cached_len: u64,
    /// the next height of switching validator
    #[prost(uint64, tag = "10")]
    pub next_switch_height: u64,
    /// the buffer height of switching validator
    #[prost(uint64, tag = "11")]
    pub transit_block: u64,
    /// cycle of switching validator
    #[prost(uint64, tag = "12")]
    pub block_num_per_epoch: u64,
    /// maximum number of participating validators
    #[prost(uint64, tag = "13")]
    pub validator_num: u64,
    /// the rounds in which each validator can produce blocks continuously
    #[prost(uint64, tag = "14")]
    pub node_propose_round: u64,
    /// currently maintained nodes
    #[prost(message, repeated, tag = "15")]
    pub members: ::prost::alloc::vec::Vec<GovernanceMember>,
    /// currently participate validators
    #[prost(message, repeated, tag = "16")]
    pub validators: ::prost::alloc::vec::Vec<GovernanceMember>,
    /// next participate validators
    #[prost(message, repeated, tag = "17")]
    pub next_validators: ::prost::alloc::vec::Vec<GovernanceMember>,
    /// the last epoch minimum number of consensus nodes that need to survive
    #[prost(uint64, tag = "18")]
    pub last_min_quorum_for_qc: u64,
    /// The base timeout for viewChange
    #[prost(uint64, tag = "19")]
    pub maxbft_round_timeout_mill: u64,
    /// The delta timeout for the viewChange
    #[prost(uint64, tag = "20")]
    pub maxbft_round_timeout_interval_mill: u64,
    /// last epoch participate validators
    #[prost(message, repeated, tag = "21")]
    pub last_validators: ::prost::alloc::vec::Vec<GovernanceMember>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConsensusType {
    Solo = 0,
    Tbft = 1,
    Mbft = 2,
    Maxbft = 3,
    Raft = 4,
    Dpos = 5,
    Pow = 10,
}
impl ConsensusType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConsensusType::Solo => "SOLO",
            ConsensusType::Tbft => "TBFT",
            ConsensusType::Mbft => "MBFT",
            ConsensusType::Maxbft => "MAXBFT",
            ConsensusType::Raft => "RAFT",
            ConsensusType::Dpos => "DPOS",
            ConsensusType::Pow => "POW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SOLO" => Some(Self::Solo),
            "TBFT" => Some(Self::Tbft),
            "MBFT" => Some(Self::Mbft),
            "MAXBFT" => Some(Self::Maxbft),
            "RAFT" => Some(Self::Raft),
            "DPOS" => Some(Self::Dpos),
            "POW" => Some(Self::Pow),
            _ => None,
        }
    }
}
