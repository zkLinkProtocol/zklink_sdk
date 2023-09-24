pub mod eip1271_signature;
pub mod eip712;
pub mod error;
pub mod packed_eth_signature;
pub mod pk_signer;
#[cfg(target_arch = "wasm32")]
pub mod wasm_binding;

use error::EthSignerError;
pub use primitive_types::{H160, H256};
pub type Address = H160;
pub use ethers_primitives::Address as EIP712Address;

#[derive(Debug, Clone)]
pub struct EthTypedData {
    pub raw_data: String,
    pub data_hash: H256,
}
