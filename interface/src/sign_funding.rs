use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::ZkSignerError;
use zklink_sdk_types::prelude::{Funding, GetBytes, TxSignature};

pub fn sign_funding(
    zklink_signer: &ZkLinkSigner,
    mut tx: Funding,
) -> Result<TxSignature, ZkSignerError> {
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}
