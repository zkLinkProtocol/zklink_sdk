#!/bin/bash
#set -euxo pipefail

SCRIPT_DIR="${SCRIPT_DIR:-$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )}"
ROOT_DIR="$SCRIPT_DIR"

BINDINGS_DIR_DEFAULT="$ROOT_DIR/binding_tests/generated"
BINDINGS_DIR=${BINDINGS_DIR:=$BINDINGS_DIR_DEFAULT}
LIB_DIR="$ROOT_DIR/target/debug"

rm -rf $BINDINGS_DIR
mkdir -p $BINDINGS_DIR

function bindings() {
    uniffi-bindgen-go $1 --out-dir "$BINDINGS_DIR"
}

bindings crypto/src/crypto.udl

#pushd $BINDINGS_DIR/..
#export LD_LIBRARY_PATH="${LD_LIBRARY_PATH:-}:$LIB_DIR"
#	CGO_LDFLAGS="-lzklink_sdk -L$LIB_DIR -lm -ldl" \
#	CGO_ENABLED=1 \
#	go test -v
