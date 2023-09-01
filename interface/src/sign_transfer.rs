use crate::error::ClientError;
use crate::TxSignature;
use web3::signing::Key;
use zklink_crypto::eth_signer::pk_signer::PrivateKeySigner;
use zklink_crypto::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_types::basic_types::TokenId;
use zklink_types::tx_type::transfer::Transfer;

#[cfg(feature = "sync")]
pub async fn sign_transfer(
    eth_signer: &PrivateKeySigner,
    zklink_syner: &ZkLinkSigner,
    tx: &mut Transfer,
    token_symbol: String,
) -> Result<TxSignature, ClientError> {
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
pub async fn sign_transfer(
    eth_signer: Arc<PrivateKeySigner>,
    zklink_syner: Arc<ZkLinkSigner>,
    tx: Arc<Transfer>,
    token_symbol: String,
) -> Result<TxSignature, ClientError> {
    let mut tx = (*tx).clone();
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
