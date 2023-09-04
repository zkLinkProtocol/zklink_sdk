//! This is implementation of a standard for hashing typed structured data for [EIP-712](https://eips.ethereum.org/EIPS/eip-712) signing standard.
//!
//! This module contains the necessary interfaces for obtaining a hash of the structure, which is later needed for EIP-712 signing.
pub mod eip712;
pub use ethers_primitives::{BytesM, Uint};
