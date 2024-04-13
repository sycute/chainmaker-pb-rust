/// Policy used to describe how to authenticate a specific action
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    /// rule keywords, e.g., ANY/MAJORITY/ALL/SELF/a number/a rate
    #[prost(string, tag = "1")]
    pub rule: ::prost::alloc::string::String,
    /// org_list describes the organization set included in the authentication
    #[prost(string, repeated, tag = "2")]
    pub org_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// role_list describes the role set included in the authentication
    /// e.g., admin/client/consensus/common
    #[prost(string, repeated, tag = "3")]
    pub role_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// online member of blockchain
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Member {
    /// organization identifier of the member
    #[prost(string, tag = "1")]
    pub org_id: ::prost::alloc::string::String,
    /// member type
    #[prost(enumeration = "MemberType", tag = "2")]
    pub member_type: i32,
    /// member identity related info bytes
    #[prost(bytes = "vec", tag = "3")]
    pub member_info: ::prost::alloc::vec::Vec<u8>,
}
/// full attribute member of blockchain
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberFull {
    /// organization identifier of the member
    #[prost(string, tag = "1")]
    pub org_id: ::prost::alloc::string::String,
    /// member type
    #[prost(enumeration = "MemberType", tag = "2")]
    pub member_type: i32,
    /// member identity related info bytes
    #[prost(bytes = "vec", tag = "3")]
    pub member_info: ::prost::alloc::vec::Vec<u8>,
    /// the identity of this member (non-uniqueness)
    #[prost(string, tag = "4")]
    pub member_id: ::prost::alloc::string::String,
    /// role of this member
    #[prost(string, tag = "5")]
    pub role: ::prost::alloc::string::String,
    /// the identity of this member (unique)
    #[prost(string, tag = "6")]
    pub uid: ::prost::alloc::string::String,
}
/// member extra data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberExtraData {
    /// sequence, like ethereum account nonce, by default is 0
    ///
    /// others
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberAndExtraData {
    #[prost(message, optional, tag = "1")]
    pub member: ::core::option::Option<Member>,
    #[prost(message, optional, tag = "2")]
    pub extra_data: ::core::option::Option<MemberExtraData>,
}
/// public key member's info
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkInfo {
    /// the der of the public key
    #[prost(bytes = "vec", tag = "1")]
    pub pk_bytes: ::prost::alloc::vec::Vec<u8>,
    /// member role
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
    /// member's org_id
    #[prost(string, tag = "3")]
    pub org_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MemberType {
    /// X509 cert
    Cert = 0,
    /// cert hash
    CertHash = 1,
    /// public key
    PublicKey = 2,
    /// did
    Did = 3,
    /// alias
    Alias = 4,
    /// address
    Addr = 5,
}
impl MemberType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MemberType::Cert => "CERT",
            MemberType::CertHash => "CERT_HASH",
            MemberType::PublicKey => "PUBLIC_KEY",
            MemberType::Did => "DID",
            MemberType::Alias => "ALIAS",
            MemberType::Addr => "ADDR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CERT" => Some(Self::Cert),
            "CERT_HASH" => Some(Self::CertHash),
            "PUBLIC_KEY" => Some(Self::PublicKey),
            "DID" => Some(Self::Did),
            "ALIAS" => Some(Self::Alias),
            "ADDR" => Some(Self::Addr),
            _ => None,
        }
    }
}
/// member status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MemberStatus {
    /// member's status is normal
    Normal = 0,
    /// member's status is invalid
    Invalid = 1,
    /// member's status is revoked
    Revoked = 2,
    /// member's status is frozen
    Frozen = 3,
}
impl MemberStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MemberStatus::Normal => "NORMAL",
            MemberStatus::Invalid => "INVALID",
            MemberStatus::Revoked => "REVOKED",
            MemberStatus::Frozen => "FROZEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL" => Some(Self::Normal),
            "INVALID" => Some(Self::Invalid),
            "REVOKED" => Some(Self::Revoked),
            "FROZEN" => Some(Self::Frozen),
            _ => None,
        }
    }
}
/// verify the member's relevant identity material type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VerifyType {
    /// CRL
    Crl = 0,
}
impl VerifyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VerifyType::Crl => "CRL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CRL" => Some(Self::Crl),
            _ => None,
        }
    }
}