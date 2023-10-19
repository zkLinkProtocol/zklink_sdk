pub use ethers_primitives::Address as EIP712Address;
pub use primitive_types::{H160, H256, U256};
use serde::{Deserialize, Serialize};

pub use eip1271_signature::EIP1271Signature;
pub use error::EthSignerError;
pub use packed_eth_signature::PackedEthSignature;
pub use pk_signer::EthSigner;

pub mod eip1271_signature;
pub mod eip712;
pub mod error;
#[cfg(feature = "web")]
pub mod json_rpc_signer;
pub mod packed_eth_signature;
pub mod pk_signer;

pub type Address = H160;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthTypedData {
    pub raw_data: String,
    pub data_hash: H256,
}
