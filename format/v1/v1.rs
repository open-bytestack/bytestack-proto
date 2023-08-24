#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MagicHeader {
    #[prost(uint64, tag = "1")]
    pub magic_number: u64,
    #[prost(uint64, tag = "2")]
    pub stack_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRecordHeader {
    #[prost(uint32, tag = "1")]
    pub magic_start: u32,
    #[prost(uint32, tag = "2")]
    pub cookie: u32,
    #[prost(uint32, tag = "3")]
    pub size: u32,
    #[prost(uint32, tag = "4")]
    pub crc: u32,
    #[prost(uint32, tag = "5")]
    pub magic_end: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexRecord {
    #[prost(uint64, tag = "1")]
    pub offset_data: u64,
    #[prost(uint32, tag = "2")]
    pub size_data: u32,
    #[prost(uint32, tag = "3")]
    pub cookie: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaRecord {
    #[prost(int64, tag = "1")]
    pub create_time: i64,
    #[prost(uint64, tag = "2")]
    pub offset_data: u64,
    #[prost(uint32, tag = "3")]
    pub size_data: u32,
    #[prost(uint32, tag = "4")]
    pub cookie: u32,
    #[prost(string, tag = "5")]
    pub filename: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MagicNumber {
    None = 0,
    HeaderMagic = 199809,
    DataMagicStart = 257758,
    DataMagicEnd = 857752,
}
impl MagicNumber {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MagicNumber::None => "None",
            MagicNumber::HeaderMagic => "HeaderMagic",
            MagicNumber::DataMagicStart => "DataMagicStart",
            MagicNumber::DataMagicEnd => "DataMagicEnd",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "None" => Some(Self::None),
            "HeaderMagic" => Some(Self::HeaderMagic),
            "DataMagicStart" => Some(Self::DataMagicStart),
            "DataMagicEnd" => Some(Self::DataMagicEnd),
            _ => None,
        }
    }
}
