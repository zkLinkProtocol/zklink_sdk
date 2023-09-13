pub mod eip712;
pub mod error;
pub mod eth_signature;
pub mod json_rpc_signer;
pub mod packed_eth_signature;
pub mod pk_signer;
pub mod raw_tx;
pub mod wasm_binding;

use error::EthSignerError;
use eth_signature::TxEthSignature;
pub use primitive_types::{H160, H256};
use raw_tx::RawTransaction;
pub type Address = H160;
pub use ethers_primitives::Address as EIP712Address;

#[derive(Debug, Clone)]
pub struct EthTypedData {
    pub raw_data: String,
    pub data_hash: H256,
}
