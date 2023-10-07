pub mod eip1271_signature;
pub mod eip712;
pub mod error;
pub mod packed_eth_signature;
pub mod pk_signer;
#[cfg(target_arch = "wasm32")]
pub mod wasm_binding;

pub use primitive_types::{H160, H256, U256};
pub type Address = H160;
pub use eip1271_signature::EIP1271Signature;
pub use error::EthSignerError;
pub use ethers_primitives::Address as EIP712Address;
pub use packed_eth_signature::PackedEthSignature;
pub use pk_signer::EthSigner;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct EthTypedData {
    pub raw_data: String,
    pub data_hash: H256,
}
