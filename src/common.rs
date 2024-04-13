#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    /// smart contract name, set by contract creator, can have multiple versions
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// smart contract version, set by contract creator, name + version should be unique
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// smart contract runtime type, set by contract creator
    #[prost(enumeration = "RuntimeType", tag = "3")]
    pub runtime_type: i32,
    /// contract status
    #[prost(enumeration = "ContractStatus", tag = "4")]
    pub status: i32,
    /// contract creator identity
    #[prost(message, optional, tag = "5")]
    pub creator: ::core::option::Option<super::accesscontrol::MemberFull>,
    /// contract address
    #[prost(string, tag = "6")]
    pub address: ::prost::alloc::string::String,
}
/// smart contract runtime, contains vm type and language type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RuntimeType {
    Invalid = 0,
    /// native implement in chainmaker-go
    Native = 1,
    /// vm-wasmer, language-c++
    Wasmer = 2,
    /// vm-wxvm, language-cpp
    Wxvm = 3,
    /// wasm interpreter in go
    Gasm = 4,
    /// vm-evm
    Evm = 5,
    /// vm-docker, language-golang
    DockerGo = 6,
    /// vm-java, language-java
    Java = 7,
    /// vm-go, language-go
    Go = 8,
}
impl RuntimeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RuntimeType::Invalid => "INVALID",
            RuntimeType::Native => "NATIVE",
            RuntimeType::Wasmer => "WASMER",
            RuntimeType::Wxvm => "WXVM",
            RuntimeType::Gasm => "GASM",
            RuntimeType::Evm => "EVM",
            RuntimeType::DockerGo => "DOCKER_GO",
            RuntimeType::Java => "JAVA",
            RuntimeType::Go => "GO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "NATIVE" => Some(Self::Native),
            "WASMER" => Some(Self::Wasmer),
            "WXVM" => Some(Self::Wxvm),
            "GASM" => Some(Self::Gasm),
            "EVM" => Some(Self::Evm),
            "DOCKER_GO" => Some(Self::DockerGo),
            "JAVA" => Some(Self::Java),
            "GO" => Some(Self::Go),
            _ => None,
        }
    }
}
/// current contract status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractStatus {
    /// normal, can be invoked
    Normal = 0,
    /// frozen, cannot be invoked temporarily
    Frozen = 1,
    /// revoked, cannot be invoked permanently
    Revoked = 2,
}
impl ContractStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractStatus::Normal => "NORMAL",
            ContractStatus::Frozen => "FROZEN",
            ContractStatus::Revoked => "REVOKED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL" => Some(Self::Normal),
            "FROZEN" => Some(Self::Frozen),
            "REVOKED" => Some(Self::Revoked),
            _ => None,
        }
    }
}
/// transaction request proposed by user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRequest {
    /// payload
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<Payload>,
    /// sender account and sender's signature
    #[prost(message, optional, tag = "2")]
    pub sender: ::core::option::Option<EndorsementEntry>,
    /// endorsers account and signatures
    #[prost(message, repeated, tag = "3")]
    pub endorsers: ::prost::alloc::vec::Vec<EndorsementEntry>,
    /// payer account and signature
    #[prost(message, optional, tag = "4")]
    pub payer: ::core::option::Option<EndorsementEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawTxRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub raw_tx: ::prost::alloc::vec::Vec<u8>,
}
/// transaction payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    /// blockchain identifier
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// transaction type
    #[prost(enumeration = "TxType", tag = "2")]
    pub tx_type: i32,
    /// transaction id set by sender, should be unique
    #[prost(string, tag = "3")]
    pub tx_id: ::prost::alloc::string::String,
    /// transaction timestamp, in unix timestamp format, seconds
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
    /// expiration timestamp in unix timestamp format
    /// after that the transaction is invalid if it is not included in block yet
    #[prost(int64, tag = "5")]
    pub expiration_time: i64,
    /// smart contract name
    #[prost(string, tag = "6")]
    pub contract_name: ::prost::alloc::string::String,
    /// invoke method
    #[prost(string, tag = "7")]
    pub method: ::prost::alloc::string::String,
    /// invoke parameters in k-v format
    #[prost(message, repeated, tag = "8")]
    pub parameters: ::prost::alloc::vec::Vec<KeyValuePair>,
    /// sequence number, default is 0
    #[prost(uint64, tag = "9")]
    pub sequence: u64,
    /// transaction limitation
    #[prost(message, optional, tag = "10")]
    pub limit: ::core::option::Option<Limit>,
}
/// endorsement info, including a signer and his signature
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndorsementEntry {
    /// signer
    #[prost(message, optional, tag = "1")]
    pub signer: ::core::option::Option<super::accesscontrol::Member>,
    /// signature
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// a k-v pair
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValuePair {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Limit defines transaction limitation, Limit as a message for easy expansion
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Limit {
    /// gas limit
    #[prost(uint64, tag = "1")]
    pub gas_limit: u64,
}
/// transaction type definition
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TxType {
    /// call a pre created contract, tx included in block
    InvokeContract = 0,
    /// query a pre-created  contract, tx not included in block
    QueryContract = 1,
    /// subscribe block info,tx info and contract info. tx not included in block
    Subscribe = 2,
    /// archive/restore block, tx not included in block
    Archive = 3,
}
impl TxType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TxType::InvokeContract => "INVOKE_CONTRACT",
            TxType::QueryContract => "QUERY_CONTRACT",
            TxType::Subscribe => "SUBSCRIBE",
            TxType::Archive => "ARCHIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVOKE_CONTRACT" => Some(Self::InvokeContract),
            "QUERY_CONTRACT" => Some(Self::QueryContract),
            "SUBSCRIBE" => Some(Self::Subscribe),
            "ARCHIVE" => Some(Self::Archive),
            _ => None,
        }
    }
}
/// tx request - tx response, only for RPC response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResponse {
    /// response code
    #[prost(enumeration = "TxStatusCode", tag = "1")]
    pub code: i32,
    /// response message
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// returned data, set in smart contract
    #[prost(message, optional, tag = "3")]
    pub contract_result: ::core::option::Option<ContractResult>,
    /// tx id of request
    #[prost(string, tag = "4")]
    pub tx_id: ::prost::alloc::string::String,
    /// async tx mode: tx timestamp is zero
    /// sync tx mode: tx timestamp is TxRequest.Payload.Timestamp
    #[prost(int64, tag = "5")]
    pub tx_timestamp: i64,
    /// async tx mode: tx block height is zero
    /// sync tx mode: tx block height is the height of block which this tx was packaged
    #[prost(uint64, tag = "6")]
    pub tx_block_height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeResult {
    /// when TxType == SUBSCRIBE_BLOCK_INFO, data type is pb.BlockInfo;
    /// when TxType == SUBSCRIBE_TX_INFO，data type is pb.Transaction.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// tx result, part of a transaction in block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Result {
    /// response code
    #[prost(enumeration = "TxStatusCode", tag = "1")]
    pub code: i32,
    /// returned data, set in smart contract
    #[prost(message, optional, tag = "2")]
    pub contract_result: ::core::option::Option<ContractResult>,
    /// hash of the transaction's read-write set
    #[prost(bytes = "vec", tag = "3")]
    pub rw_set_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
}
/// invoke user contract method return UserContractReturnPayload
/// Unmarshal from TransactResult.TxResponse.payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractResult {
    /// user contract defined return code, 0-ok, >0 user define error code. for example, insufficient balance in token transfer
    #[prost(uint32, tag = "1")]
    pub code: u32,
    /// user contract defined result
    #[prost(bytes = "vec", tag = "2")]
    pub result: ::prost::alloc::vec::Vec<u8>,
    /// user contract defined result message
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    /// gas used by current contract(include contract call)
    #[prost(uint64, tag = "4")]
    pub gas_used: u64,
    /// contract events
    #[prost(message, repeated, tag = "5")]
    pub contract_event: ::prost::alloc::vec::Vec<ContractEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateGetContract {
    #[prost(bytes = "vec", tag = "1")]
    pub contract_code: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub gas_limit: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrSlice {
    #[prost(string, repeated, tag = "1")]
    pub str_arr: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// certificate collection
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertInfos {
    #[prost(message, repeated, tag = "1")]
    pub cert_infos: ::prost::alloc::vec::Vec<CertInfo>,
}
/// certificate information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertInfo {
    /// certificate hash
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// certificate contents
    #[prost(bytes = "vec", tag = "2")]
    pub cert: ::prost::alloc::vec::Vec<u8>,
}
/// contract event saved in block chain
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractEvent {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub contract_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub contract_version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub event_data: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// contract event published to user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractEventInfo {
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub topic: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub event_index: u32,
    #[prost(string, tag = "6")]
    pub contract_name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub contract_version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "8")]
    pub event_data: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// contract event published to user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractEventInfoList {
    #[prost(message, repeated, tag = "1")]
    pub contract_events: ::prost::alloc::vec::Vec<ContractEventInfo>,
}
/// alias info
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AliasInfo {
    #[prost(string, tag = "1")]
    pub alias: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub now_cert: ::core::option::Option<AliasCertInfo>,
    #[prost(message, repeated, tag = "3")]
    pub his_certs: ::prost::alloc::vec::Vec<AliasCertInfo>,
}
/// alias info
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AliasInfos {
    #[prost(message, repeated, tag = "1")]
    pub alias_infos: ::prost::alloc::vec::Vec<AliasInfo>,
}
/// cert info
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AliasCertInfo {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub cert: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub block_height: u64,
}
/// TxStatusCode describes the tx status in tx result
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TxStatusCode {
    Success = 0,
    Timeout = 1,
    InvalidParameter = 2,
    NoPermission = 3,
    ContractFail = 4,
    InternalError = 5,
    InvalidContractTransactionType = 10,
    InvalidContractParameterContractName = 11,
    InvalidContractParameterMethod = 12,
    InvalidContractParameterInitMethod = 13,
    InvalidContractParameterUpgradeMethod = 14,
    InvalidContractParameterByteCode = 15,
    InvalidContractParameterRuntimeType = 16,
    InvalidContractParameterVersion = 17,
    GetFromTxContextFailed = 20,
    PutIntoTxContextFailed = 21,
    ContractVersionExistFailed = 22,
    ContractVersionNotExistFailed = 23,
    ContractByteCodeNotExistFailed = 24,
    MarshalSenderFailed = 25,
    InvokeInitMethodFailed = 26,
    InvokeUpgradeMethodFailed = 27,
    CreateRuntimeInstanceFailed = 28,
    UnmarshalCreatorFailed = 29,
    UnmarshalSenderFailed = 30,
    GetSenderPkFailed = 31,
    GetCreatorPkFailed = 32,
    GetCreatorFailed = 33,
    GetCreatorCertFailed = 34,
    GetSenderCertFailed = 35,
    ContractFreezeFailed = 36,
    ContractTooDeepFailed = 37,
    ContractRevokeFailed = 38,
    ContractInvokeMethodFailed = 39,
    ArchivedTx = 40,
    ArchivedBlock = 41,
    GasBalanceNotEnoughFailed = 42,
    GasLimitNotSet = 43,
    GasLimitTooSmall = 44,
    /// add for optimized charging gas return message
    GetAccountBalanceFailed = 45,
    ParseAccountBalanceFailed = 46,
    GetAccountStatusFailed = 47,
    AccountStatusFrozen = 48,
}
impl TxStatusCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TxStatusCode::Success => "SUCCESS",
            TxStatusCode::Timeout => "TIMEOUT",
            TxStatusCode::InvalidParameter => "INVALID_PARAMETER",
            TxStatusCode::NoPermission => "NO_PERMISSION",
            TxStatusCode::ContractFail => "CONTRACT_FAIL",
            TxStatusCode::InternalError => "INTERNAL_ERROR",
            TxStatusCode::InvalidContractTransactionType => "INVALID_CONTRACT_TRANSACTION_TYPE",
            TxStatusCode::InvalidContractParameterContractName => {
                "INVALID_CONTRACT_PARAMETER_CONTRACT_NAME"
            }
            TxStatusCode::InvalidContractParameterMethod => "INVALID_CONTRACT_PARAMETER_METHOD",
            TxStatusCode::InvalidContractParameterInitMethod => {
                "INVALID_CONTRACT_PARAMETER_INIT_METHOD"
            }
            TxStatusCode::InvalidContractParameterUpgradeMethod => {
                "INVALID_CONTRACT_PARAMETER_UPGRADE_METHOD"
            }
            TxStatusCode::InvalidContractParameterByteCode => {
                "INVALID_CONTRACT_PARAMETER_BYTE_CODE"
            }
            TxStatusCode::InvalidContractParameterRuntimeType => {
                "INVALID_CONTRACT_PARAMETER_RUNTIME_TYPE"
            }
            TxStatusCode::InvalidContractParameterVersion => "INVALID_CONTRACT_PARAMETER_VERSION",
            TxStatusCode::GetFromTxContextFailed => "GET_FROM_TX_CONTEXT_FAILED",
            TxStatusCode::PutIntoTxContextFailed => "PUT_INTO_TX_CONTEXT_FAILED",
            TxStatusCode::ContractVersionExistFailed => "CONTRACT_VERSION_EXIST_FAILED",
            TxStatusCode::ContractVersionNotExistFailed => "CONTRACT_VERSION_NOT_EXIST_FAILED",
            TxStatusCode::ContractByteCodeNotExistFailed => "CONTRACT_BYTE_CODE_NOT_EXIST_FAILED",
            TxStatusCode::MarshalSenderFailed => "MARSHAL_SENDER_FAILED",
            TxStatusCode::InvokeInitMethodFailed => "INVOKE_INIT_METHOD_FAILED",
            TxStatusCode::InvokeUpgradeMethodFailed => "INVOKE_UPGRADE_METHOD_FAILED",
            TxStatusCode::CreateRuntimeInstanceFailed => "CREATE_RUNTIME_INSTANCE_FAILED",
            TxStatusCode::UnmarshalCreatorFailed => "UNMARSHAL_CREATOR_FAILED",
            TxStatusCode::UnmarshalSenderFailed => "UNMARSHAL_SENDER_FAILED",
            TxStatusCode::GetSenderPkFailed => "GET_SENDER_PK_FAILED",
            TxStatusCode::GetCreatorPkFailed => "GET_CREATOR_PK_FAILED",
            TxStatusCode::GetCreatorFailed => "GET_CREATOR_FAILED",
            TxStatusCode::GetCreatorCertFailed => "GET_CREATOR_CERT_FAILED",
            TxStatusCode::GetSenderCertFailed => "GET_SENDER_CERT_FAILED",
            TxStatusCode::ContractFreezeFailed => "CONTRACT_FREEZE_FAILED",
            TxStatusCode::ContractTooDeepFailed => "CONTRACT_TOO_DEEP_FAILED",
            TxStatusCode::ContractRevokeFailed => "CONTRACT_REVOKE_FAILED",
            TxStatusCode::ContractInvokeMethodFailed => "CONTRACT_INVOKE_METHOD_FAILED",
            TxStatusCode::ArchivedTx => "ARCHIVED_TX",
            TxStatusCode::ArchivedBlock => "ARCHIVED_BLOCK",
            TxStatusCode::GasBalanceNotEnoughFailed => "GAS_BALANCE_NOT_ENOUGH_FAILED",
            TxStatusCode::GasLimitNotSet => "GAS_LIMIT_NOT_SET",
            TxStatusCode::GasLimitTooSmall => "GAS_LIMIT_TOO_SMALL",
            TxStatusCode::GetAccountBalanceFailed => "GET_ACCOUNT_BALANCE_FAILED",
            TxStatusCode::ParseAccountBalanceFailed => "PARSE_ACCOUNT_BALANCE_FAILED",
            TxStatusCode::GetAccountStatusFailed => "GET_ACCOUNT_STATUS_FAILED",
            TxStatusCode::AccountStatusFrozen => "ACCOUNT_STATUS_FROZEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCESS" => Some(Self::Success),
            "TIMEOUT" => Some(Self::Timeout),
            "INVALID_PARAMETER" => Some(Self::InvalidParameter),
            "NO_PERMISSION" => Some(Self::NoPermission),
            "CONTRACT_FAIL" => Some(Self::ContractFail),
            "INTERNAL_ERROR" => Some(Self::InternalError),
            "INVALID_CONTRACT_TRANSACTION_TYPE" => Some(Self::InvalidContractTransactionType),
            "INVALID_CONTRACT_PARAMETER_CONTRACT_NAME" => {
                Some(Self::InvalidContractParameterContractName)
            }
            "INVALID_CONTRACT_PARAMETER_METHOD" => Some(Self::InvalidContractParameterMethod),
            "INVALID_CONTRACT_PARAMETER_INIT_METHOD" => {
                Some(Self::InvalidContractParameterInitMethod)
            }
            "INVALID_CONTRACT_PARAMETER_UPGRADE_METHOD" => {
                Some(Self::InvalidContractParameterUpgradeMethod)
            }
            "INVALID_CONTRACT_PARAMETER_BYTE_CODE" => Some(Self::InvalidContractParameterByteCode),
            "INVALID_CONTRACT_PARAMETER_RUNTIME_TYPE" => {
                Some(Self::InvalidContractParameterRuntimeType)
            }
            "INVALID_CONTRACT_PARAMETER_VERSION" => Some(Self::InvalidContractParameterVersion),
            "GET_FROM_TX_CONTEXT_FAILED" => Some(Self::GetFromTxContextFailed),
            "PUT_INTO_TX_CONTEXT_FAILED" => Some(Self::PutIntoTxContextFailed),
            "CONTRACT_VERSION_EXIST_FAILED" => Some(Self::ContractVersionExistFailed),
            "CONTRACT_VERSION_NOT_EXIST_FAILED" => Some(Self::ContractVersionNotExistFailed),
            "CONTRACT_BYTE_CODE_NOT_EXIST_FAILED" => Some(Self::ContractByteCodeNotExistFailed),
            "MARSHAL_SENDER_FAILED" => Some(Self::MarshalSenderFailed),
            "INVOKE_INIT_METHOD_FAILED" => Some(Self::InvokeInitMethodFailed),
            "INVOKE_UPGRADE_METHOD_FAILED" => Some(Self::InvokeUpgradeMethodFailed),
            "CREATE_RUNTIME_INSTANCE_FAILED" => Some(Self::CreateRuntimeInstanceFailed),
            "UNMARSHAL_CREATOR_FAILED" => Some(Self::UnmarshalCreatorFailed),
            "UNMARSHAL_SENDER_FAILED" => Some(Self::UnmarshalSenderFailed),
            "GET_SENDER_PK_FAILED" => Some(Self::GetSenderPkFailed),
            "GET_CREATOR_PK_FAILED" => Some(Self::GetCreatorPkFailed),
            "GET_CREATOR_FAILED" => Some(Self::GetCreatorFailed),
            "GET_CREATOR_CERT_FAILED" => Some(Self::GetCreatorCertFailed),
            "GET_SENDER_CERT_FAILED" => Some(Self::GetSenderCertFailed),
            "CONTRACT_FREEZE_FAILED" => Some(Self::ContractFreezeFailed),
            "CONTRACT_TOO_DEEP_FAILED" => Some(Self::ContractTooDeepFailed),
            "CONTRACT_REVOKE_FAILED" => Some(Self::ContractRevokeFailed),
            "CONTRACT_INVOKE_METHOD_FAILED" => Some(Self::ContractInvokeMethodFailed),
            "ARCHIVED_TX" => Some(Self::ArchivedTx),
            "ARCHIVED_BLOCK" => Some(Self::ArchivedBlock),
            "GAS_BALANCE_NOT_ENOUGH_FAILED" => Some(Self::GasBalanceNotEnoughFailed),
            "GAS_LIMIT_NOT_SET" => Some(Self::GasLimitNotSet),
            "GAS_LIMIT_TOO_SMALL" => Some(Self::GasLimitTooSmall),
            "GET_ACCOUNT_BALANCE_FAILED" => Some(Self::GetAccountBalanceFailed),
            "PARSE_ACCOUNT_BALANCE_FAILED" => Some(Self::ParseAccountBalanceFailed),
            "GET_ACCOUNT_STATUS_FAILED" => Some(Self::GetAccountStatusFailed),
            "ACCOUNT_STATUS_FROZEN" => Some(Self::AccountStatusFrozen),
            _ => None,
        }
    }
}
/// key read version
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyVersion {
    /// the transaction identifier that last modified the key
    #[prost(string, tag = "3")]
    pub ref_tx_id: ::prost::alloc::string::String,
    /// the offset of the key in the write set of the transaction, starts from 0
    #[prost(int32, tag = "4")]
    pub ref_offset: i32,
}
/// TxRead describes a read operation on a key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRead {
    /// read key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// the value of the key
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// contract name, used in cross-contract calls
    /// set to null if only the contract in transaction request is called
    #[prost(string, tag = "3")]
    pub contract_name: ::prost::alloc::string::String,
    /// read key version
    #[prost(message, optional, tag = "4")]
    pub version: ::core::option::Option<KeyVersion>,
}
/// TxRead describes a write/delete operation on a key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxWrite {
    /// write key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// write value
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// contract name, used in cross-contract calls
    /// set to null if only the contract in transaction request is called
    #[prost(string, tag = "3")]
    pub contract_name: ::prost::alloc::string::String,
}
/// TxRWSet describes all the operations of a transaction on ledger
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRwSet {
    /// transaction identifier
    #[prost(string, tag = "1")]
    pub tx_id: ::prost::alloc::string::String,
    /// read set
    #[prost(message, repeated, tag = "2")]
    pub tx_reads: ::prost::alloc::vec::Vec<TxRead>,
    /// write set
    #[prost(message, repeated, tag = "3")]
    pub tx_writes: ::prost::alloc::vec::Vec<TxWrite>,
}
/// a transaction includes request and its result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// payload
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<Payload>,
    /// sender account and signature
    #[prost(message, optional, tag = "2")]
    pub sender: ::core::option::Option<EndorsementEntry>,
    /// endorser accounts and signatures
    #[prost(message, repeated, tag = "3")]
    pub endorsers: ::prost::alloc::vec::Vec<EndorsementEntry>,
    /// result of the transaction
    #[prost(message, optional, tag = "4")]
    pub result: ::core::option::Option<Result>,
    /// payer account and signature
    #[prost(message, optional, tag = "5")]
    pub payer: ::core::option::Option<EndorsementEntry>,
}
/// transaction info include transaction and its block height hash and tx index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    /// transaction raw data
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
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
}
/// transaction and read write set
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionWithRwSet {
    /// transaction raw data
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(message, optional, tag = "2")]
    pub rw_set: ::core::option::Option<TxRwSet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfoWithRwSet {
    /// transaction raw data
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
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
    #[prost(message, optional, tag = "6")]
    pub rw_set: ::core::option::Option<TxRwSet>,
}
/// Bird's Nest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BirdsNest {
    /// Bird's Nest config
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<BirdsNestConfig>,
    /// The final height
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// current index
    #[prost(uint32, tag = "3")]
    pub current_index: u32,
    /// A group of cuckoos filter
    #[prost(message, repeated, tag = "4")]
    pub filters: ::prost::alloc::vec::Vec<CuckooFilter>,
}
/// Sharding Bird's Nest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardingBirdsNest {
    #[prost(uint32, tag = "1")]
    pub length: u32,
    /// The final height
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// Bird's Nest config
    #[prost(message, optional, tag = "3")]
    pub config: ::core::option::Option<ShardingBirdsNestConfig>,
}
/// Chain table structure
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CuckooFilter {
    /// The field "cuckoo" is used to hold the serialized data of the cuckoo
    /// Pb limit: The size of bytes cannot be larger than 4 GB
    #[prost(bytes = "vec", tag = "1")]
    pub cuckoo: ::prost::alloc::vec::Vec<u8>,
    /// Carries the ID of the time
    #[prost(bytes = "vec", tag = "2")]
    pub extension: ::prost::alloc::vec::Vec<u8>,
    /// cuckoo configuration
    #[prost(bytes = "vec", tag = "3")]
    pub config: ::prost::alloc::vec::Vec<u8>,
}
/// Sharding bird's Nest configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardingBirdsNestConfig {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub length: u32,
    /// sharding task timeout
    #[prost(int64, tag = "3")]
    pub timeout: i64,
    /// Bird's Nest configuration
    #[prost(message, optional, tag = "4")]
    pub birdsnest: ::core::option::Option<BirdsNestConfig>,
    /// Snapshot config
    #[prost(message, optional, tag = "5")]
    pub snapshot: ::core::option::Option<SnapshotSerializerConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BirdsNestConfig {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub length: u32,
    /// rules config
    #[prost(message, optional, tag = "3")]
    pub rules: ::core::option::Option<RulesConfig>,
    /// Cuckoo config
    #[prost(message, optional, tag = "4")]
    pub cuckoo: ::core::option::Option<CuckooConfig>,
    /// Snapshot config
    #[prost(message, optional, tag = "5")]
    pub snapshot: ::core::option::Option<SnapshotSerializerConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RulesConfig {
    /// absolute expire time second
    #[prost(int64, tag = "1")]
    pub absolute_expire_time: i64,
}
/// Cuckoo config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CuckooConfig {
    #[prost(enumeration = "KeyType", tag = "1")]
    pub key_type: i32,
    /// num of tags for each bucket, which is b in paper. tag is fingerprint, which is f in paper.
    #[prost(uint32, tag = "2")]
    pub tags_per_bucket: u32,
    /// num of bits for each item, which is length of tag(fingerprint)
    #[prost(uint32, tag = "3")]
    pub bits_per_item: u32,
    /// num of keys that filter will store. this value should close to and lower
    /// 					 nextPow2(maxNumKeys/tagsPerBucket) * maxLoadFactor. cause table.NumBuckets is always a power of two
    #[prost(uint32, tag = "4")]
    pub max_num_keys: u32,
    /// has two constant parameters to choose from:
    /// TableTypeSingle normal single table
    /// TableTypePacked packed table, use semi-sort to save 1 bit per item
    #[prost(uint32, tag = "5")]
    pub table_type: u32,
}
/// Snapshot serializer config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotSerializerConfig {
    /// serialization type
    #[prost(enumeration = "SerializeIntervalType", tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub timed: ::core::option::Option<TimedSerializeIntervalConfig>,
    #[prost(message, optional, tag = "3")]
    pub block_height: ::core::option::Option<BlockHeightSerializeIntervalConfig>,
    /// filepath
    #[prost(string, tag = "4")]
    pub path: ::prost::alloc::string::String,
}
/// Timed serialization interval
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimedSerializeIntervalConfig {
    #[prost(int64, tag = "1")]
    pub interval: i64,
}
/// Block height serialization interval
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeightSerializeIntervalConfig {
    #[prost(uint64, tag = "1")]
    pub interval: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    KtDefault = 0,
    KtTimestampKey = 1,
}
impl KeyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyType::KtDefault => "KTDefault",
            KeyType::KtTimestampKey => "KTTimestampKey",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KTDefault" => Some(Self::KtDefault),
            "KTTimestampKey" => Some(Self::KtTimestampKey),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FilterExtensionType {
    FetDefault = 0,
    FetTimestamp = 1,
}
impl FilterExtensionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FilterExtensionType::FetDefault => "FETDefault",
            FilterExtensionType::FetTimestamp => "FETTimestamp",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FETDefault" => Some(Self::FetDefault),
            "FETTimestamp" => Some(Self::FetTimestamp),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RuleType {
    AbsoluteExpireTime = 0,
}
impl RuleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RuleType::AbsoluteExpireTime => "AbsoluteExpireTime",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AbsoluteExpireTime" => Some(Self::AbsoluteExpireTime),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SerializeIntervalType {
    /// Timed serialize type
    Height = 0,
    /// Timed serialize type
    Timed = 1,
    /// Exit serialize type
    Exit = 2,
}
impl SerializeIntervalType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SerializeIntervalType::Height => "Height",
            SerializeIntervalType::Timed => "Timed",
            SerializeIntervalType::Exit => "Exit",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Height" => Some(Self::Height),
            "Timed" => Some(Self::Timed),
            "Exit" => Some(Self::Exit),
            _ => None,
        }
    }
}
/// Block definition
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// header of the block
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<BlockHeader>,
    /// execution sequence of intra block transactions, generated by proposer
    #[prost(message, optional, tag = "2")]
    pub dag: ::core::option::Option<Dag>,
    /// transaction list in this block
    #[prost(message, repeated, tag = "3")]
    pub txs: ::prost::alloc::vec::Vec<Transaction>,
    /// stores the voting information of the current block
    /// not included in block hash value calculation
    #[prost(message, optional, tag = "4")]
    pub additional_data: ::core::option::Option<AdditionalData>,
}
/// block information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfo {
    /// block
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<Block>,
    /// The read/write set list corresponding to the transaction included in the block
    #[prost(message, repeated, tag = "2")]
    pub rwset_list: ::prost::alloc::vec::Vec<TxRwSet>,
}
/// block additional data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalData {
    /// extra data, with map type, excluded in hash calculation
    #[prost(map = "string, bytes", tag = "1")]
    pub extra_data:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
