/// wrapped network message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<NetMsg>,
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
    /// 属于那个模块，判断消息类型
    #[prost(string, tag = "3")]
    pub flag: ::prost::alloc::string::String,
}
/// net message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetMsg {
    /// payload of the message
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    /// message type
    #[prost(enumeration = "net_msg::MsgType", tag = "2")]
    pub r#type: i32,
    /// nodeId
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
}
/// Nested message and enum types in `NetMsg`.
pub mod net_msg {
    /// specific net message types
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MsgType {
        InvalidMsg = 0,
        Tx = 1,
        Txs = 2,
        Block = 3,
        Blocks = 4,
        ConsensusMsg = 5,
        SyncBlockMsg = 6,
        ConsistentMsg = 7,
    }
    impl MsgType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MsgType::InvalidMsg => "INVALID_MSG",
                MsgType::Tx => "TX",
                MsgType::Txs => "TXS",
                MsgType::Block => "BLOCK",
                MsgType::Blocks => "BLOCKS",
                MsgType::ConsensusMsg => "CONSENSUS_MSG",
                MsgType::SyncBlockMsg => "SYNC_BLOCK_MSG",
                MsgType::ConsistentMsg => "CONSISTENT_MSG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVALID_MSG" => Some(Self::InvalidMsg),
                "TX" => Some(Self::Tx),
                "TXS" => Some(Self::Txs),
                "BLOCK" => Some(Self::Block),
                "BLOCKS" => Some(Self::Blocks),
                "CONSENSUS_MSG" => Some(Self::ConsensusMsg),
                "SYNC_BLOCK_MSG" => Some(Self::SyncBlockMsg),
                "CONSISTENT_MSG" => Some(Self::ConsistentMsg),
                _ => None,
            }
        }
    }
}
