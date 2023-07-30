.PHONY: proto

proto:
	export OUT_DIR=format/v1 && cargo run --bin gen-format-v1
	protoc --go_out=format/v1/ format/v1/v1.proto
