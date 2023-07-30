pub mod v1;

/// _COMMON_MAGIC_NUMBER is a magic number respects to GIF file header, and identify this is file packing by bytestack
const _COMMON_MAGIC_NUMBER: u64 = 47494638;
/// _DATA_RECORD_HEADER_MAGIC_START is a magic number used by data_record
pub const _DATA_RECORD_HEADER_MAGIC_START: u32 = 257758;
/// _DATA_RECORD_HEADER_MAGIC_END is a magic number used by data_record
pub const _DATA_RECORD_HEADER_MAGIC_END: u32 = 857752;

pub use v1::*;