/// Stat contains stat
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stat {
    /// A value greater than 0 indicates a false positive in the tx filter, support for multiple transactions in the future
    #[prost(uint32, tag = "1")]
    pub fp_count: u32,
    /// Filter query time
    #[prost(int64, tag = "2")]
    pub filter_costs: i64,
    /// DB query time
    #[prost(int64, tag = "3")]
    pub db_costs: i64,
}
