SHELL := /bin/bash

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
# the directory of generated ffi files
BINDINGS_DIR?=${ROOT_DIR}/bindings/generated
BINDINGS_DIR_TEST:=${ROOT_DIR}/binding_tests/generated
BINDINGS_DIR_EXAMPLE_GO:=${ROOT_DIR}/examples/Golang/generated
BINDINGS_DIR_EXAMPLE_CPP:=${ROOT_DIR}/examples/Cpp/generated
BINDINGS_DIR_EXAMPLE_PY:=${ROOT_DIR}/examples/Python/zklink_sdk.py

# the library path
LIB_DIR := ${ROOT_DIR}/target/release
LD_LIBRARY_PATH := ${LD_LIBRARY_PATH}:${LIB_DIR}

UNIFFI_VERSION=0.25.0
UNIFFI_BINDGEN_GO_VERSION=v0.2.1+v${UNIFFI_VERSION}
UNIFFI_BINDGEN_CPP_VERSION=v0.6.0+v${UNIFFI_VERSION}

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

lint:
	#cargo fmt
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

test:
	bash -c "cd ./interface && cargo test"
	bash -c "cd ./types && cargo test"
	bash -c "cd ./utils && cargo test"
	bash -c "cd ./provider && cargo test"
	bash -c "cd ./signers && cargo test"
	bash -c "cd ./wallet && cargo test"

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

clean:
	cargo clean
	rm -rf ${BINDINGS_DIR} ${BINDINGS_DIR_EXAMPLE_GO} ${BINDINGS_DIR_TEST}

prepare_ffi_go:
	@if [[ `uniffi-bindgen-go -V | grep 'v${UNIFFI_VERSION}'` ]]; then \
		echo "uniffi-bindgen-go ${UNIFFI_VERSION} already installed"; \
	else \
		echo "install uniffi-bindgen-go"; \
		cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag ${UNIFFI_BINDGEN_GO_VERSION}; \
	fi

prepare_ffi_cpp:
	@if [[ `uniffi-bindgen-cpp -V | grep 'v${UNIFFI_VERSION}'` ]]; then \
		echo "uniffi-bindgen-cpp ${UNIFFI_VERSION} already installed"; \
	else \
		echo "install uniffi-bindgen-cpp"; \
		cargo install uniffi-bindgen-cpp --git https://github.com/NordSecurity/uniffi-bindgen-cpp --tag ${UNIFFI_BINDGEN_CPP_VERSION}; \
	fi

prepare_wasm:
	@if [[ `wasm-pack -V` ]]; then \
		echo "wasm-pack already installed"; \
	else \
		echo "install wasm-pack"; \
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; \
	fi

build_binding_lib:
	cargo build --package bindings_sdk --features="uniffi_plugin" --release