/// tx batch info,saved in additional data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxBatchInfo {
    #[prost(string, repeated, tag = "1")]
    pub batch_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, repeated, tag = "2")]
    pub index: ::prost::alloc::vec::Vec<u32>,
}
/// header of the block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    /// block version
    #[prost(uint32, tag = "1")]
    pub block_version: u32,
    /// config block or normal block or other else
    #[prost(enumeration = "BlockType", tag = "2")]
    pub block_type: i32,
    /// blockchain identifier
    #[prost(string, tag = "3")]
    pub chain_id: ::prost::alloc::string::String,
    /// block height
    #[prost(uint64, tag = "4")]
    pub block_height: u64,
    /// block hash (block identifier)
    #[prost(bytes = "vec", tag = "5")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    /// previous block hash
    #[prost(bytes = "vec", tag = "6")]
    pub pre_block_hash: ::prost::alloc::vec::Vec<u8>,
    /// previous config block height, used to trace and check if chain config is valid
    #[prost(uint64, tag = "7")]
    pub pre_conf_height: u64,
    /// count of transactions
    #[prost(uint32, tag = "8")]
    pub tx_count: u32,
    /// merkle root of transactions
    /// used to verify the existence of this transactions
    #[prost(bytes = "vec", tag = "9")]
    pub tx_root: ::prost::alloc::vec::Vec<u8>,
    /// Save the DAG feature summary, and hash the DAG after Pb serialization
    /// hash of serialized DAG
    #[prost(bytes = "vec", tag = "10")]
    pub dag_hash: ::prost::alloc::vec::Vec<u8>,
    /// The root hash of Merkle tree generated by read_write_set_digest in the result of each transaction in the block
    /// used to verify the read-write set of the block
    #[prost(bytes = "vec", tag = "11")]
    pub rw_set_root: ::prost::alloc::vec::Vec<u8>,
    /// the time stamp of the block
    #[prost(int64, tag = "12")]
    pub block_timestamp: i64,
    /// consensus parameters
    /// used to store information, include in block hash calculation
    #[prost(bytes = "vec", tag = "13")]
    pub consensus_args: ::prost::alloc::vec::Vec<u8>,
    /// proposal node identifier
    #[prost(message, optional, tag = "14")]
    pub proposer: ::core::option::Option<super::accesscontrol::Member>,
    /// signature of proposer
    #[prost(bytes = "vec", tag = "15")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// transaction execution sequence
