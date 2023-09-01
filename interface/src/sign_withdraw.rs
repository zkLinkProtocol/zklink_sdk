use crate::TxSignature;
use zklink_crypto::eth_signer::pk_signer::PrivateKeySigner;
use zklink_crypto::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_types::tx_type::withdraw::Withdraw;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use crate::error::SignError;

#[cfg(feature = "sync")]
pub fn sign_withdraw(
    eth_signer: &PrivateKeySigner,
    zklink_singer: &ZkLinkSigner,
    mut tx: Withdraw,
    l2_source_token_symbol: String,
) -> Result<TxSignature, SignError> {
    tx.signature = zklink_singer.sign_musig(&tx.get_bytes())?;
    let message = tx.get_ethereum_sign_message(&l2_source_token_symbol);
    let eth_signature = eth_signer.sign_message(message.as_bytes())?;

    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: Some(eth_signature),
    })
}

#[cfg(feature = "ffi")]
pub fn sign_withdraw(
    eth_signer: Arc<PrivateKeySigner>,
    zklink_singer: Arc<ZkLinkSigner>,
    tx: Arc<Withdraw>,
    l2_source_token_symbol: String,
) -> Result<TxSignature, SignError> {
    let mut tx = (*tx).clone();
    tx.signature = zklink_singer.sign_musig(&tx.get_bytes())?;
    let message = tx.get_ethereum_sign_message(&l2_source_token_symbol);
    let eth_signature = eth_signer.sign_message(message.as_bytes())?;

    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: Some(eth_signature),
    })
}
