#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SystemContract {
    /// system chain configuration contract
    /// used to add, delete and change the chain configuration
    ChainConfig = 0,
    /// system chain query contract
    /// used to query the configuration on the chain
    ChainQuery = 1,
    /// system certificate storage contract
    /// used to manage certificates
    CertManage = 2,
    /// governance contract
    Governance = 3,
    /// multi signature contract on chain
    MultiSign = 4,
    /// manage user contract
    ContractManage = 5,
    /// private compute contract
    PrivateCompute = 6,
    /// erc20 contract for DPoS
    DposErc20 = 7,
    /// stake contract for dpos
    DposStake = 8,
    /// subscribe block info,tx info and contract info.
    SubscribeManage = 9,
    /// archive/restore block
    ArchiveManage = 10,
    /// cross chain transaction system contract
    CrossTransaction = 11,
    /// pubkey manage system contract
    PubkeyManage = 12,
    /// account manager system contract
    AccountManager = 13,
    /// relay cross system contract
    RelayCross = 17,
    /// transaction manager system contract
    TransactionManager = 18,
    /// for test or debug contract code
    T = 99,
}
impl SystemContract {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SystemContract::ChainConfig => "CHAIN_CONFIG",
            SystemContract::ChainQuery => "CHAIN_QUERY",
            SystemContract::CertManage => "CERT_MANAGE",
            SystemContract::Governance => "GOVERNANCE",
            SystemContract::MultiSign => "MULTI_SIGN",
            SystemContract::ContractManage => "CONTRACT_MANAGE",
            SystemContract::PrivateCompute => "PRIVATE_COMPUTE",
            SystemContract::DposErc20 => "DPOS_ERC20",
            SystemContract::DposStake => "DPOS_STAKE",
            SystemContract::SubscribeManage => "SUBSCRIBE_MANAGE",
            SystemContract::ArchiveManage => "ARCHIVE_MANAGE",
            SystemContract::CrossTransaction => "CROSS_TRANSACTION",
            SystemContract::PubkeyManage => "PUBKEY_MANAGE",
            SystemContract::AccountManager => "ACCOUNT_MANAGER",
            SystemContract::RelayCross => "RELAY_CROSS",
            SystemContract::TransactionManager => "TRANSACTION_MANAGER",
            SystemContract::T => "T",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHAIN_CONFIG" => Some(Self::ChainConfig),
            "CHAIN_QUERY" => Some(Self::ChainQuery),
            "CERT_MANAGE" => Some(Self::CertManage),
            "GOVERNANCE" => Some(Self::Governance),
            "MULTI_SIGN" => Some(Self::MultiSign),
            "CONTRACT_MANAGE" => Some(Self::ContractManage),
            "PRIVATE_COMPUTE" => Some(Self::PrivateCompute),
            "DPOS_ERC20" => Some(Self::DposErc20),
            "DPOS_STAKE" => Some(Self::DposStake),
            "SUBSCRIBE_MANAGE" => Some(Self::SubscribeManage),
            "ARCHIVE_MANAGE" => Some(Self::ArchiveManage),
            "CROSS_TRANSACTION" => Some(Self::CrossTransaction),
            "PUBKEY_MANAGE" => Some(Self::PubkeyManage),
            "ACCOUNT_MANAGER" => Some(Self::AccountManager),
            "RELAY_CROSS" => Some(Self::RelayCross),
            "TRANSACTION_MANAGER" => Some(Self::TransactionManager),
            "T" => Some(Self::T),
            _ => None,
        }
    }
}
/// methods of chain config contract
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChainConfigFunction {
    /// get chain configuration
    GetChainConfig = 0,
    /// get the latest configuration block
    /// the incoming block height must exist in the database
    GetChainConfigAt = 1,
    /// update core
    CoreUpdate = 2,
    /// update block
    BlockUpdate = 3,
    /// add trusted certificate (org_id and root)
    TrustRootAdd = 4,
    /// \[self\] modify an individual's own trusted root certificate [org_id must exist in the original trust_roots,
    /// and the new root certificate must be different from other certificates]
    TrustRootUpdate = 5,
    /// delete trusted root certificate [org_ ID should be in trust_ The nodes in nodes need to be deleted]
    TrustRootDelete = 6,
    /// organization add node address
    /// org_id must already exist in nodes，you can add addresses in batches
    /// the parameter is addresses. Single addresses are separated by ","
    /// ip+port and peerid cannot be repeated
    /// Deprecated , replace by NODE_ID_ADD
    NodeAddrAdd = 7,
    /// \[self\]the organization updates an address
    /// [org_id and address must already exist in nodes, new_address is the new address. ip+port and peerId cannot be duplicated]
    /// Deprecated , replace by NODE_ID_UPDATE
    NodeAddrUpdate = 8,
    /// organization delete node address [org_id and address must already exist in nodes]
    /// Deprecated , replace by NODE_ID_DELETE
    NodeAddrDelete = 9,
    /// organization add node address in batches \[org_id在nodes不存在，批量添加地址，参数为node_ids，单地址用逗号","隔开。nodeId不能重复\]
    NodeOrgAdd = 10,
    /// organization update
    /// org_id must already exist in nodes，the parameter is addresses，Single addresses are separated by ","
    /// ip+port and peerid cannot be repeated
    NodeOrgUpdate = 11,
    /// organization delete, org_id must already exist in nodes
    NodeOrgDelete = 12,
    /// add consensus parameters, key is not exit in ext_config
    ConsensusExtAdd = 13,
    /// update onsensus parameters, key exit in ext_config
    ConsensusExtUpdate = 14,
    /// delete onsensus parameters, key exit in ext_config
    ConsensusExtDelete = 15,
    /// add permission
    PermissionAdd = 16,
    /// update permission
    PermissionUpdate = 17,
    /// delete permission
    PermissionDelete = 18,
    /// organization add node_id
    /// org_id must already exist in nodes，you can add node_id in batches
    /// the parameter is node_ids. Single node_ids are separated by ","
    /// node_id cannot be repeated
    NodeIdAdd = 19,
    /// \[self\]the organization updates a node_ids
    /// [org_id and node_ids must already exist in nodes, new_node_id is the new node_id. node_id cannot be duplicated]
    NodeIdUpdate = 20,
    /// organization delete node_id [org_id and node_id must already exist in nodes]
    NodeIdDelete = 21,
    /// add trusted member (org_id signcert role  node_id)
    TrustMemberAdd = 22,
    /// \[self\] modify an individual's own trusted member [node_id must exist in the original trust_members,
    /// and the new trust member must be different from other trust members]
    TrustMemberUpdate = 23,
    /// delete trusted member certificate [node_ ID should be in trust_ The nodes in nodes need to be deleted]
    TrustMemberDelete = 24,
    /// alter address type
    AlterAddrType = 25,
    /// able or enable gas calc
    EnableOrDisableGas = 26,
    /// set invoke base gas
    SetInvokeBaseGas = 27,
    /// set account manager admin
    SetAccountManagerAdmin = 28,
    /// list permissions
    PermissionList = 29,
    /// update version
    UpdateVersion = 30,
    /// update `enable_manual_run` flag of multi sign
    MultiSignEnableManualRun = 31,
    /// enable only_creator_can_upgrade
    EnableOnlyCreatorUpgrade = 32,
    /// disable only_creator_can_upgrade
    DisableOnlyCreatorUpgrade = 33,
    /// set invoke gas price, continued with NO. `27`
    SetInvokeGasPrice = 34,
    /// set install base price
    SetInstallBaseGas = 35,
    /// set install gas price
    SetInstallGasPrice = 36,
}
impl ChainConfigFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChainConfigFunction::GetChainConfig => "GET_CHAIN_CONFIG",
            ChainConfigFunction::GetChainConfigAt => "GET_CHAIN_CONFIG_AT",
            ChainConfigFunction::CoreUpdate => "CORE_UPDATE",
            ChainConfigFunction::BlockUpdate => "BLOCK_UPDATE",
            ChainConfigFunction::TrustRootAdd => "TRUST_ROOT_ADD",
            ChainConfigFunction::TrustRootUpdate => "TRUST_ROOT_UPDATE",
            ChainConfigFunction::TrustRootDelete => "TRUST_ROOT_DELETE",
            ChainConfigFunction::NodeAddrAdd => "NODE_ADDR_ADD",
            ChainConfigFunction::NodeAddrUpdate => "NODE_ADDR_UPDATE",
            ChainConfigFunction::NodeAddrDelete => "NODE_ADDR_DELETE",
            ChainConfigFunction::NodeOrgAdd => "NODE_ORG_ADD",
            ChainConfigFunction::NodeOrgUpdate => "NODE_ORG_UPDATE",
            ChainConfigFunction::NodeOrgDelete => "NODE_ORG_DELETE",
            ChainConfigFunction::ConsensusExtAdd => "CONSENSUS_EXT_ADD",
            ChainConfigFunction::ConsensusExtUpdate => "CONSENSUS_EXT_UPDATE",
            ChainConfigFunction::ConsensusExtDelete => "CONSENSUS_EXT_DELETE",
            ChainConfigFunction::PermissionAdd => "PERMISSION_ADD",
            ChainConfigFunction::PermissionUpdate => "PERMISSION_UPDATE",
            ChainConfigFunction::PermissionDelete => "PERMISSION_DELETE",
            ChainConfigFunction::NodeIdAdd => "NODE_ID_ADD",
            ChainConfigFunction::NodeIdUpdate => "NODE_ID_UPDATE",
            ChainConfigFunction::NodeIdDelete => "NODE_ID_DELETE",
            ChainConfigFunction::TrustMemberAdd => "TRUST_MEMBER_ADD",
            ChainConfigFunction::TrustMemberUpdate => "TRUST_MEMBER_UPDATE",
            ChainConfigFunction::TrustMemberDelete => "TRUST_MEMBER_DELETE",
            ChainConfigFunction::AlterAddrType => "ALTER_ADDR_TYPE",
            ChainConfigFunction::EnableOrDisableGas => "ENABLE_OR_DISABLE_GAS",
            ChainConfigFunction::SetInvokeBaseGas => "SET_INVOKE_BASE_GAS",
            ChainConfigFunction::SetAccountManagerAdmin => "SET_ACCOUNT_MANAGER_ADMIN",
            ChainConfigFunction::PermissionList => "PERMISSION_LIST",
            ChainConfigFunction::UpdateVersion => "UPDATE_VERSION",
            ChainConfigFunction::MultiSignEnableManualRun => "MULTI_SIGN_ENABLE_MANUAL_RUN",
            ChainConfigFunction::EnableOnlyCreatorUpgrade => "ENABLE_ONLY_CREATOR_UPGRADE",
            ChainConfigFunction::DisableOnlyCreatorUpgrade => "DISABLE_ONLY_CREATOR_UPGRADE",
            ChainConfigFunction::SetInvokeGasPrice => "SET_INVOKE_GAS_PRICE",
            ChainConfigFunction::SetInstallBaseGas => "SET_INSTALL_BASE_GAS",
            ChainConfigFunction::SetInstallGasPrice => "SET_INSTALL_GAS_PRICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GET_CHAIN_CONFIG" => Some(Self::GetChainConfig),
            "GET_CHAIN_CONFIG_AT" => Some(Self::GetChainConfigAt),
            "CORE_UPDATE" => Some(Self::CoreUpdate),
            "BLOCK_UPDATE" => Some(Self::BlockUpdate),
            "TRUST_ROOT_ADD" => Some(Self::TrustRootAdd),
            "TRUST_ROOT_UPDATE" => Some(Self::TrustRootUpdate),
            "TRUST_ROOT_DELETE" => Some(Self::TrustRootDelete),
            "NODE_ADDR_ADD" => Some(Self::NodeAddrAdd),
            "NODE_ADDR_UPDATE" => Some(Self::NodeAddrUpdate),
            "NODE_ADDR_DELETE" => Some(Self::NodeAddrDelete),
            "NODE_ORG_ADD" => Some(Self::NodeOrgAdd),
            "NODE_ORG_UPDATE" => Some(Self::NodeOrgUpdate),
            "NODE_ORG_DELETE" => Some(Self::NodeOrgDelete),
            "CONSENSUS_EXT_ADD" => Some(Self::ConsensusExtAdd),
            "CONSENSUS_EXT_UPDATE" => Some(Self::ConsensusExtUpdate),
            "CONSENSUS_EXT_DELETE" => Some(Self::ConsensusExtDelete),
            "PERMISSION_ADD" => Some(Self::PermissionAdd),
            "PERMISSION_UPDATE" => Some(Self::PermissionUpdate),
            "PERMISSION_DELETE" => Some(Self::PermissionDelete),
            "NODE_ID_ADD" => Some(Self::NodeIdAdd),
            "NODE_ID_UPDATE" => Some(Self::NodeIdUpdate),
            "NODE_ID_DELETE" => Some(Self::NodeIdDelete),
            "TRUST_MEMBER_ADD" => Some(Self::TrustMemberAdd),
            "TRUST_MEMBER_UPDATE" => Some(Self::TrustMemberUpdate),
            "TRUST_MEMBER_DELETE" => Some(Self::TrustMemberDelete),
            "ALTER_ADDR_TYPE" => Some(Self::AlterAddrType),
            "ENABLE_OR_DISABLE_GAS" => Some(Self::EnableOrDisableGas),
            "SET_INVOKE_BASE_GAS" => Some(Self::SetInvokeBaseGas),
            "SET_ACCOUNT_MANAGER_ADMIN" => Some(Self::SetAccountManagerAdmin),
            "PERMISSION_LIST" => Some(Self::PermissionList),
            "UPDATE_VERSION" => Some(Self::UpdateVersion),
            "MULTI_SIGN_ENABLE_MANUAL_RUN" => Some(Self::MultiSignEnableManualRun),
            "ENABLE_ONLY_CREATOR_UPGRADE" => Some(Self::EnableOnlyCreatorUpgrade),
            "DISABLE_ONLY_CREATOR_UPGRADE" => Some(Self::DisableOnlyCreatorUpgrade),
            "SET_INVOKE_GAS_PRICE" => Some(Self::SetInvokeGasPrice),
            "SET_INSTALL_BASE_GAS" => Some(Self::SetInstallBaseGas),
            "SET_INSTALL_GAS_PRICE" => Some(Self::SetInstallGasPrice),
            _ => None,
        }
    }
}
/// init contract parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitContract {}
/// Nested message and enum types in `InitContract`.
pub mod init_contract {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        ContractName = 0,
        ContractRuntimeType = 1,
        ContractVersion = 2,
        ContractBytecode = 3,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::ContractName => "CONTRACT_NAME",
                Parameter::ContractRuntimeType => "CONTRACT_RUNTIME_TYPE",
                Parameter::ContractVersion => "CONTRACT_VERSION",
                Parameter::ContractBytecode => "CONTRACT_BYTECODE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTRACT_NAME" => Some(Self::ContractName),
                "CONTRACT_RUNTIME_TYPE" => Some(Self::ContractRuntimeType),
                "CONTRACT_VERSION" => Some(Self::ContractVersion),
                "CONTRACT_BYTECODE" => Some(Self::ContractBytecode),
                _ => None,
            }
        }
    }
}
/// upgrade contract parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeContract {}
/// Nested message and enum types in `UpgradeContract`.
pub mod upgrade_contract {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        ContractName = 0,
        ContractRuntimeType = 1,
        ContractVersion = 2,
        ContractBytecode = 3,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::ContractName => "CONTRACT_NAME",
                Parameter::ContractRuntimeType => "CONTRACT_RUNTIME_TYPE",
                Parameter::ContractVersion => "CONTRACT_VERSION",
                Parameter::ContractBytecode => "CONTRACT_BYTECODE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTRACT_NAME" => Some(Self::ContractName),
                "CONTRACT_RUNTIME_TYPE" => Some(Self::ContractRuntimeType),
                "CONTRACT_VERSION" => Some(Self::ContractVersion),
                "CONTRACT_BYTECODE" => Some(Self::ContractBytecode),
                _ => None,
            }
        }
    }
}
/// freeze contract parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreezeContract {}
/// Nested message and enum types in `FreezeContract`.
pub mod freeze_contract {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        ContractName = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::ContractName => "CONTRACT_NAME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTRACT_NAME" => Some(Self::ContractName),
                _ => None,
            }
        }
    }
}
/// unfreeze contract parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnfreezeContract {}
/// Nested message and enum types in `UnfreezeContract`.
pub mod unfreeze_contract {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        ContractName = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::ContractName => "CONTRACT_NAME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTRACT_NAME" => Some(Self::ContractName),
                _ => None,
            }
        }
    }
}
/// revoke contract parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeContract {}
/// Nested message and enum types in `RevokeContract`.
pub mod revoke_contract {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        ContractName = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::ContractName => "CONTRACT_NAME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTRACT_NAME" => Some(Self::ContractName),
                _ => None,
            }
        }
    }
}
/// query contract parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContractInfo {}
/// Nested message and enum types in `GetContractInfo`.
pub mod get_contract_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        ContractName = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::ContractName => "CONTRACT_NAME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTRACT_NAME" => Some(Self::ContractName),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractAccess {}
/// Nested message and enum types in `ContractAccess`.
pub mod contract_access {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        NativeContractName = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::NativeContractName => "NATIVE_CONTRACT_NAME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NATIVE_CONTRACT_NAME" => Some(Self::NativeContractName),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractInfo {
    #[prost(message, repeated, tag = "1")]
    pub contract_transaction: ::prost::alloc::vec::Vec<ContractTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractTransaction {
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<super::common::Contract>,
    #[prost(string, tag = "2")]
    pub tx_id: ::prost::alloc::string::String,
}
/// methods of user management contract
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractManageFunction {
    /// init contract
    InitContract = 0,
    /// upgrade contract version
    UpgradeContract = 1,
    /// freeze contract, cannot be invoked temporarily
    FreezeContract = 2,
    /// unfreeze contract to normal status
    UnfreezeContract = 3,
    /// revoke contract, cannot be invoked permanently
    RevokeContract = 4,
    /// grant access to a native contract
    GrantContractAccess = 5,
    /// revoke access to a native contract
    RevokeContractAccess = 6,
    /// verify if has access to a certain native contract
    VerifyContractAccess = 7,
    /// initial new chain maker version native contract list
    InitNewNativeContract = 8,
}
impl ContractManageFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractManageFunction::InitContract => "INIT_CONTRACT",
            ContractManageFunction::UpgradeContract => "UPGRADE_CONTRACT",
            ContractManageFunction::FreezeContract => "FREEZE_CONTRACT",
            ContractManageFunction::UnfreezeContract => "UNFREEZE_CONTRACT",
            ContractManageFunction::RevokeContract => "REVOKE_CONTRACT",
            ContractManageFunction::GrantContractAccess => "GRANT_CONTRACT_ACCESS",
            ContractManageFunction::RevokeContractAccess => "REVOKE_CONTRACT_ACCESS",
            ContractManageFunction::VerifyContractAccess => "VERIFY_CONTRACT_ACCESS",
            ContractManageFunction::InitNewNativeContract => "INIT_NEW_NATIVE_CONTRACT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INIT_CONTRACT" => Some(Self::InitContract),
            "UPGRADE_CONTRACT" => Some(Self::UpgradeContract),
            "FREEZE_CONTRACT" => Some(Self::FreezeContract),
            "UNFREEZE_CONTRACT" => Some(Self::UnfreezeContract),
            "REVOKE_CONTRACT" => Some(Self::RevokeContract),
            "GRANT_CONTRACT_ACCESS" => Some(Self::GrantContractAccess),
            "REVOKE_CONTRACT_ACCESS" => Some(Self::RevokeContractAccess),
            "VERIFY_CONTRACT_ACCESS" => Some(Self::VerifyContractAccess),
            "INIT_NEW_NATIVE_CONTRACT" => Some(Self::InitNewNativeContract),
            _ => None,
        }
    }
}
/// methods of contract query
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractQueryFunction {
    /// get contract information
    GetContractInfo = 0,
    /// get contract bytecode
    GetContractBytecode = 1,
    /// get all installed contract
    GetContractList = 2,
    /// get native contract list has access to
    GetDisabledContractList = 3,
}
impl ContractQueryFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractQueryFunction::GetContractInfo => "GET_CONTRACT_INFO",
            ContractQueryFunction::GetContractBytecode => "GET_CONTRACT_BYTECODE",
            ContractQueryFunction::GetContractList => "GET_CONTRACT_LIST",
            ContractQueryFunction::GetDisabledContractList => "GET_DISABLED_CONTRACT_LIST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GET_CONTRACT_INFO" => Some(Self::GetContractInfo),
            "GET_CONTRACT_BYTECODE" => Some(Self::GetContractBytecode),
            "GET_CONTRACT_LIST" => Some(Self::GetContractList),
            "GET_DISABLED_CONTRACT_LIST" => Some(Self::GetDisabledContractList),
            _ => None,
        }
    }
}
/// methods of DPoS ERC20 contract
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DPoSerc20Function {
    /// get owner of DPoS
    GetOwner = 0,
    /// get decimals of DPoS
    GetDecimals = 1,
    /// transfer token at DPoS
    Transfer = 2,
    /// transfer token from user at DPoS
    TransferFrom = 3,
    /// get balance of user at DPoS
    GetBalanceof = 4,
    /// approve token for user to other user at DPoS
    Approve = 5,
    /// get allowance at DPoS
    GetAllowance = 6,
    /// burn token at DPoS
    Burn = 7,
    /// mint token at DPoS
    Mint = 8,
    /// transfer owner ship at DPoS
    TransferOwnership = 9,
    /// get total supply of tokens
    GetTotalSupply = 10,
}
impl DPoSerc20Function {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DPoSerc20Function::GetOwner => "GET_OWNER",
            DPoSerc20Function::GetDecimals => "GET_DECIMALS",
            DPoSerc20Function::Transfer => "TRANSFER",
            DPoSerc20Function::TransferFrom => "TRANSFER_FROM",
            DPoSerc20Function::GetBalanceof => "GET_BALANCEOF",
            DPoSerc20Function::Approve => "APPROVE",
            DPoSerc20Function::GetAllowance => "GET_ALLOWANCE",
            DPoSerc20Function::Burn => "BURN",
            DPoSerc20Function::Mint => "MINT",
            DPoSerc20Function::TransferOwnership => "TRANSFER_OWNERSHIP",
            DPoSerc20Function::GetTotalSupply => "GET_TOTAL_SUPPLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GET_OWNER" => Some(Self::GetOwner),
            "GET_DECIMALS" => Some(Self::GetDecimals),
            "TRANSFER" => Some(Self::Transfer),
            "TRANSFER_FROM" => Some(Self::TransferFrom),
            "GET_BALANCEOF" => Some(Self::GetBalanceof),
            "APPROVE" => Some(Self::Approve),
            "GET_ALLOWANCE" => Some(Self::GetAllowance),
            "BURN" => Some(Self::Burn),
            "MINT" => Some(Self::Mint),
            "TRANSFER_OWNERSHIP" => Some(Self::TransferOwnership),
            "GET_TOTAL_SUPPLY" => Some(Self::GetTotalSupply),
            _ => None,
        }
    }
}
/// methods of chain query contract
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChainQueryFunction {
    /// get block by transactionId
    GetBlockByTxId = 0,
    /// get transaction by transactionId
    GetTxByTxId = 1,
    /// get block by block height
    GetBlockByHeight = 2,
    /// get chain information, include current height and consensus node list
    GetChainInfo = 3,
    /// get the last configuration block
    GetLastConfigBlock = 4,
    /// get block by block hash
    GetBlockByHash = 5,
    /// get the list of chains the node knows
    GetNodeChainList = 6,
    /// get governance information
    GetGovernanceContract = 7,
    /// get read/write set information by eight
    GetBlockWithTxrwsetsByHeight = 8,
    /// get read/write set information by hash
    GetBlockWithTxrwsetsByHash = 9,
    /// get the last block
    GetLastBlock = 10,
    /// get full block by height
    GetFullBlockByHeight = 11,
    /// get block height by tx id
    GetBlockHeightByTxId = 12,
    /// get block height by hash
    GetBlockHeightByHash = 13,
    /// get block header by height
    GetBlockHeaderByHeight = 14,
    /// get archived block height
    GetArchivedBlockHeight = 15,
    /// get all contract info list
    GetAllContracts = 16,
    /// get merkle path by tx id
    GetMerklePathByTxId = 17,
    /// get archive status
    GetArchiveStatus = 18,
}
impl ChainQueryFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChainQueryFunction::GetBlockByTxId => "GET_BLOCK_BY_TX_ID",
            ChainQueryFunction::GetTxByTxId => "GET_TX_BY_TX_ID",
            ChainQueryFunction::GetBlockByHeight => "GET_BLOCK_BY_HEIGHT",
            ChainQueryFunction::GetChainInfo => "GET_CHAIN_INFO",
            ChainQueryFunction::GetLastConfigBlock => "GET_LAST_CONFIG_BLOCK",
            ChainQueryFunction::GetBlockByHash => "GET_BLOCK_BY_HASH",
            ChainQueryFunction::GetNodeChainList => "GET_NODE_CHAIN_LIST",
            ChainQueryFunction::GetGovernanceContract => "GET_GOVERNANCE_CONTRACT",
            ChainQueryFunction::GetBlockWithTxrwsetsByHeight => "GET_BLOCK_WITH_TXRWSETS_BY_HEIGHT",
            ChainQueryFunction::GetBlockWithTxrwsetsByHash => "GET_BLOCK_WITH_TXRWSETS_BY_HASH",
            ChainQueryFunction::GetLastBlock => "GET_LAST_BLOCK",
            ChainQueryFunction::GetFullBlockByHeight => "GET_FULL_BLOCK_BY_HEIGHT",
            ChainQueryFunction::GetBlockHeightByTxId => "GET_BLOCK_HEIGHT_BY_TX_ID",
            ChainQueryFunction::GetBlockHeightByHash => "GET_BLOCK_HEIGHT_BY_HASH",
            ChainQueryFunction::GetBlockHeaderByHeight => "GET_BLOCK_HEADER_BY_HEIGHT",
            ChainQueryFunction::GetArchivedBlockHeight => "GET_ARCHIVED_BLOCK_HEIGHT",
            ChainQueryFunction::GetAllContracts => "GET_ALL_CONTRACTS",
            ChainQueryFunction::GetMerklePathByTxId => "GET_MERKLE_PATH_BY_TX_ID",
            ChainQueryFunction::GetArchiveStatus => "GET_ARCHIVE_STATUS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GET_BLOCK_BY_TX_ID" => Some(Self::GetBlockByTxId),
            "GET_TX_BY_TX_ID" => Some(Self::GetTxByTxId),
            "GET_BLOCK_BY_HEIGHT" => Some(Self::GetBlockByHeight),
            "GET_CHAIN_INFO" => Some(Self::GetChainInfo),
            "GET_LAST_CONFIG_BLOCK" => Some(Self::GetLastConfigBlock),
            "GET_BLOCK_BY_HASH" => Some(Self::GetBlockByHash),
            "GET_NODE_CHAIN_LIST" => Some(Self::GetNodeChainList),
            "GET_GOVERNANCE_CONTRACT" => Some(Self::GetGovernanceContract),
            "GET_BLOCK_WITH_TXRWSETS_BY_HEIGHT" => Some(Self::GetBlockWithTxrwsetsByHeight),
            "GET_BLOCK_WITH_TXRWSETS_BY_HASH" => Some(Self::GetBlockWithTxrwsetsByHash),
            "GET_LAST_BLOCK" => Some(Self::GetLastBlock),
            "GET_FULL_BLOCK_BY_HEIGHT" => Some(Self::GetFullBlockByHeight),
            "GET_BLOCK_HEIGHT_BY_TX_ID" => Some(Self::GetBlockHeightByTxId),
            "GET_BLOCK_HEIGHT_BY_HASH" => Some(Self::GetBlockHeightByHash),
            "GET_BLOCK_HEADER_BY_HEIGHT" => Some(Self::GetBlockHeaderByHeight),
            "GET_ARCHIVED_BLOCK_HEIGHT" => Some(Self::GetArchivedBlockHeight),
            "GET_ALL_CONTRACTS" => Some(Self::GetAllContracts),
            "GET_MERKLE_PATH_BY_TX_ID" => Some(Self::GetMerklePathByTxId),
            "GET_ARCHIVE_STATUS" => Some(Self::GetArchiveStatus),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiSignInfo {
    /// current tx payload
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<super::common::Payload>,
    /// call system contract name
    #[prost(string, tag = "2")]
    pub contract_name: ::prost::alloc::string::String,
    /// call system contract method
    #[prost(string, tag = "3")]
    pub method: ::prost::alloc::string::String,
    /// call system contract parameters
    /// repeated common.KeyValuePair parameters = 4;
    /// status
    #[prost(enumeration = "MultiSignStatus", tag = "4")]
    pub status: i32,
    /// vote list
    #[prost(message, repeated, tag = "5")]
    pub vote_infos: ::prost::alloc::vec::Vec<MultiSignVoteInfo>,
    /// call system contract message
    #[prost(string, tag = "6")]
    pub message: ::prost::alloc::string::String,
    /// call system contract result
    #[prost(bytes = "vec", tag = "7")]
    pub result: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiSignVoteInfo {
    #[prost(enumeration = "VoteStatus", tag = "1")]
    pub vote: i32,
    #[prost(message, optional, tag = "2")]
    pub endorsement: ::core::option::Option<super::common::EndorsementEntry>,
}
/// revoke contract parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiReq {}
/// Nested message and enum types in `MultiReq`.
pub mod multi_req {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        SysContractName = 0,
        SysMethod = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::SysContractName => "SYS_CONTRACT_NAME",
                Parameter::SysMethod => "SYS_METHOD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYS_CONTRACT_NAME" => Some(Self::SysContractName),
                "SYS_METHOD" => Some(Self::SysMethod),
                _ => None,
            }
        }
    }
}
/// revoke contract parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiVote {}
/// Nested message and enum types in `MultiVote`.
pub mod multi_vote {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        VoteInfo = 0,
        TxId = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::VoteInfo => "VOTE_INFO",
                Parameter::TxId => "TX_ID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VOTE_INFO" => Some(Self::VoteInfo),
                "TX_ID" => Some(Self::TxId),
                _ => None,
            }
        }
    }
}
/// revoke contract parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiQuery {}
/// Nested message and enum types in `MultiQuery`.
pub mod multi_query {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        TxId = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::TxId => "TX_ID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TX_ID" => Some(Self::TxId),
                _ => None,
            }
        }
    }
}
/// methods of managing multi signature
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MultiSignFunction {
    /// multi signature request
    Req = 0,
    /// multi signature voting
    Vote = 1,
    /// multi signature query
    Query = 2,
    /// multi signature execute
    Trig = 3,
}
impl MultiSignFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MultiSignFunction::Req => "REQ",
            MultiSignFunction::Vote => "VOTE",
            MultiSignFunction::Query => "QUERY",
            MultiSignFunction::Trig => "TRIG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REQ" => Some(Self::Req),
            "VOTE" => Some(Self::Vote),
            "QUERY" => Some(Self::Query),
            "TRIG" => Some(Self::Trig),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoteStatus {
    Agree = 0,
    Reject = 1,
}
impl VoteStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoteStatus::Agree => "AGREE",
            VoteStatus::Reject => "REJECT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AGREE" => Some(Self::Agree),
            "REJECT" => Some(Self::Reject),
            _ => None,
        }
    }
}
/// smart contract runtime, contains vm type and language type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MultiSignStatus {
    Processing = 0,
    Adopted = 1,
    Refused = 2,
    Failed = 3,
    Passed = 4,
}
impl MultiSignStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MultiSignStatus::Processing => "PROCESSING",
            MultiSignStatus::Adopted => "ADOPTED",
            MultiSignStatus::Refused => "REFUSED",
            MultiSignStatus::Failed => "FAILED",
            MultiSignStatus::Passed => "PASSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROCESSING" => Some(Self::Processing),
            "ADOPTED" => Some(Self::Adopted),
            "REFUSED" => Some(Self::Refused),
            "FAILED" => Some(Self::Failed),
            "PASSED" => Some(Self::Passed),
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
/// methods of Transaction Manager
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionManagerFunction {
    /// add
    AddBlacklistTxIds = 0,
    /// delete
    DeleteBlacklistTxIds = 1,
    /// get
    GetBlacklistTxIds = 2,
}
impl TransactionManagerFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionManagerFunction::AddBlacklistTxIds => "ADD_BLACKLIST_TX_IDS",
            TransactionManagerFunction::DeleteBlacklistTxIds => "DELETE_BLACKLIST_TX_IDS",
            TransactionManagerFunction::GetBlacklistTxIds => "GET_BLACKLIST_TX_IDS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ADD_BLACKLIST_TX_IDS" => Some(Self::AddBlacklistTxIds),
            "DELETE_BLACKLIST_TX_IDS" => Some(Self::DeleteBlacklistTxIds),
            "GET_BLACKLIST_TX_IDS" => Some(Self::GetBlacklistTxIds),
            _ => None,
        }
    }
}
/// subscribe block payload parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeBlock {}
/// Nested message and enum types in `SubscribeBlock`.
pub mod subscribe_block {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        StartBlock = 0,
        EndBlock = 1,
        WithRwset = 2,
        OnlyHeader = 3,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::StartBlock => "START_BLOCK",
                Parameter::EndBlock => "END_BLOCK",
                Parameter::WithRwset => "WITH_RWSET",
                Parameter::OnlyHeader => "ONLY_HEADER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "START_BLOCK" => Some(Self::StartBlock),
                "END_BLOCK" => Some(Self::EndBlock),
                "WITH_RWSET" => Some(Self::WithRwset),
                "ONLY_HEADER" => Some(Self::OnlyHeader),
                _ => None,
            }
        }
    }
}
/// subscribe transaction payload parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTx {}
/// Nested message and enum types in `SubscribeTx`.
pub mod subscribe_tx {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        StartBlock = 0,
        EndBlock = 1,
        ContractName = 2,
        TxIds = 3,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::StartBlock => "START_BLOCK",
                Parameter::EndBlock => "END_BLOCK",
                Parameter::ContractName => "CONTRACT_NAME",
                Parameter::TxIds => "TX_IDS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "START_BLOCK" => Some(Self::StartBlock),
                "END_BLOCK" => Some(Self::EndBlock),
                "CONTRACT_NAME" => Some(Self::ContractName),
                "TX_IDS" => Some(Self::TxIds),
                _ => None,
            }
        }
    }
}
/// subscribe contract event parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeContractEvent {}
/// Nested message and enum types in `SubscribeContractEvent`.
pub mod subscribe_contract_event {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        Topic = 0,
        ContractName = 1,
        StartBlock = 2,
        EndBlock = 3,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::Topic => "TOPIC",
                Parameter::ContractName => "CONTRACT_NAME",
                Parameter::StartBlock => "START_BLOCK",
                Parameter::EndBlock => "END_BLOCK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TOPIC" => Some(Self::Topic),
                "CONTRACT_NAME" => Some(Self::ContractName),
                "START_BLOCK" => Some(Self::StartBlock),
                "END_BLOCK" => Some(Self::EndBlock),
                _ => None,
            }
        }
    }
}
/// methods of subscribe
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscribeFunction {
    SubscribeBlock = 0,
    SubscribeTx = 1,
    SubscribeContractEvent = 2,
}
impl SubscribeFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscribeFunction::SubscribeBlock => "SUBSCRIBE_BLOCK",
            SubscribeFunction::SubscribeTx => "SUBSCRIBE_TX",
            SubscribeFunction::SubscribeContractEvent => "SUBSCRIBE_CONTRACT_EVENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBSCRIBE_BLOCK" => Some(Self::SubscribeBlock),
            "SUBSCRIBE_TX" => Some(Self::SubscribeTx),
            "SUBSCRIBE_CONTRACT_EVENT" => Some(Self::SubscribeContractEvent),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveGateway {}
