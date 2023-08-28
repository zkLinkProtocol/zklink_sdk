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
	cd bindings/crypto && cargo build

build_bindings: build_binding_files build_binding_lib

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
LIB_DIR = ${ROOT_DIR}/target/debug

test_go: build_bindings
	export LD_LIBRARY_PATH="${LD_LIBRARY_PATH}:${LIB_DIR}"
	export CGO_LDFLAGS="-lzklink_crypto_binding -L$LIB_DIR -lm -ldl"
	export CGO_ENABLED=1
	cd ${ROOT_DIR}/binding_tests && go test  -v


