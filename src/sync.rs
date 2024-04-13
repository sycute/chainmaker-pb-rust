/// network message of synchronization module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncMsg {
    /// sync message type
    #[prost(enumeration = "sync_msg::MsgType", tag = "1")]
    pub r#type: i32,
    /// payload for the message
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `SyncMsg`.
pub mod sync_msg {
    /// specific syncblockmessage types
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MsgType {
        NodeStatusReq = 0,
        NodeStatusResp = 1,
        BlockSyncReq = 2,
        BlockSyncResp = 3,
    }
    impl MsgType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MsgType::NodeStatusReq => "NODE_STATUS_REQ",
                MsgType::NodeStatusResp => "NODE_STATUS_RESP",
                MsgType::BlockSyncReq => "BLOCK_SYNC_REQ",
                MsgType::BlockSyncResp => "BLOCK_SYNC_RESP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NODE_STATUS_REQ" => Some(Self::NodeStatusReq),
                "NODE_STATUS_RESP" => Some(Self::NodeStatusResp),
                "BLOCK_SYNC_REQ" => Some(Self::BlockSyncReq),
                "BLOCK_SYNC_RESP" => Some(Self::BlockSyncResp),
                _ => None,
            }
        }
    }
}
/// response message for node status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeightBcm {
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
    #[prost(uint64, tag = "2")]
    pub archived_height: u64,
}
/// block request message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockSyncReq {
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
    #[prost(uint64, tag = "2")]
    pub batch_size: u64,
    #[prost(bool, tag = "3")]
    pub with_rwset: bool,
}
/// batch blocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockBatch {
    #[prost(message, repeated, tag = "1")]
    pub batches: ::prost::alloc::vec::Vec<super::common::Block>,
}
/// information of batch blocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfoBatch {
    #[prost(message, repeated, tag = "1")]
    pub batch: ::prost::alloc::vec::Vec<super::common::BlockInfo>,
}
/// block response message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncBlockBatch {
    #[prost(bool, tag = "3")]
    pub with_rwset: bool,
    #[prost(oneof = "sync_block_batch::Data", tags = "1, 2")]
    pub data: ::core::option::Option<sync_block_batch::Data>,
}
/// Nested message and enum types in `SyncBlockBatch`.
pub mod sync_block_batch {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// the response structure returned when requesting block data
        #[prost(message, tag = "1")]
        BlockBatch(super::BlockBatch),
        /// when requesting a block and its read / write set data
        #[prost(message, tag = "2")]
        BlockinfoBatch(super::BlockInfoBatch),
    }
}