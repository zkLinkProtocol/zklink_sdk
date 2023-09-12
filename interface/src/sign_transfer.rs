use crate::error::SignError;
#[cfg(not(feature = "ffi"))]
use crate::TxSignature;
#[cfg(feature = "ffi")]
use std::sync::Arc;
#[cfg(not(feature = "ffi"))]
use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
#[cfg(not(feature = "ffi"))]
use zklink_signers::eth_signer::pk_signer::PrivateKeySigner;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_types::tx_type::transfer::Transfer;

#[cfg(feature = "sync")]
pub fn sign_transfer(
    eth_signer: &PrivateKeySigner,
    zklink_syner: &ZkLinkSigner,
    mut tx: Transfer,
    token_symbol: String,
) -> Result<TxSignature, SignError> {
    tx.signature = zklink_syner.sign_musig(&tx.get_bytes())?;
    let message = tx
        .get_ethereum_sign_message(&token_symbol)
        .as_bytes()
        .to_vec();
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
