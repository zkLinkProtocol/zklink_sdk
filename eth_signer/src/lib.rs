use crate::error::SignerError;
use crate::eth_signature::TxEthSignature;
use crate::raw_tx::RawTransaction;
use async_trait::async_trait;
use web3::types::Address;
pub use web3::types::H256;

pub mod eip1271_signature;
pub mod error;
pub mod eth_signature;
pub mod json_rpc_signer;
pub mod packed_eth_signature;
pub mod pk_signer;
pub mod raw_tx;

#[async_trait]
pub trait EthereumSigner: Send + Sync + Clone {
    async fn sign_message(&self, message: &[u8]) -> Result<TxEthSignature, SignerError>;
    async fn sign_transaction(&self, raw_tx: RawTransaction) -> Result<Vec<u8>, SignerError>;
    async fn get_address(&self) -> Result<Address, SignerError>;
}
