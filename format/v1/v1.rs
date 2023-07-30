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
pub struct DataHeader {
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
    #[prost(uint32, tag = "1")]
    pub cookie: u32,
    #[prost(uint64, tag = "2")]
    pub offset_data: u64,
    #[prost(uint64, tag = "3")]
    pub size_data: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MegicRecord {
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
