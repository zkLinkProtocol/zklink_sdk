SHELL := /bin/bash

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
# the directory of generated ffi files
BINDINGS_DIR?=${ROOT_DIR}/bindings/generated
BINDINGS_DIR_TEST:=${ROOT_DIR}/binding_tests/generated
BINDINGS_DIR_EXAMPLE_GO:=${ROOT_DIR}/examples/Golang/generated
BINDINGS_DIR_EXAMPLE_PY:=${ROOT_DIR}/examples/Python

# the library path
LIB_DIR := ${ROOT_DIR}/target/release
LD_LIBRARY_PATH := ${LD_LIBRARY_PATH}:${LIB_DIR}

UNIFFI_VERSION=0.23.0
UNIFFI_BINDGEN_GO_VERSION=v0.1.5+v${UNIFFI_VERSION}

# check the os version
ifeq ($(OS),Windows_NT)
	LIB_FILE="libzklink_sdk.dll"
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		LIB_FILE="libzklink_sdk.so"
	endif
	ifeq ($(UNAME_S),Darwin)
		LIB_FILE="libzklink_sdk.dylib"
	endif
endif

.PHONY: lint
lint:
	cargo fmt
	cargo clippy --features ffi -- -D warnings
	cargo clippy --features web -- -D warnings
	cargo clippy -- -D warnings
	cargo sort
	bash -c "cd ./interface && cargo sort"
	bash -c "cd ./types && cargo sort"
	bash -c "cd ./utils && cargo sort"
	bash -c "cd ./provider && cargo sort"
	bash -c "cd ./signers && cargo sort"
	cargo machete

.PHONY: test
test:
	bash -c "cd ./interface && cargo test"
	bash -c "cd ./types && cargo test"
	bash -c "cd ./utils && cargo test"
	bash -c "cd ./provider && cargo test"
	bash -c "cd ./signers && cargo test"
	bash -c "cd ./wallet && cargo test"

.PHONY: check
check:
	make lint test prepare_ffi_go build_go test_go prepare_wasm build_wasm build_python copy_lib_to_py_example run_example_python

lint-check:
	cargo fmt -- --check
	cargo clippy  --all-targets
	cargo sort --check
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
	cargo build --all-targets

.PHONY: clean
clean:
	cargo clean
	rm -rf ${BINDINGS_DIR} ${BINDINGS_DIR_EXAMPLE_GO} ${BINDINGS_DIR_TEST}

.PHONY: prepare_ffi_go
prepare_ffi_go:
	@if [[ `uniffi-bindgen-go -V | grep 'v.${UNIFFI_VERSION}'` ]]; then \
		echo "uniffi-bindgen-go ${UNIFFI_VERSION} already installed"; \
	else \
		echo "install uniffi-bindgen-go"; \
		cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag ${UNIFFI_BINDGEN_GO_VERSION}; \
	fi

.PHONY: prepare_wasm
prepare_wasm:
	@if [[ `wasm-pack -V` ]]; then \
		echo "wasm-pack already installed"; \
	else \
		echo "install wasm-pack"; \
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; \
	fi

.PHONY: build_binding_files_go
build_binding_files_go: prepare_ffi_go
	rm -rf ${BINDINGS_DIR} ${BINDINGS_DIR_EXAMPLE_GO} ${BINDINGS_DIR_TEST}
	uniffi-bindgen-go ${ROOT_DIR}/bindings/sdk/src/ffi.udl --out-dir ${BINDINGS_DIR} --config=${ROOT_DIR}/bindings/sdk/uniffi.toml
	uniffi-bindgen-go ${ROOT_DIR}/bindings/sdk/src/ffi.udl --out-dir ${BINDINGS_DIR_EXAMPLE_GO} --config=${ROOT_DIR}/bindings/sdk/uniffi.toml
	uniffi-bindgen-go ${ROOT_DIR}/bindings/sdk/src/ffi.udl --out-dir ${BINDINGS_DIR_TEST} --config=${ROOT_DIR}/bindings/sdk/uniffi.toml

