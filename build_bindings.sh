#!/bin/bash
#set -euxo pipefail

SCRIPT_DIR="${SCRIPT_DIR:-$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )}"
ROOT_DIR="$SCRIPT_DIR"

BINDINGS_DIR_DEFAULT="$ROOT_DIR/binding_tests/generated"
BINDINGS_DIR=${BINDINGS_DIR:=$BINDINGS_DIR_DEFAULT}

rm -rf $BINDINGS_DIR
mkdir -p $BINDINGS_DIR

function bindings() {
    uniffi-bindgen-go $1 --out-dir "$BINDINGS_DIR"
}

bindings crypto/src/ffi.udl
