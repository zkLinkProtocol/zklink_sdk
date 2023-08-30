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

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
# the ffi lib directory
LIB_DIR := ${ROOT_DIR}/target/release
# the directory of generated golang files
BINDINGS_DIR:=${ROOT_DIR}/binding_tests/generated
LD_LIBRARY_PATH := ${LD_LIBRARY_PATH}:${LIB_DIR}
BINDINGS_DIR?="${ROOT_DIR}/binding_tests/generated"
UNIFFI_VERSION=0.23.0
UNIFFI_BINDGEN_GO_VERSION=v0.1.3+v${UNIFFI_VERSION}

prepare_ffi:
	@if [[ `uniffi-bindgen-go -V | grep 'v.${UNIFFI_VERSION}'` ]]; then \
		echo "uniffi-bindgen-go ${UNIFFI_VERSION} already installed"; \
	else \
		echo "install uniffi-bindgen-go"; \
		cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag ${UNIFFI_BINDGEN_GO_VERSION}; \
	fi

prepare_wasm:
	@if [[ `wasm-pack -V` ]]; then \
		echo "wasm-pack already installed"; \
	else \
		echo "install wasm-pack"; \
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; \
	fi

build_binding_files: prepare_ffi
	rm -rf ${BINDINGS_DIR}
	uniffi-bindgen-go ${ROOT_DIR}/crypto/src/ffi.udl --out-dir ${BINDINGS_DIR}
	uniffi-bindgen-go ${ROOT_DIR}/types/src/ffi.udl --out-dir ${BINDINGS_DIR}

build_binding_lib:
	cargo build --package bindings_sdk --release

test_go: build_binding_lib build_binding_files
	cd ${ROOT_DIR}/binding_tests && \
	LD_LIBRARY_PATH=${LD_LIBRARY_PATH} \
	CGO_LDFLAGS="-lzklink_sdk -L${LIB_DIR} -lm -ldl" \
	CGO_ENABLED=1 \
	go test  -v