.PHONY: build_binding_files_python
build_binding_files_python:
	cargo run -p bindings_sdk --features="python" --bin uniffi-bindgen -- generate ${ROOT_DIR}/bindings/sdk/src/ffi.udl --config ${ROOT_DIR}/bindings/sdk/uniffi.toml --language python --out-dir ${BINDINGS_DIR}
	cargo run -p bindings_sdk --features="python" --bin uniffi-bindgen -- generate ${ROOT_DIR}/bindings/sdk/src/ffi.udl --config ${ROOT_DIR}/bindings/sdk/uniffi.toml --language python --out-dir ${BINDINGS_DIR_EXAMPLE_PY}

.PHONY: build_binding_lib_go
build_binding_lib_go:
	cargo build --package bindings_sdk --features="golang" --release

.PHONY: build_binding_lib_python
build_binding_lib_python:
	cargo build --package bindings_sdk --features="python" --release

.PHONY: build_python
build_python: build_binding_files_python build_binding_lib_python

.PHONY: copy_lib_to_py_example
copy_lib_to_py_example:
	rm -f examples/Python/libzklink_sdk* && cp ./target/release/${LIB_FILE} examples/Python

.PHONY: build_go
build_go: build_binding_files_go build_binding_lib_go

.PHONY: build_wasm
build_wasm: prepare_wasm
	cd ${ROOT_DIR}/bindings/wasm && \
	wasm-pack build --release --target=web --out-name=zklink-sdk-web --out-dir=${BINDINGS_DIR}/wasm/web-dist -- --features web && \
    wasm-pack build --release --target=nodejs --out-name=zklink-sdk-node --out-dir=${BINDINGS_DIR}/wasm/node-dist
	#wasm-pack build --release --target=bundler --out-name=zklink-bundler-node --out-dir=dist

.PHONY: test_wasm
test_wasm:
	cd ${ROOT_DIR}/bindings/wasm && \
	wasm-pack test --firefox --headless -- --test test_rpc

.PHONY: test_go
test_go: build_go
	cd ${ROOT_DIR}/binding_tests && \
	LD_LIBRARY_PATH=${LD_LIBRARY_PATH} \
	CGO_LDFLAGS="-lzklink_sdk -L${LIB_DIR} -lm -ldl" \
	CGO_ENABLED=1 \
	go test  -v

run_example_go_%: ${ROOT_DIR}/examples/Golang/%.go
	@cd ${ROOT_DIR}/examples/Golang && \
	LD_LIBRARY_PATH=${LD_LIBRARY_PATH} \
	CGO_LDFLAGS="-lzklink_sdk -L${LIB_DIR} -lm -ldl" \
	CGO_ENABLED=1 \
	go run $<

run_example_python_%: ${ROOT_DIR}/examples/Python/%.py
	@cd ${ROOT_DIR}/examples/Python && \
	python3 $<

run_example_js_%: ${ROOT_DIR}/examples/Javascript/node-example/%.js
	@cd ${ROOT_DIR}/examples/Javascript/node-example && \
	node $< \

GO_FILES = 1_change_pubkey 2_withdraw 3_transfer 4_forced_exit 5_order_matching 6_contract_matching 7_auto_deleveraging 8_funding 9_liquidation 10_update_global_var
RUN_GO_EXAMPLES = $(patsubst %, run_example_go_%, $(GO_FILES))
run_example_go:  ${RUN_GO_EXAMPLES}

PY_FILES = 1_change_pubkey 2_withdraw 5_order_matching
RUN_PYTHON_EXAMPLES = $(patsubst %, run_example_python_%, $(PY_FILES))
run_example_python: ${RUN_PYTHON_EXAMPLES}

JS_FILES = 1_change_pubkey 2_auto_deleveraging 3_update_global_var 4_contract_matching 5_liquidation 6_funding
RUN_JS_EXAMPLES = $(patsubst %, run_example_js_%, $(JS_FILES))
run_example_js: ${RUN_JS_EXAMPLES}