/// Nested message and enum types in `SaveGateway`.
pub mod save_gateway {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// gateway information byte
        GatewayInfoByte = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::GatewayInfoByte => "GATEWAY_INFO_BYTE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GATEWAY_INFO_BYTE" => Some(Self::GatewayInfoByte),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGateway {}
/// Nested message and enum types in `UpdateGateway`.
pub mod update_gateway {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// gateway id
        GatewayId = 0,
        /// gateway information byte
        GatewayInfoByte = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::GatewayId => "GATEWAY_ID",
                Parameter::GatewayInfoByte => "GATEWAY_INFO_BYTE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GATEWAY_ID" => Some(Self::GatewayId),
                "GATEWAY_INFO_BYTE" => Some(Self::GatewayInfoByte),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGateway {}
/// Nested message and enum types in `GetGateway`.
pub mod get_gateway {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// gateway id
        GatewayId = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::GatewayId => "GATEWAY_ID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GATEWAY_ID" => Some(Self::GatewayId),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGatewayByRange {}
/// Nested message and enum types in `GetGatewayByRange`.
pub mod get_gateway_by_range {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// start gateway id
        StartGatewayId = 0,
        /// stop gateway id
        StopGatewayId = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::StartGatewayId => "START_GATEWAY_ID",
                Parameter::StopGatewayId => "STOP_GATEWAY_ID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "START_GATEWAY_ID" => Some(Self::StartGatewayId),
                "STOP_GATEWAY_ID" => Some(Self::StopGatewayId),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveCrossChainInfo {}
/// Nested message and enum types in `SaveCrossChainInfo`.
pub mod save_cross_chain_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// cross_chain_info_byte
        CrossChainInfoByte = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::CrossChainInfoByte => "CROSS_CHAIN_INFO_BYTE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CROSS_CHAIN_INFO_BYTE" => Some(Self::CrossChainInfoByte),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCrossChainTry {}
/// Nested message and enum types in `UpdateCrossChainTry`.
pub mod update_cross_chain_try {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// cross chain id
        CrossChainId = 0,
        /// cross chain transaction byte
        CrossChainTxByte = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::CrossChainId => "CROSS_CHAIN_ID",
                Parameter::CrossChainTxByte => "CROSS_CHAIN_TX_BYTE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CROSS_CHAIN_ID" => Some(Self::CrossChainId),
                "CROSS_CHAIN_TX_BYTE" => Some(Self::CrossChainTxByte),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCrossChainResult {}
/// Nested message and enum types in `UpdateCrossChainResult`.
pub mod update_cross_chain_result {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// cross chain id
        CrossChainId = 0,
        /// cross chain result
        CrossChainResult = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::CrossChainId => "CROSS_CHAIN_ID",
                Parameter::CrossChainResult => "CROSS_CHAIN_RESULT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CROSS_CHAIN_ID" => Some(Self::CrossChainId),
                "CROSS_CHAIN_RESULT" => Some(Self::CrossChainResult),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteErrorCrossChainTxList {}
/// Nested message and enum types in `DeleteErrorCrossChainTxList`.
pub mod delete_error_cross_chain_tx_list {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// cross chain id
        CrossChainId = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::CrossChainId => "CROSS_CHAIN_ID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CROSS_CHAIN_ID" => Some(Self::CrossChainId),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCrossChainConfirm {}
/// Nested message and enum types in `UpdateCrossChainConfirm`.
pub mod update_cross_chain_confirm {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// cross chain id
        CrossChainId = 0,
        /// cross chain confirm byte
        CrossChainConfirmByte = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::CrossChainId => "CROSS_CHAIN_ID",
                Parameter::CrossChainConfirmByte => "CROSS_CHAIN_CONFIRM_BYTE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CROSS_CHAIN_ID" => Some(Self::CrossChainId),
                "CROSS_CHAIN_CONFIRM_BYTE" => Some(Self::CrossChainConfirmByte),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSrcGatewayConfirm {}
/// Nested message and enum types in `UpdateSrcGatewayConfirm`.
pub mod update_src_gateway_confirm {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// cross chain id
        CrossChainId = 0,
        /// confirm result
        ConfirmResult = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::CrossChainId => "CROSS_CHAIN_ID",
                Parameter::ConfirmResult => "CONFIRM_RESULT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CROSS_CHAIN_ID" => Some(Self::CrossChainId),
                "CONFIRM_RESULT" => Some(Self::ConfirmResult),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCrossChainInfo {}
/// Nested message and enum types in `GetCrossChainInfo`.
pub mod get_cross_chain_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// cross chain id
        CrossChainId = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::CrossChainId => "CROSS_CHAIN_ID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CROSS_CHAIN_ID" => Some(Self::CrossChainId),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCrossChainInfoByRange {}
/// Nested message and enum types in `GetCrossChainInfoByRange`.
pub mod get_cross_chain_info_by_range {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// start cross chain id
        StartCrossChainId = 0,
        /// stop cross chain id
        StopCrossChainId = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::StartCrossChainId => "START_CROSS_CHAIN_ID",
                Parameter::StopCrossChainId => "STOP_CROSS_CHAIN_ID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "START_CROSS_CHAIN_ID" => Some(Self::StartCrossChainId),
                "STOP_CROSS_CHAIN_ID" => Some(Self::StopCrossChainId),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCrossAdmin {}
/// Nested message and enum types in `SetCrossAdmin`.
pub mod set_cross_admin {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// admin address
        CrossAdminAddress = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::CrossAdminAddress => "CROSS_ADMIN_ADDRESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CROSS_ADMIN_ADDRESS" => Some(Self::CrossAdminAddress),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCrossAdmin {}
/// Nested message and enum types in `DeleteCrossAdmin`.
pub mod delete_cross_admin {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// admin address
        CrossAdminAddress = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::CrossAdminAddress => "CROSS_ADMIN_ADDRESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CROSS_ADMIN_ADDRESS" => Some(Self::CrossAdminAddress),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDataType {}
/// Nested message and enum types in `EventDataType`.
pub mod event_data_type {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// string
        String = 0,
        /// map
        Map = 1,
        /// byte
        Byte = 2,
        /// BOOL
        Bool = 3,
        /// int
        Int = 4,
        /// float
        Float = 5,
        /// array
        Array = 6,
        /// hash, bcos
        Hash = 7,
        /// ADDRESS, bcos
        Address = 8,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::String => "STRING",
                Parameter::Map => "MAP",
                Parameter::Byte => "BYTE",
                Parameter::Bool => "BOOL",
                Parameter::Int => "INT",
                Parameter::Float => "FLOAT",
                Parameter::Array => "ARRAY",
                Parameter::Hash => "HASH",
                Parameter::Address => "ADDRESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STRING" => Some(Self::String),
                "MAP" => Some(Self::Map),
                "BYTE" => Some(Self::Byte),
                "BOOL" => Some(Self::Bool),
                "INT" => Some(Self::Int),
                "FLOAT" => Some(Self::Float),
                "ARRAY" => Some(Self::Array),
                "HASH" => Some(Self::Hash),
                "ADDRESS" => Some(Self::Address),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Code {}
/// Nested message and enum types in `Code`.
pub mod code {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// sucess
        GatewaySuccess = 0,
        /// timeout
        GatewayTimeout = 1,
        /// parameter invalid
        InvalidParameter = 2,
        /// tx prove error
        TxProveError = 3,
        /// contract fail
        ContractFail = 4,
        /// internal error
        InternalError = 5,
        /// relay chain error
        RelayChainError = 6,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::GatewaySuccess => "GATEWAY_SUCCESS",
                Parameter::GatewayTimeout => "GATEWAY_TIMEOUT",
                Parameter::InvalidParameter => "INVALID_PARAMETER",
                Parameter::TxProveError => "TX_PROVE_ERROR",
                Parameter::ContractFail => "CONTRACT_FAIL",
                Parameter::InternalError => "INTERNAL_ERROR",
                Parameter::RelayChainError => "RELAY_CHAIN_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GATEWAY_SUCCESS" => Some(Self::GatewaySuccess),
                "GATEWAY_TIMEOUT" => Some(Self::GatewayTimeout),
                "INVALID_PARAMETER" => Some(Self::InvalidParameter),
                "TX_PROVE_ERROR" => Some(Self::TxProveError),
                "CONTRACT_FAIL" => Some(Self::ContractFail),
                "INTERNAL_ERROR" => Some(Self::InternalError),
                "RELAY_CHAIN_ERROR" => Some(Self::RelayChainError),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossType {}
/// Nested message and enum types in `CrossType`.
pub mod cross_type {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// cross chain query
        Query = 0,
        /// cross chain invoke
        Invoke = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::Query => "QUERY",
                Parameter::Invoke => "INVOKE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "QUERY" => Some(Self::Query),
                "INVOKE" => Some(Self::Invoke),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResultValue {}
/// Nested message and enum types in `TxResultValue`.
pub mod tx_result_value {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// success
        TxSuccess = 0,
        /// timeout
        TxTimeout = 1,
        /// fail
        TxFail = 2,
        /// tx not exist
        TxNotExist = 3,
        /// no permissions
        TxNoPermissions = 4,
        /// no gateway
        GatewayNotFound = 5,
        /// gateway ping error
        GatewayPingpongError = 6,
        /// chain ping error
        ChainPingError = 7,
        /// src gateway get error
        SrcGatewayGetError = 8,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::TxSuccess => "TX_SUCCESS",
                Parameter::TxTimeout => "TX_TIMEOUT",
                Parameter::TxFail => "TX_FAIL",
                Parameter::TxNotExist => "TX_NOT_EXIST",
                Parameter::TxNoPermissions => "TX_NO_PERMISSIONS",
                Parameter::GatewayNotFound => "GATEWAY_NOT_FOUND",
                Parameter::GatewayPingpongError => "GATEWAY_PINGPONG_ERROR",
                Parameter::ChainPingError => "CHAIN_PING_ERROR",
                Parameter::SrcGatewayGetError => "SRC_GATEWAY_GET_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TX_SUCCESS" => Some(Self::TxSuccess),
                "TX_TIMEOUT" => Some(Self::TxTimeout),
                "TX_FAIL" => Some(Self::TxFail),
                "TX_NOT_EXIST" => Some(Self::TxNotExist),
                "TX_NO_PERMISSIONS" => Some(Self::TxNoPermissions),
                "GATEWAY_NOT_FOUND" => Some(Self::GatewayNotFound),
                "GATEWAY_PINGPONG_ERROR" => Some(Self::GatewayPingpongError),
                "CHAIN_PING_ERROR" => Some(Self::ChainPingError),
                "SRC_GATEWAY_GET_ERROR" => Some(Self::SrcGatewayGetError),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxVerifyRsult {}
/// Nested message and enum types in `TxVerifyRsult`.
pub mod tx_verify_rsult {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// success
        VerifySuccess = 0,
        /// failed
        VerifyInvalid = 1,
        /// not need
        VerifyNotNeed = 2,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::VerifySuccess => "VERIFY_SUCCESS",
                Parameter::VerifyInvalid => "VERIFY_INVALID",
                Parameter::VerifyNotNeed => "VERIFY_NOT_NEED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERIFY_SUCCESS" => Some(Self::VerifySuccess),
                "VERIFY_INVALID" => Some(Self::VerifyInvalid),
                "VERIFY_NOT_NEED" => Some(Self::VerifyNotNeed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossChainStateValue {}
/// Nested message and enum types in `CrossChainStateValue`.
pub mod cross_chain_state_value {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// new
        New = 0,
        /// wait execute
        WaitExecute = 1,
        /// wait confirm
        WaitConfirm = 2,
        /// confirm end
        ConfirmEnd = 3,
        /// cancel end
        CancelEnd = 4,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::New => "NEW",
                Parameter::WaitExecute => "WAIT_EXECUTE",
                Parameter::WaitConfirm => "WAIT_CONFIRM",
                Parameter::ConfirmEnd => "CONFIRM_END",
                Parameter::CancelEnd => "CANCEL_END",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NEW" => Some(Self::New),
                "WAIT_EXECUTE" => Some(Self::WaitExecute),
                "WAIT_CONFIRM" => Some(Self::WaitConfirm),
                "CONFIRM_END" => Some(Self::ConfirmEnd),
                "CANCEL_END" => Some(Self::CancelEnd),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventName {}
/// Nested message and enum types in `EventName`.
pub mod event_name {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        /// new
        NewCrossChain = 0,
        /// try end
        CrossChainTryEnd = 1,
        /// update result end
        UpadateResultEnd = 2,
        /// confirm end
        GatewayConfirmEnd = 3,
        /// src gateway confirm end
        SrcGatewayConfirmEnd = 4,
        /// set admin
        SetCrossAdmin = 5,
        /// delete admin
        DeleteCrossAdmin = 6,
        /// new gateway
        NewCrossGateway = 7,
        /// gateway update
        CrossGatewayUpdate = 8,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::NewCrossChain => "NEW_CROSS_CHAIN",
                Parameter::CrossChainTryEnd => "CROSS_CHAIN_TRY_END",
                Parameter::UpadateResultEnd => "UPADATE_RESULT_END",
                Parameter::GatewayConfirmEnd => "GATEWAY_CONFIRM_END",
                Parameter::SrcGatewayConfirmEnd => "SRC_GATEWAY_CONFIRM_END",
                Parameter::SetCrossAdmin => "SET_CROSS_ADMIN",
                Parameter::DeleteCrossAdmin => "DELETE_CROSS_ADMIN",
                Parameter::NewCrossGateway => "NEW_CROSS_GATEWAY",
                Parameter::CrossGatewayUpdate => "CROSS_GATEWAY_UPDATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NEW_CROSS_CHAIN" => Some(Self::NewCrossChain),
                "CROSS_CHAIN_TRY_END" => Some(Self::CrossChainTryEnd),
                "UPADATE_RESULT_END" => Some(Self::UpadateResultEnd),
                "GATEWAY_CONFIRM_END" => Some(Self::GatewayConfirmEnd),
                "SRC_GATEWAY_CONFIRM_END" => Some(Self::SrcGatewayConfirmEnd),
                "SET_CROSS_ADMIN" => Some(Self::SetCrossAdmin),
                "DELETE_CROSS_ADMIN" => Some(Self::DeleteCrossAdmin),
                "NEW_CROSS_GATEWAY" => Some(Self::NewCrossGateway),
                "CROSS_GATEWAY_UPDATE" => Some(Self::CrossGatewayUpdate),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossChainInfo {
    /// cross chain id
    #[prost(string, tag = "1")]
    pub cross_chain_id: ::prost::alloc::string::String,
    /// cross chain name
    #[prost(string, tag = "2")]
    pub cross_chain_name: ::prost::alloc::string::String,
    /// cross chain flag
    #[prost(string, tag = "3")]
    pub cross_chain_flag: ::prost::alloc::string::String,
    /// src gateway id
    #[prost(string, tag = "4")]
    pub from: ::prost::alloc::string::String,
    /// cross chain message
    #[prost(message, repeated, tag = "5")]
    pub cross_chain_msg: ::prost::alloc::vec::Vec<CrossChainMsg>,
    /// first tx
    #[prost(message, optional, tag = "6")]
    pub first_tx_content: ::core::option::Option<TxContentWithVerify>,
    /// tx content adn verify result
    #[prost(message, repeated, tag = "7")]
    pub cross_chain_tx_content: ::prost::alloc::vec::Vec<TxContentWithVerify>,
    /// cross chain result
    #[prost(bool, tag = "8")]
    pub cross_chain_result: bool,
    /// cross chain confirm result
    #[prost(message, repeated, tag = "9")]
    pub gateway_confirm_result: ::prost::alloc::vec::Vec<CrossChainConfirm>,
    /// cross chain state
    #[prost(enumeration = "cross_chain_state_value::Parameter", tag = "10")]
    pub state: i32,
    /// confirm information
    #[prost(message, optional, tag = "11")]
    pub confirm_info: ::core::option::Option<ConfirmInfo>,
    /// cancel information
    #[prost(message, optional, tag = "12")]
    pub cancel_info: ::core::option::Option<CancelInfo>,
    /// confirm result
    #[prost(message, optional, tag = "13")]
    pub confirm_result: ::core::option::Option<CrossChainConfirm>,
    /// timeout
    #[prost(int64, tag = "14")]
    pub timeout: i64,
    /// cross type
    #[prost(enumeration = "cross_type::Parameter", tag = "19")]
    pub cross_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossChainMsg {
    /// target gateway id
    #[prost(string, tag = "1")]
    pub gateway_id: ::prost::alloc::string::String,
    /// target chain resource id
    #[prost(string, tag = "2")]
    pub chain_rid: ::prost::alloc::string::String,
    /// target contract name
    #[prost(string, tag = "3")]
    pub contract_name: ::prost::alloc::string::String,
    /// target method
    #[prost(string, tag = "4")]
    pub method: ::prost::alloc::string::String,
    /// sign identity
    #[prost(string, repeated, tag = "5")]
    pub identity: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// target contract parameter
    #[prost(string, tag = "6")]
    pub parameter: ::prost::alloc::string::String,
    /// contract parameter data
    #[prost(int32, repeated, tag = "7")]
    pub param_data: ::prost::alloc::vec::Vec<i32>,
    /// bcos, parameter data type
    #[prost(enumeration = "event_data_type::Parameter", repeated, tag = "8")]
    pub param_data_type: ::prost::alloc::vec::Vec<i32>,
    /// extra data
    #[prost(string, tag = "9")]
    pub extra_data: ::prost::alloc::string::String,
    /// confirm information
    #[prost(message, optional, tag = "10")]
    pub confirm_info: ::core::option::Option<ConfirmInfo>,
    /// cancel information
    #[prost(message, optional, tag = "11")]
    pub cancel_info: ::core::option::Option<CancelInfo>,
    /// bcos abi
    #[prost(string, tag = "12")]
    pub abi: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxContentWithVerify {
    /// tx content
    #[prost(message, optional, tag = "1")]
    pub tx_content: ::core::option::Option<TxContent>,
    /// try result
    #[prost(string, repeated, tag = "2")]
    pub try_result: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// tx verify result
    #[prost(enumeration = "tx_verify_rsult::Parameter", tag = "3")]
    pub tx_verify_result: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmInfo {
    /// chain resource id
    #[prost(string, tag = "2")]
    pub chain_rid: ::prost::alloc::string::String,
    /// contract name
    #[prost(string, tag = "3")]
    pub contract_name: ::prost::alloc::string::String,
    /// method
    #[prost(string, tag = "4")]
    pub method: ::prost::alloc::string::String,
    /// parameter
    #[prost(string, tag = "5")]
    pub parameter: ::prost::alloc::string::String,
    /// parameter data
    #[prost(int32, repeated, tag = "6")]
    pub param_data: ::prost::alloc::vec::Vec<i32>,
    /// bcos, parameter data type
    #[prost(enumeration = "event_data_type::Parameter", repeated, tag = "7")]
    pub param_data_type: ::prost::alloc::vec::Vec<i32>,
    /// extra data
    #[prost(string, tag = "8")]
    pub extra_data: ::prost::alloc::string::String,
    /// bcos abi
    #[prost(string, tag = "9")]
    pub abi: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossChainConfirm {
    /// result code
    #[prost(enumeration = "code::Parameter", tag = "1")]
    pub code: i32,
    /// message
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxContent {
    /// tx id
    #[prost(string, tag = "1")]
    pub tx_id: ::prost::alloc::string::String,
    /// tx content
    #[prost(bytes = "vec", tag = "2")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    /// tx result
    #[prost(enumeration = "tx_result_value::Parameter", tag = "3")]
    pub tx_result: i32,
    /// gateway id
    #[prost(string, tag = "4")]
    pub gateway_id: ::prost::alloc::string::String,
    /// chain resource id
    #[prost(string, tag = "5")]
    pub chain_rid: ::prost::alloc::string::String,
    /// tx prove json string
    #[prost(string, tag = "6")]
    pub tx_prove: ::prost::alloc::string::String,
    /// block height
    #[prost(int64, tag = "7")]
    pub block_height: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelInfo {
    /// chain resource id
    #[prost(string, tag = "2")]
    pub chain_rid: ::prost::alloc::string::String,
    /// contract name
    #[prost(string, tag = "3")]
    pub contract_name: ::prost::alloc::string::String,
    /// method
    #[prost(string, tag = "4")]
    pub method: ::prost::alloc::string::String,
    /// parameter
    #[prost(string, tag = "5")]
    pub parameter: ::prost::alloc::string::String,
    /// param_data
    #[prost(int32, repeated, tag = "6")]
    pub param_data: ::prost::alloc::vec::Vec<i32>,
    /// bcos param data type
    #[prost(enumeration = "event_data_type::Parameter", repeated, tag = "7")]
    pub param_data_type: ::prost::alloc::vec::Vec<i32>,
    /// extra data
    #[prost(string, tag = "8")]
    pub extra_data: ::prost::alloc::string::String,
    /// bcos abi
    #[prost(string, tag = "9")]
    pub abi: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossChainTxUpChain {
    /// confirm index
    #[prost(int32, tag = "1")]
    pub index: i32,
    /// tx content and verify result
    #[prost(message, optional, tag = "2")]
    pub tx_content_with_verify: ::core::option::Option<TxContentWithVerify>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossChainConfirmUpChain {
    /// confirm index
    #[prost(int32, tag = "1")]
    pub index: i32,
    /// confirm result
    #[prost(message, optional, tag = "2")]
    pub cross_chain_confirm: ::core::option::Option<CrossChainConfirm>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RelayCrossFunction {
    /// save gateway
    SaveGateway = 0,
    /// update gateway
    UpdateGateway = 1,
    /// save cross chain info
    SaveCrossChainInfo = 2,
    /// get error cross chain transaction list
    GetErrorCrossChainTxList = 3,
    /// delete error cross chain transaction list
    DeleteErrorCrossChainTxList = 4,
    /// update cross chain try
    UpdateCrossChainTry = 5,
    /// update cross chain result
    UpdateCrossChainResult = 6,
    /// update cross chain confirm
    UpdateCrossChainConfirm = 7,
    /// update source gateway confirm
    UpdateSrcGatewayConfirm = 8,
    /// get gateway number
    GetGatewayNum = 9,
    /// get gateway
    GetGateway = 10,
    /// get gateway by range
    GetGatewayByRange = 11,
    /// get cross chain number
    GetCrossChainNum = 12,
    /// get cross chain information
    GetCrossChainInfo = 13,
    /// get cross chain information by range
    GetCrossChainInfoByRange = 14,
    /// get not end cross chian id list
    GetNotEndCrossChianIdList = 15,
    /// set admin
    SetCrossAdmin = 16,
    /// delete admin
    DeleteCrossAdemin = 17,
    /// check if sender is admin
    IsCrossAdmin = 18,
}
impl RelayCrossFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RelayCrossFunction::SaveGateway => "SAVE_GATEWAY",
            RelayCrossFunction::UpdateGateway => "UPDATE_GATEWAY",
            RelayCrossFunction::SaveCrossChainInfo => "SAVE_CROSS_CHAIN_INFO",
            RelayCrossFunction::GetErrorCrossChainTxList => "GET_ERROR_CROSS_CHAIN_TX_LIST",
            RelayCrossFunction::DeleteErrorCrossChainTxList => "DELETE_ERROR_CROSS_CHAIN_TX_LIST",
            RelayCrossFunction::UpdateCrossChainTry => "UPDATE_CROSS_CHAIN_TRY",
            RelayCrossFunction::UpdateCrossChainResult => "UPDATE_CROSS_CHAIN_RESULT",
            RelayCrossFunction::UpdateCrossChainConfirm => "UPDATE_CROSS_CHAIN_CONFIRM",
            RelayCrossFunction::UpdateSrcGatewayConfirm => "UPDATE_SRC_GATEWAY_CONFIRM",
            RelayCrossFunction::GetGatewayNum => "GET_GATEWAY_NUM",
            RelayCrossFunction::GetGateway => "GET_GATEWAY",
            RelayCrossFunction::GetGatewayByRange => "GET_GATEWAY_BY_RANGE",
            RelayCrossFunction::GetCrossChainNum => "GET_CROSS_CHAIN_NUM",
            RelayCrossFunction::GetCrossChainInfo => "GET_CROSS_CHAIN_INFO",
            RelayCrossFunction::GetCrossChainInfoByRange => "GET_CROSS_CHAIN_INFO_BY_RANGE",
            RelayCrossFunction::GetNotEndCrossChianIdList => "GET_NOT_END_CROSS_CHIAN_ID_LIST",
            RelayCrossFunction::SetCrossAdmin => "SET_CROSS_ADMIN",
            RelayCrossFunction::DeleteCrossAdemin => "DELETE_CROSS_ADEMIN",
            RelayCrossFunction::IsCrossAdmin => "IS_CROSS_ADMIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SAVE_GATEWAY" => Some(Self::SaveGateway),
            "UPDATE_GATEWAY" => Some(Self::UpdateGateway),
            "SAVE_CROSS_CHAIN_INFO" => Some(Self::SaveCrossChainInfo),
            "GET_ERROR_CROSS_CHAIN_TX_LIST" => Some(Self::GetErrorCrossChainTxList),
            "DELETE_ERROR_CROSS_CHAIN_TX_LIST" => Some(Self::DeleteErrorCrossChainTxList),
            "UPDATE_CROSS_CHAIN_TRY" => Some(Self::UpdateCrossChainTry),
            "UPDATE_CROSS_CHAIN_RESULT" => Some(Self::UpdateCrossChainResult),
            "UPDATE_CROSS_CHAIN_CONFIRM" => Some(Self::UpdateCrossChainConfirm),
            "UPDATE_SRC_GATEWAY_CONFIRM" => Some(Self::UpdateSrcGatewayConfirm),
            "GET_GATEWAY_NUM" => Some(Self::GetGatewayNum),
            "GET_GATEWAY" => Some(Self::GetGateway),
            "GET_GATEWAY_BY_RANGE" => Some(Self::GetGatewayByRange),
            "GET_CROSS_CHAIN_NUM" => Some(Self::GetCrossChainNum),
            "GET_CROSS_CHAIN_INFO" => Some(Self::GetCrossChainInfo),
            "GET_CROSS_CHAIN_INFO_BY_RANGE" => Some(Self::GetCrossChainInfoByRange),
            "GET_NOT_END_CROSS_CHIAN_ID_LIST" => Some(Self::GetNotEndCrossChianIdList),
            "SET_CROSS_ADMIN" => Some(Self::SetCrossAdmin),
            "DELETE_CROSS_ADEMIN" => Some(Self::DeleteCrossAdemin),
            "IS_CROSS_ADMIN" => Some(Self::IsCrossAdmin),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountMultiSign {
    #[prost(bytes = "vec", tag = "1")]
    pub payloads: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub client_sign: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub public_key_info: ::prost::alloc::vec::Vec<u8>,
}
/// account multi sign req
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountMultiSignsReq {
    #[prost(message, repeated, tag = "1")]
    pub gas_multi_signs: ::prost::alloc::vec::Vec<AccountMultiSign>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RechargeGas {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub gas_amount: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RechargeGasReq {
    #[prost(message, repeated, tag = "1")]
    pub batch_recharge_gas: ::prost::alloc::vec::Vec<RechargeGas>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetContractMethodPayerParams {
    #[prost(string, tag = "1")]
    pub contract_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub method: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub payer_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// params keys for native call
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetContractMethodPayer {}
/// Nested message and enum types in `SetContractMethodPayer`.
pub mod set_contract_method_payer {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        Endorsement = 0,
        Params = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::Endorsement => "ENDORSEMENT",
                Parameter::Params => "PARAMS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENDORSEMENT" => Some(Self::Endorsement),
                "PARAMS" => Some(Self::Params),
                _ => None,
            }
        }
    }
}
/// params for native call
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsetContractMethodPayer {}
/// Nested message and enum types in `UnsetContractMethodPayer`.
pub mod unset_contract_method_payer {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        ContractName = 0,
        Method = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::ContractName => "CONTRACT_NAME",
                Parameter::Method => "METHOD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTRACT_NAME" => Some(Self::ContractName),
                "METHOD" => Some(Self::Method),
                _ => None,
            }
        }
    }
}
/// params for native call
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContractMethodPayer {}
/// Nested message and enum types in `GetContractMethodPayer`.
pub mod get_contract_method_payer {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        ContractName = 0,
        Method = 1,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::ContractName => "CONTRACT_NAME",
                Parameter::Method => "METHOD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTRACT_NAME" => Some(Self::ContractName),
                "METHOD" => Some(Self::Method),
                _ => None,
            }
        }
    }
}
/// params for native call
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxPayer {}
/// Nested message and enum types in `GetTxPayer`.
pub mod get_tx_payer {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        TxId = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::TxId => "TX_ID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TX_ID" => Some(Self::TxId),
                _ => None,
            }
        }
    }
}
/// methods of private compute contract
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GasAccountFunction {
    /// set admin
    SetAdmin = 0,
    /// get admin
    GetAdmin = 1,
    /// recharge gas
    RechargeGas = 2,
    /// get balance
    GetBalance = 3,
    /// charge gas
    ChargeGas = 4,
    /// frozen account
    FrozenAccount = 5,
    /// unfrozen account
    UnfrozenAccount = 6,
    /// account status
    AccountStatus = 7,
    /// refund gas
    RefundGas = 8,
    /// refund gas for vm
    RefundGasVm = 9,
    /// charge gas for multi accounts
    ChargeGasForMultiAccount = 10,
    /// set payer for contract's method
    SetContractMethodPayer = 11,
    /// clear payer for contract's method
    UnsetContractMethodPayer = 12,
    /// get payer for contract's method
    GetContractMethodPayer = 13,
    /// gey payer of tx
    GetTxPayer = 14,
}
impl GasAccountFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GasAccountFunction::SetAdmin => "SET_ADMIN",
            GasAccountFunction::GetAdmin => "GET_ADMIN",
            GasAccountFunction::RechargeGas => "RECHARGE_GAS",
            GasAccountFunction::GetBalance => "GET_BALANCE",
            GasAccountFunction::ChargeGas => "CHARGE_GAS",
            GasAccountFunction::FrozenAccount => "FROZEN_ACCOUNT",
            GasAccountFunction::UnfrozenAccount => "UNFROZEN_ACCOUNT",
            GasAccountFunction::AccountStatus => "ACCOUNT_STATUS",
            GasAccountFunction::RefundGas => "REFUND_GAS",
            GasAccountFunction::RefundGasVm => "REFUND_GAS_VM",
            GasAccountFunction::ChargeGasForMultiAccount => "CHARGE_GAS_FOR_MULTI_ACCOUNT",
            GasAccountFunction::SetContractMethodPayer => "SET_CONTRACT_METHOD_PAYER",
            GasAccountFunction::UnsetContractMethodPayer => "UNSET_CONTRACT_METHOD_PAYER",
            GasAccountFunction::GetContractMethodPayer => "GET_CONTRACT_METHOD_PAYER",
            GasAccountFunction::GetTxPayer => "GET_TX_PAYER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SET_ADMIN" => Some(Self::SetAdmin),
            "GET_ADMIN" => Some(Self::GetAdmin),
            "RECHARGE_GAS" => Some(Self::RechargeGas),
            "GET_BALANCE" => Some(Self::GetBalance),
            "CHARGE_GAS" => Some(Self::ChargeGas),
            "FROZEN_ACCOUNT" => Some(Self::FrozenAccount),
            "UNFROZEN_ACCOUNT" => Some(Self::UnfrozenAccount),
            "ACCOUNT_STATUS" => Some(Self::AccountStatus),
            "REFUND_GAS" => Some(Self::RefundGas),
            "REFUND_GAS_VM" => Some(Self::RefundGasVm),
            "CHARGE_GAS_FOR_MULTI_ACCOUNT" => Some(Self::ChargeGasForMultiAccount),
            "SET_CONTRACT_METHOD_PAYER" => Some(Self::SetContractMethodPayer),
            "UNSET_CONTRACT_METHOD_PAYER" => Some(Self::UnsetContractMethodPayer),
            "GET_CONTRACT_METHOD_PAYER" => Some(Self::GetContractMethodPayer),
            "GET_TX_PAYER" => Some(Self::GetTxPayer),
            _ => None,
        }
    }
}
/// methods of chain query contract
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TestContractFunction {
    /// put data, parameters: k,v
    P = 0,
    /// get data parameter: k
    G = 1,
    /// nothing to do.
    N = 2,
    /// delete data by key, parameter: k
    D = 3,
}
impl TestContractFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TestContractFunction::P => "P",
            TestContractFunction::G => "G",
            TestContractFunction::N => "N",
            TestContractFunction::D => "D",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "P" => Some(Self::P),
            "G" => Some(Self::G),
            "N" => Some(Self::N),
            "D" => Some(Self::D),
            _ => None,
        }
    }
}
/// archive block payload parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveBlock {}
/// Nested message and enum types in `ArchiveBlock`.
pub mod archive_block {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        BlockHeight = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::BlockHeight => "BLOCK_HEIGHT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BLOCK_HEIGHT" => Some(Self::BlockHeight),
                _ => None,
            }
        }
    }
}
/// restore block payload parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreBlock {}
/// Nested message and enum types in `RestoreBlock`.
pub mod restore_block {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Parameter {
        FullBlock = 0,
    }
    impl Parameter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Parameter::FullBlock => "FULL_BLOCK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FULL_BLOCK" => Some(Self::FullBlock),
                _ => None,
            }
        }
    }
}
/// methods of archive block
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ArchiveFunction {
    ArchiveBlock = 0,
    RestoreBlock = 1,
}
impl ArchiveFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ArchiveFunction::ArchiveBlock => "ARCHIVE_BLOCK",
            ArchiveFunction::RestoreBlock => "RESTORE_BLOCK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ARCHIVE_BLOCK" => Some(Self::ArchiveBlock),
            "RESTORE_BLOCK" => Some(Self::RestoreBlock),
            _ => None,
        }
    }
}
/// methods of pubkey management
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PubkeyManageFunction {
    /// add one pubkey
    PubkeyAdd = 0,
    /// delete pubkeys
    PubkeyDelete = 1,
    /// query pubkeys
    PubkeyQuery = 2,
}
impl PubkeyManageFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PubkeyManageFunction::PubkeyAdd => "PUBKEY_ADD",
            PubkeyManageFunction::PubkeyDelete => "PUBKEY_DELETE",
            PubkeyManageFunction::PubkeyQuery => "PUBKEY_QUERY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PUBKEY_ADD" => Some(Self::PubkeyAdd),
            "PUBKEY_DELETE" => Some(Self::PubkeyDelete),
            "PUBKEY_QUERY" => Some(Self::PubkeyQuery),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Epoch {
    /// ID with auto-increment
    #[prost(uint64, tag = "1")]
    pub epoch_id: u64,
    /// A collection of validators for the current generation
    #[prost(string, repeated, tag = "2")]
    pub proposer_vector: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Next epoch switch height
    #[prost(uint64, tag = "3")]
    pub next_epoch_create_height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    /// The address of the verifier: base58.Encode(sha256(pubkey))
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// active punishment tags whether validator is jailed or not
    #[prost(bool, tag = "2")]
    pub jailed: bool,
    /// validator status: Bonded / Unbonding / Unbonded
    #[prost(enumeration = "BondStatus", tag = "3")]
    pub status: i32,
    /// delegate token amount
    #[prost(string, tag = "4")]
    pub tokens: ::prost::alloc::string::String,
    /// delegate share amount
    #[prost(string, tag = "5")]
    pub delegator_shares: ::prost::alloc::string::String,
    /// undelegate entry epoch id
    #[prost(uint64, tag = "6")]
    pub unbonding_epoch_id: u64,
    /// undelegate entry complete epoch id
    #[prost(uint64, tag = "7")]
    pub unbonding_completion_epoch_id: u64,
    /// validator self delegate token amount
    #[prost(string, tag = "8")]
    pub self_delegation: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delegation {
    /// delegator cert hash
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator cert hash
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// share amount
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingDelegation {
    /// epoch id
    #[prost(string, tag = "1")]
    pub epoch_id: ::prost::alloc::string::String,
    /// delegator cert hash
    #[prost(string, tag = "2")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator cert hash
    #[prost(string, tag = "3")]
    pub validator_address: ::prost::alloc::string::String,
    /// unbond entry records
    #[prost(message, repeated, tag = "4")]
    pub entries: ::prost::alloc::vec::Vec<UnbondingDelegationEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingDelegationEntry {
    /// create epoch id
    #[prost(uint64, tag = "1")]
    pub creation_epoch_id: u64,
    /// complete epoch id
    #[prost(uint64, tag = "2")]
    pub completion_epoch_id: u64,
    /// undelegate amount
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorVector {
    /// validator cert hash vector
    #[prost(string, repeated, tag = "1")]
    pub vector: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationInfo {
    /// delegate slice
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<Delegation>,
}
/// methods of DPoS stake contract
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DPoSStakeFunction {
    /// get all validator candidates
    GetAllCandidates = 0,
    /// get validator by address
    GetValidatorByAddress = 1,
    /// delegate
    Delegate = 2,
    /// get delegate by address
    GetDelegationsByAddress = 3,
    /// get user delegation by validator
    GetUserDelegationByValidator = 4,
    /// undelegate
    Undelegate = 5,
    /// read epoch by id
    ReadEpochById = 6,
    /// read latest epoch
    ReadLatestEpoch = 7,
    /// set node id before join network
    SetNodeId = 8,
    /// get node id after join network
    GetNodeId = 9,
    /// update min self delegation
    UpdateMinSelfDelegation = 10,
    /// read min self delegation
    ReadMinSelfDelegation = 11,
    /// update epoch validator number
    UpdateEpochValidatorNumber = 12,
    /// read epoch validator number
    ReadEpochValidatorNumber = 13,
    /// update epoch block number
    UpdateEpochBlockNumber = 14,
    /// read epoch block number
    ReadEpochBlockNumber = 15,
    /// read complete unbounding epoch number
    ReadCompleteUnboundingEpochNumber = 16,
    /// read system contract address
    ReadSystemContractAddr = 18,
}
impl DPoSStakeFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DPoSStakeFunction::GetAllCandidates => "GET_ALL_CANDIDATES",
            DPoSStakeFunction::GetValidatorByAddress => "GET_VALIDATOR_BY_ADDRESS",
            DPoSStakeFunction::Delegate => "DELEGATE",
            DPoSStakeFunction::GetDelegationsByAddress => "GET_DELEGATIONS_BY_ADDRESS",
            DPoSStakeFunction::GetUserDelegationByValidator => "GET_USER_DELEGATION_BY_VALIDATOR",
            DPoSStakeFunction::Undelegate => "UNDELEGATE",
            DPoSStakeFunction::ReadEpochById => "READ_EPOCH_BY_ID",
            DPoSStakeFunction::ReadLatestEpoch => "READ_LATEST_EPOCH",
            DPoSStakeFunction::SetNodeId => "SET_NODE_ID",
            DPoSStakeFunction::GetNodeId => "GET_NODE_ID",
            DPoSStakeFunction::UpdateMinSelfDelegation => "UPDATE_MIN_SELF_DELEGATION",
            DPoSStakeFunction::ReadMinSelfDelegation => "READ_MIN_SELF_DELEGATION",
            DPoSStakeFunction::UpdateEpochValidatorNumber => "UPDATE_EPOCH_VALIDATOR_NUMBER",
            DPoSStakeFunction::ReadEpochValidatorNumber => "READ_EPOCH_VALIDATOR_NUMBER",
            DPoSStakeFunction::UpdateEpochBlockNumber => "UPDATE_EPOCH_BLOCK_NUMBER",
            DPoSStakeFunction::ReadEpochBlockNumber => "READ_EPOCH_BLOCK_NUMBER",
            DPoSStakeFunction::ReadCompleteUnboundingEpochNumber => {
                "READ_COMPLETE_UNBOUNDING_EPOCH_NUMBER"
            }
            DPoSStakeFunction::ReadSystemContractAddr => "READ_SYSTEM_CONTRACT_ADDR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GET_ALL_CANDIDATES" => Some(Self::GetAllCandidates),
            "GET_VALIDATOR_BY_ADDRESS" => Some(Self::GetValidatorByAddress),
            "DELEGATE" => Some(Self::Delegate),
            "GET_DELEGATIONS_BY_ADDRESS" => Some(Self::GetDelegationsByAddress),
            "GET_USER_DELEGATION_BY_VALIDATOR" => Some(Self::GetUserDelegationByValidator),
            "UNDELEGATE" => Some(Self::Undelegate),
            "READ_EPOCH_BY_ID" => Some(Self::ReadEpochById),
            "READ_LATEST_EPOCH" => Some(Self::ReadLatestEpoch),
            "SET_NODE_ID" => Some(Self::SetNodeId),
            "GET_NODE_ID" => Some(Self::GetNodeId),
            "UPDATE_MIN_SELF_DELEGATION" => Some(Self::UpdateMinSelfDelegation),
            "READ_MIN_SELF_DELEGATION" => Some(Self::ReadMinSelfDelegation),
            "UPDATE_EPOCH_VALIDATOR_NUMBER" => Some(Self::UpdateEpochValidatorNumber),
            "READ_EPOCH_VALIDATOR_NUMBER" => Some(Self::ReadEpochValidatorNumber),
            "UPDATE_EPOCH_BLOCK_NUMBER" => Some(Self::UpdateEpochBlockNumber),
            "READ_EPOCH_BLOCK_NUMBER" => Some(Self::ReadEpochBlockNumber),
            "READ_COMPLETE_UNBOUNDING_EPOCH_NUMBER" => {
                Some(Self::ReadCompleteUnboundingEpochNumber)
            }
            "READ_SYSTEM_CONTRACT_ADDR" => Some(Self::ReadSystemContractAddr),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BondStatus {
    Bonded = 0,
    Unbonding = 1,
    Unbonded = 2,
}
impl BondStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BondStatus::Bonded => "BONDED",
            BondStatus::Unbonding => "UNBONDING",
            BondStatus::Unbonded => "UNBONDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BONDED" => Some(Self::Bonded),
            "UNBONDING" => Some(Self::Unbonding),
            "UNBONDED" => Some(Self::Unbonded),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteAttestationRequest {
    #[prost(message, repeated, tag = "1")]
    pub sign_pair: ::prost::alloc::vec::Vec<SignInfo>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<RemoteAttestationPayload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteAttestationPayload {
    #[prost(string, tag = "1")]
    pub challenge: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub org_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateDeployRequest {
    #[prost(message, repeated, tag = "1")]
    pub sign_pair: ::prost::alloc::vec::Vec<SignInfo>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<PrivateDeployPayload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateDeployPayload {
    #[prost(string, tag = "1")]
    pub code_bytes: ::prost::alloc::string::String,
    /// deploy args which is packed by abi
    #[prost(string, tag = "2")]
    pub private_rlp_data: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub passwd: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub sig_algo: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub contract_name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub contract_version: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub code_hash: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "8")]
    pub org_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "9")]
    pub time_stamp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateComputeRequest {
    #[prost(message, repeated, tag = "1")]
    pub sign_pair: ::prost::alloc::vec::Vec<SignInfo>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<PrivateComputePayload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateComputePayload {
    #[prost(string, tag = "1")]
    pub private_rlp_data: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub passwd: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sig_algo: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub contract_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub code_hash: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "6")]
    pub org_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "7")]
    pub time_stamp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInfo {
    #[prost(string, tag = "1")]
    pub client_sign: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cert: ::prost::alloc::string::String,
}
/// methods of private compute contract
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrivateComputeFunction {
    /// get contract code
    GetContract = 0,
    /// get private data
    GetData = 1,
    /// save cert of tee
    SaveCaCert = 2,
    /// save private data dir
    SaveDir = 3,
    /// save data of private computation result
    SaveData = 4,
    /// save enclave report
    SaveEnclaveReport = 5,
    /// get enclave proof
    GetEnclaveProof = 6,
    /// get cert of tee
    GetCaCert = 7,
    /// get private data dir
    GetDir = 8,
    /// check caller cert auth
    CheckCallerCertAuth = 9,
    GetEnclaveEncryptPubKey = 10,
    GetEnclaveVerificationPubKey = 11,
    GetEnclaveReport = 12,
    GetEnclaveChallenge = 13,
    GetEnclaveSignature = 14,
    SaveRemoteAttestation = 15,
}
impl PrivateComputeFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PrivateComputeFunction::GetContract => "GET_CONTRACT",
            PrivateComputeFunction::GetData => "GET_DATA",
            PrivateComputeFunction::SaveCaCert => "SAVE_CA_CERT",
            PrivateComputeFunction::SaveDir => "SAVE_DIR",
            PrivateComputeFunction::SaveData => "SAVE_DATA",
            PrivateComputeFunction::SaveEnclaveReport => "SAVE_ENCLAVE_REPORT",
            PrivateComputeFunction::GetEnclaveProof => "GET_ENCLAVE_PROOF",
            PrivateComputeFunction::GetCaCert => "GET_CA_CERT",
            PrivateComputeFunction::GetDir => "GET_DIR",
            PrivateComputeFunction::CheckCallerCertAuth => "CHECK_CALLER_CERT_AUTH",
            PrivateComputeFunction::GetEnclaveEncryptPubKey => "GET_ENCLAVE_ENCRYPT_PUB_KEY",
            PrivateComputeFunction::GetEnclaveVerificationPubKey => {
                "GET_ENCLAVE_VERIFICATION_PUB_KEY"
            }
            PrivateComputeFunction::GetEnclaveReport => "GET_ENCLAVE_REPORT",
            PrivateComputeFunction::GetEnclaveChallenge => "GET_ENCLAVE_CHALLENGE",
            PrivateComputeFunction::GetEnclaveSignature => "GET_ENCLAVE_SIGNATURE",
            PrivateComputeFunction::SaveRemoteAttestation => "SAVE_REMOTE_ATTESTATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GET_CONTRACT" => Some(Self::GetContract),
            "GET_DATA" => Some(Self::GetData),
            "SAVE_CA_CERT" => Some(Self::SaveCaCert),
            "SAVE_DIR" => Some(Self::SaveDir),
            "SAVE_DATA" => Some(Self::SaveData),
            "SAVE_ENCLAVE_REPORT" => Some(Self::SaveEnclaveReport),
            "GET_ENCLAVE_PROOF" => Some(Self::GetEnclaveProof),
            "GET_CA_CERT" => Some(Self::GetCaCert),
            "GET_DIR" => Some(Self::GetDir),
            "CHECK_CALLER_CERT_AUTH" => Some(Self::CheckCallerCertAuth),
            "GET_ENCLAVE_ENCRYPT_PUB_KEY" => Some(Self::GetEnclaveEncryptPubKey),
            "GET_ENCLAVE_VERIFICATION_PUB_KEY" => Some(Self::GetEnclaveVerificationPubKey),
            "GET_ENCLAVE_REPORT" => Some(Self::GetEnclaveReport),
            "GET_ENCLAVE_CHALLENGE" => Some(Self::GetEnclaveChallenge),
            "GET_ENCLAVE_SIGNATURE" => Some(Self::GetEnclaveSignature),
            "SAVE_REMOTE_ATTESTATION" => Some(Self::SaveRemoteAttestation),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossState {
    #[prost(enumeration = "CrossTxState", tag = "1")]
    pub state: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrossTransactionFunction {
    /// transaction execute
    Execute = 0,
    /// transaction commit
    Commit = 1,
    /// transaction rollback
    Rollback = 2,
    /// read cross id state
    ReadState = 3,
    /// save cross other transaction proof
    SaveProof = 4,
    /// read cross other transaction proof
    ReadProof = 5,
    /// arbitrate the cross transaction
    Arbitrate = 6,
}
impl CrossTransactionFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrossTransactionFunction::Execute => "EXECUTE",
            CrossTransactionFunction::Commit => "COMMIT",
            CrossTransactionFunction::Rollback => "ROLLBACK",
            CrossTransactionFunction::ReadState => "READ_STATE",
            CrossTransactionFunction::SaveProof => "SAVE_PROOF",
            CrossTransactionFunction::ReadProof => "READ_PROOF",
            CrossTransactionFunction::Arbitrate => "ARBITRATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTE" => Some(Self::Execute),
            "COMMIT" => Some(Self::Commit),
            "ROLLBACK" => Some(Self::Rollback),
            "READ_STATE" => Some(Self::ReadState),
            "SAVE_PROOF" => Some(Self::SaveProof),
            "READ_PROOF" => Some(Self::ReadProof),
            "ARBITRATE" => Some(Self::Arbitrate),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrossTxState {
    /// NON_EXIST cross id is not exist
    NonExist = 0,
    /// INIT just mark this cross is has been processed
    Init = 1,
    /// EXECUTE_OK cross tx execute successfully
    ExecuteOk = 2,
    /// EXECUTE_FAIL cross tx execute fail
    ExecuteFail = 3,
    /// COMMIT_OK cross tx commit successfully
    CommitOk = 4,
    /// COMMIT_FAIL cross tx commit fail
    CommitFail = 5,
    /// ROLLBACK_OK cross tx rollback successfully
    RollbackOk = 6,
    /// ROLLBACK_FAIL cross tx rollback fail
    RollbackFail = 7,
}
impl CrossTxState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrossTxState::NonExist => "NON_EXIST",
            CrossTxState::Init => "INIT",
            CrossTxState::ExecuteOk => "EXECUTE_OK",
            CrossTxState::ExecuteFail => "EXECUTE_FAIL",
            CrossTxState::CommitOk => "COMMIT_OK",
            CrossTxState::CommitFail => "COMMIT_FAIL",
            CrossTxState::RollbackOk => "ROLLBACK_OK",
            CrossTxState::RollbackFail => "ROLLBACK_FAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NON_EXIST" => Some(Self::NonExist),
            "INIT" => Some(Self::Init),
            "EXECUTE_OK" => Some(Self::ExecuteOk),
            "EXECUTE_FAIL" => Some(Self::ExecuteFail),
            "COMMIT_OK" => Some(Self::CommitOk),
            "COMMIT_FAIL" => Some(Self::CommitFail),
            "ROLLBACK_OK" => Some(Self::RollbackOk),
            "ROLLBACK_FAIL" => Some(Self::RollbackFail),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrossArbitrateCmd {
    /// AUTO_CMD automatic processing according to the process
    AutoCmd = 0,
    /// EXECUTE_CMD execute the execution flow
    ExecuteCmd = 1,
    /// COMMIT_CMD execute the commit flow
    CommitCmd = 2,
    /// COMMIT_CMD execute the rollback flow
    RollbackCmd = 3,
}
impl CrossArbitrateCmd {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrossArbitrateCmd::AutoCmd => "AUTO_CMD",
            CrossArbitrateCmd::ExecuteCmd => "EXECUTE_CMD",
            CrossArbitrateCmd::CommitCmd => "COMMIT_CMD",
            CrossArbitrateCmd::RollbackCmd => "ROLLBACK_CMD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTO_CMD" => Some(Self::AutoCmd),
            "EXECUTE_CMD" => Some(Self::ExecuteCmd),
            "COMMIT_CMD" => Some(Self::CommitCmd),
            "ROLLBACK_CMD" => Some(Self::RollbackCmd),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CallType {
    Direct = 0,
    Cross = 1,
}
impl CallType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CallType::Direct => "DIRECT",
            CallType::Cross => "CROSS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DIRECT" => Some(Self::Direct),
            "CROSS" => Some(Self::Cross),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrossParams {
    Creator = 0,
    Sender = 1,
    CallType = 2,
}
impl CrossParams {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrossParams::Creator => "CREATOR",
            CrossParams::Sender => "SENDER",
            CrossParams::CallType => "CALL_TYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CREATOR" => Some(Self::Creator),
            "SENDER" => Some(Self::Sender),
            "CALL_TYPE" => Some(Self::CallType),
            _ => None,
        }
    }
}
/// methods of certificate management
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CertManageFunction {
    /// add one certificate
    CertAdd = 0,
    /// delete certificates
    CertsDelete = 1,
    /// query certificates
    CertsQuery = 2,
    /// freeze certificates
    CertsFreeze = 3,
    /// unfreeze certificates
    CertsUnfreeze = 4,
    /// revoke certificates
    CertsRevoke = 5,
    /// add one certificate alias, any
    CertAliasAdd = 6,
    /// update one certificate alias, self
    CertAliasUpdate = 7,
    /// delete certificate alias, admin
    CertsAliasDelete = 8,
    /// query certificate alias, admin
    CertsAliasQuery = 9,
}
impl CertManageFunction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CertManageFunction::CertAdd => "CERT_ADD",
            CertManageFunction::CertsDelete => "CERTS_DELETE",
            CertManageFunction::CertsQuery => "CERTS_QUERY",
            CertManageFunction::CertsFreeze => "CERTS_FREEZE",
            CertManageFunction::CertsUnfreeze => "CERTS_UNFREEZE",
            CertManageFunction::CertsRevoke => "CERTS_REVOKE",
            CertManageFunction::CertAliasAdd => "CERT_ALIAS_ADD",
            CertManageFunction::CertAliasUpdate => "CERT_ALIAS_UPDATE",
            CertManageFunction::CertsAliasDelete => "CERTS_ALIAS_DELETE",
            CertManageFunction::CertsAliasQuery => "CERTS_ALIAS_QUERY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CERT_ADD" => Some(Self::CertAdd),
            "CERTS_DELETE" => Some(Self::CertsDelete),
            "CERTS_QUERY" => Some(Self::CertsQuery),
            "CERTS_FREEZE" => Some(Self::CertsFreeze),
            "CERTS_UNFREEZE" => Some(Self::CertsUnfreeze),
            "CERTS_REVOKE" => Some(Self::CertsRevoke),
            "CERT_ALIAS_ADD" => Some(Self::CertAliasAdd),
            "CERT_ALIAS_UPDATE" => Some(Self::CertAliasUpdate),
            "CERTS_ALIAS_DELETE" => Some(Self::CertsAliasDelete),
            "CERTS_ALIAS_QUERY" => Some(Self::CertsAliasQuery),
            _ => None,
        }
    }
}
