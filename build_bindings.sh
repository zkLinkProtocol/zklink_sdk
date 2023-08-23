#!/bin/bash
set -euxo pipefail

SCRIPT_DIR="${SCRIPT_DIR:-$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )}"
ROOT_DIR="$SCRIPT_DIR"

BINDINGS_DIR="$ROOT_DIR/bindings/generated"
BINARIES_DIR="$ROOT_DIR/target/debug"

rm -rf $BINDINGS_DIR
mkdir -p $BINDINGS_DIR

function bindings() {
    uniffi-bindgen-go $1 --out-dir "$BINDINGS_DIR"
}

bindings bindings/zklink_crypto/src/crypto.udl

pushd $BINDINGS_DIR/..
