use crate::TxSignature;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_types::tx_type::order_matching::OrderMatching;

#[cfg(feature = "sync")]
pub fn sign_order_matching(
    zklink_signer: &ZkLinkSigner,
    mut tx: OrderMatching,
) -> Result<TxSignature, ZkSignerError> {
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}

#[cfg(feature = "ffi")]
pub fn sign_order_matching(
    zklink_signer: Arc<ZkLinkSigner>,
    tx: Arc<OrderMatching>,
) -> Result<TxSignature, ZkSignerError> {
    let mut tx = (*tx).clone();
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}
