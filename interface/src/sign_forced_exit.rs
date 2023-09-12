#[cfg(not(feature = "ffi"))]
use crate::TxSignature;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_types::tx_type::forced_exit::ForcedExit;

#[cfg(feature = "sync")]
pub fn sign_forced_exit(
    zklink_signer: &ZkLinkSigner,
    mut tx: ForcedExit,
) -> Result<TxSignature, ZkSignerError> {
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}

#[cfg(feature = "ffi")]
pub fn create_signed_forced_exit(
    zklink_signer: Arc<ZkLinkSigner>,
    tx: Arc<ForcedExit>,
) -> Result<Arc<ForcedExit>, ZkSignerError> {
    let mut tx = (*tx).clone();
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(Arc::new(tx))
}
