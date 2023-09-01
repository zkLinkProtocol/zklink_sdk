use crate::TxSignature;
use zklink_crypto::zklink_signer::error::ZkSignerError;
use zklink_crypto::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_types::tx_type::order_matching::OrderMatching;

#[cfg(feature = "sync")]
pub fn sign_order_matching(
    zklink_signer: &ZkLinkSigner,
    tx: &mut OrderMatchinga,
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
    tx: Arc<OrderMatchinga>,
) -> Result<TxSignature, ZkSignerError> {
    let mut tx = (*tx).clone();
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}
