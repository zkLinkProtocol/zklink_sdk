#[cfg(target_arch = "wasm32")]
pub mod crypto;
#[cfg(target_arch = "wasm32")]
pub mod signer;
// #[cfg(target_arch = "wasm32")]
pub mod tx_types;
#[cfg(target_arch = "wasm32")]
pub mod provider;
pub mod rpc;
// #[cfg(not(target_arch = "wasm32"))]
// pub mod wallet;
// #[cfg(not(target_arch = "wasm32"))]
// pub mod error;

extern crate getrandom;
