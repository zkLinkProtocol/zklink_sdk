//! This is implementation of a standard for hashing typed structured data for [EIP-712](https://eips.ethereum.org/EIPS/eip-712) signing standard.
//!
//! This module contains the necessary interfaces for obtaining a hash of the structure, which is later needed for EIP-712 signing.
#![allow(dead_code)]
mod eip712_standard;
mod member_collector;
mod member_types;

pub use eip712_standard::*;
use ethers::types::{Address, U256};
pub use member_collector::*;
pub use member_types::*;

#[derive(Debug, Clone)]
pub struct EIP712Domain<ADDRESS: Structuralization> {
    /// About dapp
    name: String,
    version: String,
    /// About chain id: [EIP-155](https://eips.ethereum.org/EIPS/eip-155).
    chain_id: U256,
    verifying_contract: ADDRESS,
}

impl EIP712Domain<Address> {
    pub fn new1(layer_one_chain_id: u32, verifying_contract: Address) -> Self {
        Self::new(
            "ZkLink".to_owned(),
            "1".to_owned(),
            U256::from(layer_one_chain_id),
            verifying_contract,
        )
    }
}

impl<ADDRESS: Structuralization> EIP712Domain<ADDRESS> {
    pub fn new(name: String, version: String, chain_id: U256, verifying_contract: ADDRESS) -> Self {
        Self {
            name,
            version,
            chain_id,
            verifying_contract,
        }
    }

    pub fn encode_message<MSG: EIP712>(&self, message: &MSG) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice("\x19\x01".as_bytes());
        bytes.extend_from_slice(self.hash_struct().as_bytes());
        bytes.extend_from_slice(message.hash_struct().as_bytes());
        bytes
    }
}

impl<ADDRESS: Structuralization> EIP712 for EIP712Domain<ADDRESS> {
    const STRUCT_NAME: &'static str = "EIP712Domain";

    fn absorb_member<BUILDER: AbsorbMember>(&self, builder: &mut BUILDER) {
        builder.absorb("name", &self.name);
        builder.absorb("version", &self.version);
        builder.absorb("chainId", &self.chain_id);
        builder.absorb("verifyingContract", &self.verifying_contract);
    }
}