build_binding_files_python:
	rm -rf ${BINDINGS_DIR} ${BINDINGS_DIR_EXAMPLE_PY}
	cargo run -p bindings_sdk --features="uniffi_builtin" --bin uniffi-bindgen -- generate ${ROOT_DIR}/bindings/sdk/src/ffi.udl --config ${ROOT_DIR}/bindings/sdk/uniffi.toml --language python --out-dir ${BINDINGS_DIR}
	cp ${BINDINGS_DIR}/*.py ${BINDINGS_DIR_EXAMPLE_PY}

build_python: build_binding_files_python build_binding_lib copy_lib_to_py_example

copy_lib_to_py_example:
	rm -f examples/Python/libzklink_sdk* && cp ./target/release/${LIB_FILE} examples/Python

build_binding_files_go: prepare_ffi_go
	rm -rf ${BINDINGS_DIR} ${BINDINGS_DIR_EXAMPLE_GO} ${BINDINGS_DIR_TEST}
	uniffi-bindgen-go ${ROOT_DIR}/bindings/sdk/src/ffi.udl --out-dir ${BINDINGS_DIR} --config=${ROOT_DIR}/bindings/sdk/uniffi.toml
	echo "Apply Temporary Go Binding Fix"
	sed -i 's/Lower(value BigUint) RustBufferI/Lower(value BigUint) RustBuffer/' ${BINDINGS_DIR}/zklink_sdk/zklink_sdk.go
	cp -r ${BINDINGS_DIR} ${BINDINGS_DIR_EXAMPLE_GO}
	cp -r ${BINDINGS_DIR} ${BINDINGS_DIR_TEST}

build_go: build_binding_files_go build_binding_lib

build_binding_files_cpp:
	rm -rf ${BINDINGS_DIR} ${BINDINGS_DIR_EXAMPLE_CPP}
	uniffi-bindgen-cpp ${ROOT_DIR}/bindings/sdk/src/ffi.udl --out-dir ${BINDINGS_DIR} --config=${ROOT_DIR}/bindings/sdk/uniffi.toml
	echo "Apply Temporary Cpp Binding Fix"
	python3 fix_cpp_header.py ${BINDINGS_DIR}/zklink_sdk.hpp
	cp -r ${BINDINGS_DIR} ${BINDINGS_DIR_EXAMPLE_CPP}

build_cpp: build_binding_files_cpp build_binding_lib

build_wasm: prepare_wasm
	cd ${ROOT_DIR}/bindings/wasm && \
	wasm-pack build --release --target=web --out-name=zklink-sdk-web --out-dir=web-dist -- --features web && \
    wasm-pack build --release --target=nodejs --out-name=zklink-sdk-node --out-dir=node-dist
	#wasm-pack build --release --target=bundler --out-name=zklink-bundler-node --out-dir=dist

test_wasm:
	cd ${ROOT_DIR}/bindings/wasm && \
	wasm-pack test --firefox --headless -- --test test_rpc

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

run_example_cpp_%: ${ROOT_DIR}/examples/Cpp/%.cpp
	@cd ${ROOT_DIR}/examples/Cpp && \
	LD_LIBRARY_PATH=${LD_LIBRARY_PATH} \
	g++ $< generated/zklink_sdk.cpp -std=c++2a -L${LIB_DIR} -lzklink_sdk && \
	LD_LIBRARY_PATH=${LD_LIBRARY_PATH} ./a.out && rm a.out

run_example_python_%: ${ROOT_DIR}/examples/Python/%.py
	@cd ${ROOT_DIR}/examples/Python && \
	python3 $<

run_example_js_%: ${ROOT_DIR}/examples/Javascript/node-example/%.js
	@cd ${ROOT_DIR}/examples/Javascript/node-example && \
	node $< \

GO_FILES = 1_change_pubkey 2_withdraw 3_transfer 4_forced_exit 5_order_matching 6_contract_matching 7_auto_deleveraging 8_funding 9_liquidation 10_update_global_var
RUN_GO_EXAMPLES = $(patsubst %, run_example_go_%, $(GO_FILES))
run_example_go:  ${RUN_GO_EXAMPLES}

PY_FILES = 1_change_pubkey 2_withdraw 3_transfer 5_order_matching
RUN_PYTHON_EXAMPLES = $(patsubst %, run_example_python_%, $(PY_FILES))
run_example_python: ${RUN_PYTHON_EXAMPLES}

JS_FILES = 1_change_pubkey 2_auto_deleveraging 3_update_global_var 4_contract_matching 5_liquidation 6_funding
RUN_JS_EXAMPLES = $(patsubst %, run_example_js_%, $(JS_FILES))
run_example_js: ${RUN_JS_EXAMPLES}

CPP_FILES = 1_change_pubkey 2_withdraw 3_transfer 5_order_matching
RUN_CPP_EXAMPLES = $(patsubst %, run_example_cpp_%, $(CPP_FILES))
run_example_cpp: ${RUN_CPP_EXAMPLES}
