use crate::error::SignError;
use crate::TxSignature;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_signers::eth_signer::pk_signer::EthSigner;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_types::tx_type::transfer::Transfer;
use zklink_types::tx_type::TxTrait;

pub fn sign_transfer(
    eth_signer: &EthSigner,
    zklink_syner: &ZkLinkSigner,
    mut tx: Transfer,
    token_symbol: &str,
) -> Result<TxSignature, SignError> {
    tx.signature = zklink_syner.sign_musig(&tx.get_bytes())?;
    let message = tx.get_eth_sign_msg(token_symbol).as_bytes().to_vec();
    let eth_signature = eth_signer.sign_message(&message)?;

    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: Some(eth_signature),
    })
}

#[cfg(feature = "ffi")]
pub fn create_signed_transfer(
    zklink_syner: Arc<ZkLinkSigner>,
    tx: Arc<Transfer>,
) -> Result<Arc<Transfer>, SignError> {
    let mut tx = (*tx).clone();
    tx.signature = zklink_syner.sign_musig(&tx.get_bytes())?;
    Ok(Arc::new(tx))
}
