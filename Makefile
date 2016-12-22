PROTOC3_PATH ?= protoc

default:
	
build: compile_protos
	cargo build

compile_protos:
	$(PROTOC3_PATH) --rust_out=src src/protos/*.proto
	$(PROTOC3_PATH) --rust-grpc_out=src src/protos/*.proto

partserver: compile_protos
	cargo run --bin partserver
