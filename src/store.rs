/// KeyModification -- QueryResult for history db query. Holds a transaction ID, value,
/// timestamp, and delete marker which resulted from a history query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyModification {
    #[prost(string, tag = "1")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    #[prost(bool, tag = "4")]
    pub is_delete: bool,
    #[prost(uint64, tag = "5")]
    pub block_height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxHistory {
    #[prost(string, tag = "1")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Kv {
    #[prost(string, tag = "1")]
    pub contract_name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// block structure used in serialization
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedBlock {
    /// header of block
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::common::BlockHeader>,
    /// transaction execution sequence of the block, described by DAG
    #[prost(message, optional, tag = "2")]
    pub dag: ::core::option::Option<super::common::Dag>,
    /// transaction id list within the block
    #[prost(string, repeated, tag = "3")]
    pub tx_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// block additional data, not included in block hash calculation
    #[prost(message, optional, tag = "4")]
    pub additional_data: ::core::option::Option<super::common::AdditionalData>,
}
/// Block and its read/write set information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockWithRwSet {
    /// block data
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<super::common::Block>,
    /// transaction read/write set of blocks
    #[prost(message, repeated, tag = "2")]
    pub tx_rw_sets: ::prost::alloc::vec::Vec<super::common::TxRwSet>,
    /// contract event info
    #[prost(message, repeated, tag = "3")]
    pub contract_events: ::prost::alloc::vec::Vec<super::common::ContractEvent>,
}
/// transaction info include transaction and its block height hash and tx index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionStoreInfo {
    /// transaction raw data
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<super::common::Transaction>,
    /// block height
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    /// Deprecated, block hash
    #[prost(bytes = "vec", tag = "3")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    /// transaction index in block
    #[prost(uint32, tag = "4")]
    pub tx_index: u32,
    /// block header timestamp
    #[prost(int64, tag = "5")]
    pub block_timestamp: i64,
    /// transaction offset index in file
    #[prost(message, optional, tag = "6")]
    pub transaction_store_info: ::core::option::Option<StoreInfo>,
}
/// store data information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreInfo {
    /// store type
    #[prost(enumeration = "DataStoreType", tag = "1")]
    pub store_type: i32,
    /// file name
    #[prost(string, tag = "2")]
    pub file_name: ::prost::alloc::string::String,
    /// offset in file
    #[prost(uint64, tag = "3")]
    pub offset: u64,
    /// data length
    #[prost(uint64, tag = "4")]
    pub byte_len: u64,
}
/// xxx.fdb information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileRange {
    /// file name: 00000000000000000019.xxx
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
    /// file contains block start height
    #[prost(uint64, tag = "2")]
    pub start: u64,
    /// file contains block end height
    #[prost(uint64, tag = "3")]
    pub end: u64,
}
/// archive status data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveStatus {
    /// store type
    #[prost(enumeration = "StoreType", tag = "1")]
    pub r#type: i32,
    /// file range data
    #[prost(message, repeated, tag = "2")]
    pub file_ranges: ::prost::alloc::vec::Vec<FileRange>,
    /// current archive pivot height
    #[prost(uint64, tag = "3")]
    pub archive_pivot: u64,
    /// max allow archive height
    #[prost(uint64, tag = "4")]
    pub max_allow_archive_height: u64,
    /// node archive process status
    #[prost(enumeration = "ArchiveProcess", tag = "5")]
    pub process: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DbType {
    InvalidDb = 0,
    BlockDb = 1,
    BlockIndexDb = 2,
    TxDb = 3,
    TxIndexDb = 4,
    SoftDb = 5,
    StateDb = 6,
    ReadWriteDb = 7,
}
impl DbType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DbType::InvalidDb => "INVALID_DB",
            DbType::BlockDb => "BLOCK_DB",
            DbType::BlockIndexDb => "BLOCK_INDEX_DB",
            DbType::TxDb => "TX_DB",
            DbType::TxIndexDb => "TX_INDEX_DB",
            DbType::SoftDb => "SOFT_DB",
            DbType::StateDb => "STATE_DB",
            DbType::ReadWriteDb => "READ_WRITE_DB",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_DB" => Some(Self::InvalidDb),
            "BLOCK_DB" => Some(Self::BlockDb),
            "BLOCK_INDEX_DB" => Some(Self::BlockIndexDb),
            "TX_DB" => Some(Self::TxDb),
            "TX_INDEX_DB" => Some(Self::TxIndexDb),
            "SOFT_DB" => Some(Self::SoftDb),
            "STATE_DB" => Some(Self::StateDb),
            "READ_WRITE_DB" => Some(Self::ReadWriteDb),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataStoreType {
    /// 文件系统存储
    FileStore = 0,
    /// SQL数据库存储
    SqlStore = 1,
    /// 云对象存储
    Cos = 2,
}
impl DataStoreType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataStoreType::FileStore => "FILE_STORE",
            DataStoreType::SqlStore => "SQL_STORE",
            DataStoreType::Cos => "COS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FILE_STORE" => Some(Self::FileStore),
            "SQL_STORE" => Some(Self::SqlStore),
            "COS" => Some(Self::Cos),
            _ => None,
        }
    }
}
/// store type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoreType {
    RawDb = 0,
    Bfdb = 1,
}
impl StoreType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StoreType::RawDb => "RawDB",
            StoreType::Bfdb => "BFDB",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RawDB" => Some(Self::RawDb),
            "BFDB" => Some(Self::Bfdb),
            _ => None,
        }
    }
}
/// archive in process status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ArchiveProcess {
    /// chain is normal status
    Normal = 0,
    /// chain is doing archive
    Archiving = 1,
    /// chain is doing restore
    Restoring = 2,
}
impl ArchiveProcess {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ArchiveProcess::Normal => "Normal",
            ArchiveProcess::Archiving => "Archiving",
            ArchiveProcess::Restoring => "Restoring",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Normal" => Some(Self::Normal),
            "Archiving" => Some(Self::Archiving),
            "Restoring" => Some(Self::Restoring),
            _ => None,
        }
    }
}
