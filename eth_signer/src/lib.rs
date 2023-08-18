use crate::eth_signature::TxEthSignature;
use crate::error::SignerError;
use crate::raw_tx::RawTransaction;
use web3::types::Address;
use async_trait::async_trait;

pub mod raw_tx;
pub mod pk_signer;
pub mod json_rpc_signer;
pub mod error;
pub mod packed_eth_signature;
pub mod eth_signature;
pub mod eip1271_signature;


#[async_trait]
pub trait EthereumSigner: Send + Sync + Clone {
    async fn sign_message(&self, message: &[u8]) -> Result<TxEthSignature, SignerError>;
    async fn sign_transaction(&self, raw_tx: RawTransaction) -> Result<Vec<u8>, SignerError>;
    async fn get_address(&self) -> Result<Address, SignerError>;
}