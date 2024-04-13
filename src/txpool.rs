/// TxPoolSignal is used by tx pool to send signal to block proposer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxPoolSignal {
    /// transaction event type
    #[prost(enumeration = "SignalType", tag = "1")]
    pub signal_type: i32,
    /// chainId
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
}
/// TxPoolStatus defines txPool status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxPoolStatus {
    /// the max size of config tx pool
    #[prost(int32, tag = "1")]
    pub config_tx_pool_size: i32,
    /// the max size of common tx pool
    #[prost(int32, tag = "2")]
    pub common_tx_pool_size: i32,
    /// the num of config tx in queue cache
    #[prost(int32, tag = "3")]
    pub config_tx_num_in_queue: i32,
    /// the num of config tx in pending cache
    #[prost(int32, tag = "4")]
    pub config_tx_num_in_pending: i32,
    /// the num of common tx in queue cache
    #[prost(int32, tag = "5")]
    pub common_tx_num_in_queue: i32,
    /// the num of common tx in pending cache
    #[prost(int32, tag = "6")]
    pub common_tx_num_in_pending: i32,
}
/// TxPoolMsg contains all txPool msg type and msg body
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxPoolMsg {
    /// txPool message type
    #[prost(enumeration = "TxPoolMsgType", tag = "1")]
    pub r#type: i32,
    /// message bytes
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// transaction batch, used to add transaction efficiently in normal and batch txPool
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxBatch {
    /// batch id = timestamp(8byte)+nodeId(8byte)+batchHash(8byte)
    #[prost(string, tag = "1")]
    pub batch_id: ::prost::alloc::string::String,
    /// batch size
    #[prost(int32, tag = "3")]
    pub size: i32,
    /// transaction list
    #[prost(message, repeated, tag = "4")]
    pub txs: ::prost::alloc::vec::Vec<super::common::Transaction>,
    /// Map: transaction ID mapping record( key: transaction ID, value: transaction index in txs)
    #[prost(map = "string, int32", tag = "5")]
    pub tx_ids_map: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    /// batch signature
    #[prost(message, optional, tag = "6")]
    pub endorsement: ::core::option::Option<super::common::EndorsementEntry>,
}
/// transaction recover request, used to request transactions from proposer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRecoverRequest {
    /// node id
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    /// height
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// txId list
    #[prost(string, repeated, tag = "3")]
    pub tx_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// transaction recover response, used to return transactions to the validators by proposer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRecoverResponse {
    /// node id
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    /// height
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// tx list
    #[prost(message, repeated, tag = "3")]
    pub txs: ::prost::alloc::vec::Vec<super::common::Transaction>,
}
/// batch recover request, used to request batch from proposer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxBatchRecoverRequest {
    /// proposer node id
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    /// height
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// batchId list
    #[prost(string, repeated, tag = "3")]
    pub batch_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// batch recover response, used to return transactions to the validators by proposer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxBatchRecoverResponse {
    /// node id
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    /// height
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// batch list
    #[prost(message, repeated, tag = "3")]
    pub tx_batches: ::prost::alloc::vec::Vec<TxBatch>,
}
/// rpc get pool status request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPoolStatusRequest {
    /// blockchain identifier
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
}
/// rpc get tx ids by type and stage request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxIdsByTypeAndStageRequest {
    /// blockchain identifier
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// tx type
    #[prost(enumeration = "TxType", tag = "2")]
    pub tx_type: i32,
    /// tx stage
    #[prost(enumeration = "TxStage", tag = "3")]
    pub tx_stage: i32,
}
/// rpc get tx ids by type and stage response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxIdsByTypeAndStageResponse {
    /// tx id list
    #[prost(string, repeated, tag = "1")]
    pub tx_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// rpc get txs in pool by tx ids request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxsInPoolByTxIdsRequest {
    /// blockchain identifier
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// tx id list
    #[prost(string, repeated, tag = "2")]
    pub tx_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// rpc get txs in pool by tx ids response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxsInPoolByTxIdsResponse {
    /// txs in the tx pool
    #[prost(message, repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<super::common::Transaction>,
    /// tx ids of txs that are not in the tx pool
    #[prost(string, repeated, tag = "2")]
    pub tx_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// SignalType is a transaction event type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignalType {
    /// no transaction
    NoEvent = 0,
    /// new transaction
    TransactionIncome = 1,
    /// packing block
    BlockPropose = 2,
}
impl SignalType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignalType::NoEvent => "NO_EVENT",
            SignalType::TransactionIncome => "TRANSACTION_INCOME",
            SignalType::BlockPropose => "BLOCK_PROPOSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_EVENT" => Some(Self::NoEvent),
            "TRANSACTION_INCOME" => Some(Self::TransactionIncome),
            "BLOCK_PROPOSE" => Some(Self::BlockPropose),
            _ => None,
        }
    }
}
/// TxType is the transaction type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TxType {
    /// unknown
    UnknownType = 0,
    /// config transaction
    ConfigTx = 1,
    /// common transaction
    CommonTx = 2,
    /// config and common transaction
    AllType = 3,
}
impl TxType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TxType::UnknownType => "UNKNOWN_TYPE",
            TxType::ConfigTx => "CONFIG_TX",
            TxType::CommonTx => "COMMON_TX",
            TxType::AllType => "ALL_TYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_TYPE" => Some(Self::UnknownType),
            "CONFIG_TX" => Some(Self::ConfigTx),
            "COMMON_TX" => Some(Self::CommonTx),
            "ALL_TYPE" => Some(Self::AllType),
            _ => None,
        }
    }
}
/// TxStage is the current transaction stage
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TxStage {
    /// unknown
    UnknownStage = 0,
    ///   in queue
    InQueue = 1,
    /// in pending
    InPending = 2,
    /// in queue and in pending
    AllStage = 3,
}
impl TxStage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TxStage::UnknownStage => "UNKNOWN_STAGE",
            TxStage::InQueue => "IN_QUEUE",
            TxStage::InPending => "IN_PENDING",
            TxStage::AllStage => "ALL_STAGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_STAGE" => Some(Self::UnknownStage),
            "IN_QUEUE" => Some(Self::InQueue),
            "IN_PENDING" => Some(Self::InPending),
            "ALL_STAGE" => Some(Self::AllStage),
            _ => None,
        }
    }
}
/// TxPoolMsgType defines different type message in txPool
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TxPoolMsgType {
    /// single transaction type
    SingleTx = 0,
    /// batch transaction type
    BatchTx = 1,
    /// transaction recover request type
    RecoverReq = 2,
    /// transaction recover response type
    RecoverResp = 3,
}
impl TxPoolMsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TxPoolMsgType::SingleTx => "SINGLE_TX",
            TxPoolMsgType::BatchTx => "BATCH_TX",
            TxPoolMsgType::RecoverReq => "RECOVER_REQ",
            TxPoolMsgType::RecoverResp => "RECOVER_RESP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SINGLE_TX" => Some(Self::SingleTx),
            "BATCH_TX" => Some(Self::BatchTx),
            "RECOVER_REQ" => Some(Self::RecoverReq),
            "RECOVER_RESP" => Some(Self::RecoverResp),
            _ => None,
        }
    }
}