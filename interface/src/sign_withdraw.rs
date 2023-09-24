use crate::error::SignError;
use crate::TxSignature;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_sdk_types::tx_type::withdraw::Withdraw;
use zklink_signers::eth_signer::pk_signer::EthSigner;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;

pub fn sign_withdraw(
    eth_signer: &EthSigner,
    zklink_singer: &ZkLinkSigner,
    mut tx: Withdraw,
    l2_source_token_symbol: &str,
) -> Result<TxSignature, SignError> {
    tx.sign(zklink_singer)?;
    let message = tx.get_eth_sign_msg(l2_source_token_symbol);
    let eth_signature = eth_signer.sign_message(message.as_bytes())?;

    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: Some(eth_signature),
    })
}

#[cfg(feature = "ffi")]
pub fn create_signed_withdraw(
    zklink_singer: Arc<ZkLinkSigner>,
    tx: Arc<Withdraw>,
) -> Result<Arc<Withdraw>, SignError> {
    let mut tx = (*tx).clone();
    tx.signature = zklink_singer.sign_musig(&tx.get_bytes())?;
    Ok(Arc::new(tx))
}
