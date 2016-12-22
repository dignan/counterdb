PROTOC3_PATH ?= protoc

default:
	
build: compile_protos
	cargo build

compile_protos:
	$(PROTOC3_PATH) --rust_out=src/protocol/ src/protocol/protos/*.proto
	$(PROTOC3_PATH) --rust-grpc_out=src/protocol/ src/protocol/protos/*.proto

partserver: compile_protos
	cargo run --bin partserver