/// Using adjacency table storage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dag {
    /// sequence number of transaction topological sort
    /// the sequence number of the transaction topological sort associated with the transaction
    #[prost(message, repeated, tag = "2")]
    pub vertexes: ::prost::alloc::vec::Vec<dag::Neighbor>,
}
/// Nested message and enum types in `DAG`.
pub mod dag {
    /// Neighbor node: related transactions with reading and writing conflicts
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Neighbor {
        #[prost(uint32, repeated, tag = "1")]
        pub neighbors: ::prost::alloc::vec::Vec<u32>,
    }
}
/// BlockType specify block pack txs type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockType {
    /// Normal block, pack multi txs into one block
    NormalBlock = 0,
    /// Config block, only include 1 chain config update tx in this block
    ConfigBlock = 1,
    /// Sql Contract init or upgrade block, only include 1 sql contract init or upgrade tx in this block
    ContractMgrBlock = 2,
    /// block.Txs\[0\] is a coinbase tx
    HasCoinbase = 4,
}
impl BlockType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockType::NormalBlock => "NORMAL_BLOCK",
            BlockType::ConfigBlock => "CONFIG_BLOCK",
            BlockType::ContractMgrBlock => "CONTRACT_MGR_BLOCK",
            BlockType::HasCoinbase => "HAS_COINBASE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL_BLOCK" => Some(Self::NormalBlock),
            "CONFIG_BLOCK" => Some(Self::ConfigBlock),
            "CONTRACT_MGR_BLOCK" => Some(Self::ContractMgrBlock),
            "HAS_COINBASE" => Some(Self::HasCoinbase),
            _ => None,
        }
    }
}
