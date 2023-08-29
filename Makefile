lint:
	cargo fmt
	cargo clippy
	cargo sort
	bash -c "cd ./crypto && cargo sort"
	bash -c "cd ./interface && cargo sort"
	bash -c "cd ./types && cargo sort"
	bash -c "cd ./utils && cargo sort"
	bash -c "cd ./provider && cargo sort"
	bash -c "cd ./signer && cargo sort"
	bash -c "cd ./wallet && cargo sort"
	cargo machete

lint-check:
	cargo fmt -- --check
	cargo clippy
	cargo sort --check
	bash -c "cd ./crypto && cargo sort --check"
	bash -c "cd ./interface && cargo sort --check"
	bash -c "cd ./types && cargo sort --check"
	bash -c "cd ./utils && cargo sort --check"
	bash -c "cd ./provider && cargo sort --check"
	bash -c "cd ./signer && cargo sort --check"
	bash -c "cd ./wallet && cargo sort --check"
	cargo machete

install_tool:
	cargo install taplo-cli --locked
	cargo install cargo-sort cargo-machete

build:
	cargo build

clean:
	cargo clean

build_binding_files:
	export BINDINGS_DIR="${ROOT_DIR}/binding_tests/generated"
	sh build_bindings.sh

build_binding_lib:
	cargo build --package zklink_sdk

build_bindings: build_binding_lib build_binding_files

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
LIB_DIR = ${ROOT_DIR}/target/x86_64-apple-darwin/debug

test_go: build_bindings
	echo ${LIB_DIR}
	LD_LIBRARY_PATH="${LD_LIBRARY_PATH}:${LIB_DIR}" \
	CGO_LDFLAGS="-lzklink_sdk -L$LIB_DIR -lm -ldl" \
	CGO_ENABLED=1 \
	cd ${ROOT_DIR}/binding_tests && go test  -v


