SHELL := /bin/bash

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
	cargo test --all
	make test_go

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

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
# the ffi lib directory
LIB_DIR := ${ROOT_DIR}/target/release
# the directory of generated golang files
BINDINGS_DIR:=${ROOT_DIR}/binding_tests/generated
BINDINGS_DIR_EXAMPLE:=${ROOT_DIR}/examples/Golang/generated
LD_LIBRARY_PATH := ${LD_LIBRARY_PATH}:${LIB_DIR}
BINDINGS_DIR?="${ROOT_DIR}/binding_tests/generated"
UNIFFI_VERSION=0.23.0
UNIFFI_BINDGEN_GO_VERSION=v0.1.5+v${UNIFFI_VERSION}

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
	rm -rf ${BINDINGS_DIR} ${BINDINGS_DIR_EXAMPLE}
	uniffi-bindgen-go ${ROOT_DIR}/bindings/sdk/src/ffi.udl --out-dir ${BINDINGS_DIR} --config=${ROOT_DIR}/bindings/sdk/ffi_golang.toml
	uniffi-bindgen-go ${ROOT_DIR}/bindings/sdk/src/ffi.udl --out-dir ${BINDINGS_DIR_EXAMPLE} --config=${ROOT_DIR}/bindings/sdk/ffi_golang.toml

build_binding_lib:
	cargo build --package bindings_sdk --release

test_go: build_binding_files build_binding_lib
	cd ${ROOT_DIR}/binding_tests && \
	LD_LIBRARY_PATH=${LD_LIBRARY_PATH} \
	CGO_LDFLAGS="-lzklink_sdk -L${LIB_DIR} -lm -ldl" \
	CGO_ENABLED=1 \
	go test  -v

build_wasm: prepare_wasm
	cd ${ROOT_DIR}/bindings/wasm && \
	wasm-pack build --release --target=web --out-name=zklink-sdk-web --out-dir=web-dist -- --features web && \
    wasm-pack build --release --target=nodejs --out-name=zklink-sdk-node --out-dir=node-dist
	#wasm-pack build --release --target=bundler --out-name=zklink-bundler-node --out-dir=dist
test_wasm:
	cd ${ROOT_DIR}/bindings/wasm && \
	wasm-pack test --firefox --headless -- --test test_rpc


run_example_go_%: ${ROOT_DIR}/examples/Golang/%.go
	@cd ${ROOT_DIR}/examples/Golang && \
	LD_LIBRARY_PATH=${LD_LIBRARY_PATH} \
	CGO_LDFLAGS="-lzklink_sdk -L${LIB_DIR} -lm -ldl" \
	CGO_ENABLED=1 \
	go run $<

run_example_js_%:   ${ROOT_DIR}/examples/Javascript/node-example/%.js
	@cd ${ROOT_DIR}/examples/Javascript/node-example && \
	node $< \

GO_FILES = 1_change_pubkey 2_withdraw 3_transfer 4_forced_exit 5_order_matching 6_contract_matching 7_auto_deleveraging 8_funding 9_liquidation 10_update_global_var
RUN_GO_EXAMPLES = $(patsubst %, run_example_go_%, $(GO_FILES))
run_example_go:  ${RUN_GO_EXAMPLES}

JS_FILES = 1_change_pubkey 2_auto_deleveraging 3_update_global_var 4_contract_matching 5_liquidation 6_funding
RUN_JS_EXAMPLES = $(patsubst %, run_example_js_%, $(JS_FILES))
run_example_js: ${RUN_JS_EXAMPLES}