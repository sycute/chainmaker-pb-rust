#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResp {
    #[prost(enumeration = "RegisterStatus", tag = "1")]
    pub register_status: i32,
    /// code > 0 means error
    #[prost(uint32, tag = "2")]
    pub code: u32,
    /// default "",else means error msg
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveBlockRequest {
    #[prost(string, tag = "1")]
    pub chain_unique: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<super::common::BlockInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleArchiveBlockResp {
    #[prost(uint64, tag = "1")]
    pub archived_begin_height: u64,
    #[prost(uint64, tag = "2")]
    pub archived_end_height: u64,
    /// code > 0 means error
    #[prost(uint32, tag = "3")]
    pub code: u32,
    /// default "",else means error msg
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveBlockResp {
    #[prost(enumeration = "ArchiveStatus", tag = "1")]
    pub archive_status: i32,
    /// code > 0 means error
    #[prost(uint32, tag = "2")]
    pub code: u32,
    /// default "",else means error msg
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveStatusRequest {
    #[prost(string, tag = "1")]
    pub chain_unique: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveStatusResp {
    #[prost(uint64, tag = "1")]
    pub archived_height: u64,
    #[prost(bool, tag = "2")]
    pub in_archive: bool,
    /// code > 0 means error
    #[prost(uint32, tag = "3")]
    pub code: u32,
    /// default "",else means error msg
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeBlocksRequest {
    #[prost(string, tag = "1")]
    pub chain_unique: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub start_height: u64,
    #[prost(uint64, tag = "3")]
    pub end_height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockByHashRequest {
    #[prost(string, tag = "1")]
    pub chain_unique: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(enumeration = "OperationByHash", tag = "3")]
    pub operation: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockByHeightRequest {
    #[prost(string, tag = "1")]
    pub chain_unique: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// 根据块高读查询信息
    #[prost(enumeration = "OperationByHeight", tag = "3")]
    pub operation: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockByTxIdRequest {
    #[prost(string, tag = "1")]
    pub chain_unique: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tx_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxDetailByIdRequest {
    #[prost(string, tag = "1")]
    pub chain_unique: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(enumeration = "OperationByTxId", tag = "3")]
    pub operation: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxDetailByIdResp {
    /// 区块高度
    #[prost(uint64, tag = "1")]
    pub height: u64,
    /// 区块是否存在
    #[prost(bool, tag = "2")]
    pub tx_exist: bool,
    /// tx确认时间
    #[prost(uint64, tag = "3")]
    pub tx_confirmed_time: u64,
    /// code > 0 means error
    #[prost(uint32, tag = "4")]
    pub code: u32,
    /// default "",else means error msg
    #[prost(string, tag = "5")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockWithRwSetResp {
    /// 区块数据
    #[prost(message, optional, tag = "1")]
    pub block_data: ::core::option::Option<super::common::BlockInfo>,
    /// code > 0 means error
    #[prost(uint32, tag = "2")]
    pub code: u32,
    /// default "",else means error msg
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRwSetResp {
    /// 读写集数据
    #[prost(message, optional, tag = "1")]
    pub rw_set: ::core::option::Option<super::common::TxRwSet>,
    /// code > 0 means error
    #[prost(uint32, tag = "2")]
    pub code: u32,
    /// default "",else means error msg
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionResp {
    /// 交易
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<super::common::Transaction>,
    /// code > 0 means error
    #[prost(uint32, tag = "2")]
    pub code: u32,
    /// default "",else means error msg
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreStatusResp {
    #[prost(message, optional, tag = "1")]
    pub store_status: ::core::option::Option<StoreStatus>,
    /// code > 0 means error
    #[prost(uint32, tag = "2")]
    pub code: u32,
    /// default "",else means error msg
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreStatus {
    #[prost(string, tag = "1")]
    pub chain_unique: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub store_infos: ::prost::alloc::vec::Vec<StoreInfo>,
    #[prost(int64, tag = "3")]
    pub size: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreInfo {
    #[prost(enumeration = "StoreDataType", tag = "1")]
    pub r#type: i32,
    #[prost(message, repeated, tag = "2")]
    pub file_infos: ::prost::alloc::vec::Vec<FileInfo>,
    #[prost(int64, tag = "3")]
    pub size: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub start: u64,
    #[prost(uint64, tag = "3")]
    pub end: u64,
    #[prost(int64, tag = "4")]
    pub size: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationByHash {
    /// 1. 检查区块是否存在，如果存在返回块高度
    OperationBlockExists = 0,
    /// 2. 查询common.Block
    OperationGetBlockByHash = 1,
    /// 3. 查询块高度，仅返回
    OperationGetHeightByHash = 2,
}
impl OperationByHash {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationByHash::OperationBlockExists => "OperationBlockExists",
            OperationByHash::OperationGetBlockByHash => "OperationGetBlockByHash",
            OperationByHash::OperationGetHeightByHash => "OperationGetHeightByHash",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OperationBlockExists" => Some(Self::OperationBlockExists),
            "OperationGetBlockByHash" => Some(Self::OperationGetBlockByHash),
            "OperationGetHeightByHash" => Some(Self::OperationGetHeightByHash),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationByHeight {
    /// 查询common.Block
    OperationGetBlockByHeight = 0,
    /// 仅仅查询common.BlockHeader
    OperationGetBlockHeaderByHeight = 1,
}
impl OperationByHeight {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationByHeight::OperationGetBlockByHeight => "OperationGetBlockByHeight",
            OperationByHeight::OperationGetBlockHeaderByHeight => "OperationGetBlockHeaderByHeight",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OperationGetBlockByHeight" => Some(Self::OperationGetBlockByHeight),
            "OperationGetBlockHeaderByHeight" => Some(Self::OperationGetBlockHeaderByHeight),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationByTxId {
    /// 根据txid查询块高度
    OperationGetTxHeight = 0,
    /// 根据txid判断tx是否存在
    OperationTxExists = 1,
    /// 根据txid查询tx确认时间
    OperationGetTxConfirmedTime = 2,
}
impl OperationByTxId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationByTxId::OperationGetTxHeight => "OperationGetTxHeight",
            OperationByTxId::OperationTxExists => "OperationTxExists",
            OperationByTxId::OperationGetTxConfirmedTime => "OperationGetTxConfirmedTime",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OperationGetTxHeight" => Some(Self::OperationGetTxHeight),
            "OperationTxExists" => Some(Self::OperationTxExists),
            "OperationGetTxConfirmedTime" => Some(Self::OperationGetTxConfirmedTime),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RegisterStatus {
    /// 注册成功
    Success = 0,
    /// 注册失败,有同一条链其他节点正在注册,可以稍后重试
    Conflict = 1,
}
impl RegisterStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RegisterStatus::Success => "RegisterStatusSuccess",
            RegisterStatus::Conflict => "RegisterStatusConflict",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RegisterStatusSuccess" => Some(Self::Success),
            "RegisterStatusConflict" => Some(Self::Conflict),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ArchiveStatus {
    /// 归档区块失败
    Failed = 0,
    /// 区块正确,且已经在归档中心存在
    HasArchived = 1,
    /// 传输区块正足额,且这个区块正确写入到了归档中心
    Success = 2,
}
impl ArchiveStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ArchiveStatus::Failed => "ArchiveStatusFailed",
            ArchiveStatus::HasArchived => "ArchiveStatusHasArchived",
            ArchiveStatus::Success => "ArchiveStatusSuccess",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ArchiveStatusFailed" => Some(Self::Failed),
            "ArchiveStatusHasArchived" => Some(Self::HasArchived),
            "ArchiveStatusSuccess" => Some(Self::Success),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoreDataType {
    /// 1. block file data
    BlockData = 0,
    /// 2. compressed file data
    CompressedData = 1,
    /// 3. decompress file data
    DecompressData = 2,
}
impl StoreDataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StoreDataType::BlockData => "BlockData",
            StoreDataType::CompressedData => "CompressedData",
            StoreDataType::DecompressData => "DecompressData",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BlockData" => Some(Self::BlockData),
            "CompressedData" => Some(Self::CompressedData),
            "DecompressData" => Some(Self::DecompressData),
            _ => None,
        }
    }
}
