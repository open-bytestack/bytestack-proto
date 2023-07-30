package v1

const (
	// COMMON_MAGIC_NUMBER is a magic number respects to GIF file header, and identify this is file packing by bytestack
	COMMON_MAGIC_NUMBER = 47494638

	// _DATA_RECORD_HEADER_MAGIC_START is a magic number used by data_record
	DATA_HEADER_MAGIC_START = 257758

	// _DATA_RECORD_HEADER_MAGIC_END is a magic number used by data_record
	DATA_HEADER_MAGIC_END = 857752
)
