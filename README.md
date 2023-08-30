[![Main Build Status][build-image]][build-link]
[![Audit Status][audit-image]][audit-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust Stable][rustc-image]


ZkLink SDK is an open-source cross-platform library that implements low-level cryptographic wallet functionality
for ZkLink and Ethereum-based  blockchain networks.
The codebase is primarily in Rust and provides cross-language bindings using multiple tools:

- [uniffi-bindgen-go](https://github.com/NordSecurity/uniffi-bindgen-go/tree/main) is used to generate bindings for Golang.
- [wasm-bindgen] is used to generate bindings for JavaScript and TypeScript to facilitate interactions with the Wasm modules (for web browser extensions or other web-related apps).

## Pre-requisites
Naturally, you will need the [Rust toolchain] installed.
Besides that, for [UniFFI] language bindings, you will need the corresponding language formatters:

```bash
cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.1.3+v0.23.0
```

For the JavaScript-Wasm bindings, you will need the `wasm-pack` to be installed:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Depending on your target platform, you may also need additional tooling for a given platform (e.g. Android NDK).

## Generate bindings

```bash
uniffi-bindgen-go path/to/definitions.udl
```
