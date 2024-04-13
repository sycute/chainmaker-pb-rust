/// a string k-v pair for config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigKeyValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// ChainConfig
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainConfig {
    /// blockchain identifier
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// blockchain version
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// member type
    #[prost(string, tag = "3")]
    pub auth_type: ::prost::alloc::string::String,
    /// config sequence
    #[prost(uint64, tag = "4")]
    pub sequence: u64,
    /// encryption algorithm related configuration
    #[prost(message, optional, tag = "5")]
    pub crypto: ::core::option::Option<CryptoConfig>,
    /// block related configuration
    #[prost(message, optional, tag = "6")]
    pub block: ::core::option::Option<BlockConfig>,
    /// core module related configuration
    #[prost(message, optional, tag = "7")]
    pub core: ::core::option::Option<CoreConfig>,
    /// consensus related configuration
    #[prost(message, optional, tag = "8")]
    pub consensus: ::core::option::Option<ConsensusConfig>,
    /// trusted root related configuration
    /// for alliance members, the initial member's root info of the consortium; for public chain, there is no need to configure
    /// Key: node_id; value: address, node public key / CA certificate
    #[prost(message, repeated, tag = "9")]
    pub trust_roots: ::prost::alloc::vec::Vec<TrustRootConfig>,
    #[prost(message, repeated, tag = "10")]
    pub trust_members: ::prost::alloc::vec::Vec<TrustMemberConfig>,
    /// permission related configuration
    #[prost(message, repeated, tag = "11")]
    pub resource_policies: ::prost::alloc::vec::Vec<ResourcePolicy>,
    #[prost(message, optional, tag = "12")]
    pub contract: ::core::option::Option<ContractConfig>,
    /// snapshot module related configuration
    #[prost(message, optional, tag = "13")]
    pub snapshot: ::core::option::Option<SnapshotConfig>,
    /// scheduler module related configuration
    #[prost(message, optional, tag = "14")]
    pub scheduler: ::core::option::Option<SchedulerConfig>,
    /// tx sim context module related configuration
    #[prost(message, optional, tag = "15")]
    pub context: ::core::option::Option<ContextConfig>,
    /// disabled native contracts list for permission control purposes
    #[prost(string, repeated, tag = "16")]
    pub disabled_native_contract: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// gas account config
    #[prost(message, optional, tag = "18")]
    pub account_config: ::core::option::Option<GasAccountConfig>,
    /// vm config
    #[prost(message, optional, tag = "17")]
    pub vm: ::core::option::Option<Vm>,
}
/// specific permission configuration structure
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourcePolicy {
    /// resource name
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// policy(permission)
    #[prost(message, optional, tag = "2")]
    pub policy: ::core::option::Option<super::accesscontrol::Policy>,
}
/// encryption configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoConfig {
    /// enable Transaction timestamp verification or Not
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
/// blockConfig
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockConfig {
    /// enable transaction timestamp verification or Not
    #[prost(bool, tag = "1")]
    pub tx_timestamp_verify: bool,
    /// expiration time of transaction timestamp (seconds)
    #[prost(uint32, tag = "2")]
    pub tx_timeout: u32,
    /// maximum number of transactions in a block
    #[prost(uint32, tag = "3")]
    pub block_tx_capacity: u32,
    /// maximum block size, in MB
    #[prost(uint32, tag = "4")]
    pub block_size: u32,
    /// block proposing interval, in ms
    #[prost(uint32, tag = "5")]
    pub block_interval: u32,
    /// maximum size of transaction's parameter, in MB
    #[prost(uint32, tag = "6")]
    pub tx_parameter_size: u32,
    /// enable block timestamp verification or Not
    #[prost(bool, tag = "7")]
    pub block_timestamp_verify: bool,
    /// expiration time of block timestamp (seconds)
    #[prost(uint32, tag = "8")]
    pub block_timeout: u32,
}
/// Scheduler configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulerConfig {
    /// for evidence contract
    #[prost(bool, tag = "1")]
    pub enable_evidence: bool,
}
/// gas account config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasAccountConfig {
    /// for admin address
    #[prost(string, tag = "1")]
    pub gas_admin_address: ::prost::alloc::string::String,
    /// for admin gas count
    #[prost(uint32, tag = "2")]
    pub gas_count: u32,
    /// for gas manager
    #[prost(bool, tag = "3")]
    pub enable_gas: bool,
    /// default gas value for invoke user contract
    #[prost(uint64, tag = "4")]
    pub default_gas: u64,
    /// default gas price per `byte` for invoke user contract
    #[prost(float, tag = "5")]
    pub default_gas_price: f32,
    /// default gas value for install/upgrade user contract
    #[prost(uint64, tag = "6")]
    pub install_base_gas: u64,
    /// default gas price per `byte` for install/upgrade user contract
    #[prost(float, tag = "7")]
    pub install_gas_price: f32,
}
/// Snapshot configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotConfig {
    /// for the evidence contract
    #[prost(bool, tag = "1")]
    pub enable_evidence: bool,
}
/// TxSimContext configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextConfig {
    /// for the evidence contract
    #[prost(bool, tag = "1")]
    pub enable_evidence: bool,
}
/// consensus message turbo configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusTurboConfig {
    /// switch of consensus message turbo
    #[prost(bool, tag = "1")]
    pub consensus_message_turbo: bool,
    /// retry time of get tx by txIds from txpool
    #[prost(uint64, tag = "2")]
    pub retry_time: u64,
    /// the interval of retry get tx by txIds from txpool(ms)
    #[prost(uint64, tag = "3")]
    pub retry_interval: u64,
}
/// core module related configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoreConfig {
    /// [0, 60], the time when the transaction scheduler gets the transaction from the transaction pool to schedule
    #[prost(uint64, tag = "1")]
    pub tx_scheduler_timeout: u64,
    /// [0, 60], the time-out for verification after the transaction scheduler obtains the transaction from the block
    #[prost(uint64, tag = "2")]
    pub tx_scheduler_validate_timeout: u64,
    /// the configuration of consensus message turbo
    #[prost(message, optional, tag = "3")]
    pub consensus_turbo_config: ::core::option::Option<ConsensusTurboConfig>,
    /// enable sender group, used for handling txs with sender conflicts efficiently
    #[prost(bool, tag = "4")]
    pub enable_sender_group: bool,
    /// enable conflicts bit window, used for dynamic tuning the capacity of tx execution goroutine pool
    #[prost(bool, tag = "5")]
    pub enable_conflicts_bit_window: bool,
    /// enable optimize charge gas for the same account transactions
    #[prost(bool, tag = "6")]
    pub enable_optimize_charge_gas: bool,
}
/// consensus module related configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusConfig {
    /// consensus type
    #[prost(enumeration = "super::consensus::ConsensusType", tag = "1")]
    pub r#type: i32,
    /// organization list of nodes
    #[prost(message, repeated, tag = "2")]
    pub nodes: ::prost::alloc::vec::Vec<OrgConfig>,
    /// expand the field, record the difficulty, reward and other consensus algorithm configuration
    #[prost(message, repeated, tag = "3")]
    pub ext_config: ::prost::alloc::vec::Vec<ConfigKeyValue>,
    /// Initialize the configuration of DPOS
    #[prost(message, repeated, tag = "4")]
    pub dpos_config: ::prost::alloc::vec::Vec<ConfigKeyValue>,
}
/// organization related configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgConfig {
    /// organization identifier
    #[prost(string, tag = "1")]
    pub org_id: ::prost::alloc::string::String,
    /// address list owned by the organization
    /// Deprecated , replace by node_id
    #[prost(string, repeated, tag = "2")]
    pub address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// node id list owned by the organization
    #[prost(string, repeated, tag = "3")]
    pub node_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// trusted root related configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustRootConfig {
    /// oranization ideftifier
    #[prost(string, tag = "1")]
    pub org_id: ::prost::alloc::string::String,
    /// root certificate / public key
    #[prost(string, repeated, tag = "2")]
    pub root: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractConfig {
    #[prost(bool, tag = "1")]
    pub enable_sql_support: bool,
    /// disabled native contracts list for permission control purposes
    #[prost(string, repeated, tag = "2")]
    pub disabled_native_contract: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If it is true, Only creators are allowed to upgrade contract.
    #[prost(bool, tag = "3")]
    pub only_creator_can_upgrade: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustMemberConfig {
    /// member info
    #[prost(string, tag = "1")]
    pub member_info: ::prost::alloc::string::String,
    /// oranization ideftifier
    #[prost(string, tag = "2")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub role: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub node_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vm {
    #[prost(string, repeated, tag = "1")]
    pub support_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "AddrType", tag = "2")]
    pub addr_type: i32,
    #[prost(message, optional, tag = "3")]
    pub native: ::core::option::Option<VmNative>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmNative {
    #[prost(message, optional, tag = "1")]
    pub multisign: ::core::option::Option<MultiSign>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiSign {
    /// enable multi sign execute contract from sender
    #[prost(bool, tag = "1")]
    pub enable_manual_run: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AddrType {
    Chainmaker = 0,
    Zxl = 1,
    Ethereum = 2,
}
impl AddrType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AddrType::Chainmaker => "CHAINMAKER",
            AddrType::Zxl => "ZXL",
            AddrType::Ethereum => "ETHEREUM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHAINMAKER" => Some(Self::Chainmaker),
            "ZXL" => Some(Self::Zxl),
            "ETHEREUM" => Some(Self::Ethereum),
            _ => None,
        }
    }
}
/// rquest for debug configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugConfigRequest {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<ConfigKeyValue>,
}
/// Rrsponse for debug configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugConfigResponse {
    /// 0 success
    /// 1 fail
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// failure message
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// request for check new block configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNewBlockChainConfigRequest {}
/// response for check new block configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNewBlockChainConfigResponse {
    /// 0 success
    /// 1 fail
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// failure message
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// request for log level
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogLevelsRequest {}
/// response for log level
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogLevelsResponse {
    /// 0 success
    /// 1 fail
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// failure message
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// BirdsNest Bird's Nest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxFilterConfig {
    /// Transaction filter type
    #[prost(enumeration = "TxFilterType", tag = "1")]
    pub r#type: i32,
    /// Bird's nest configuration
    #[prost(message, optional, tag = "2")]
    pub birds_nest: ::core::option::Option<super::common::BirdsNestConfig>,
    /// Sharding bird's nest configuration
    #[prost(message, optional, tag = "3")]
    pub sharding_birds_nest: ::core::option::Option<super::common::ShardingBirdsNestConfig>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TxFilterType {
    None = 0,
    BirdsNest = 1,
    Map = 2,
    ShardingBirdsNest = 3,
}
impl TxFilterType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TxFilterType::None => "None",
            TxFilterType::BirdsNest => "BirdsNest",
            TxFilterType::Map => "Map",
            TxFilterType::ShardingBirdsNest => "ShardingBirdsNest",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "None" => Some(Self::None),
            "BirdsNest" => Some(Self::BirdsNest),
            "Map" => Some(Self::Map),
            "ShardingBirdsNest" => Some(Self::ShardingBirdsNest),
            _ => None,
        }
    }
}
/// Request for chainmaker version
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainMakerVersionRequest {}
/// Response for chainmaker version
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainMakerVersionResponse {
    /// 0 success
    /// 1 fail
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
