use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::ZkSignerError;
use zklink_sdk_types::prelude::{AutoDeleveraging, GetBytes, TxSignature};

pub fn sign_auto_deleveraging(
    zklink_signer: &ZkLinkSigner,
    mut tx: AutoDeleveraging,
) -> Result<TxSignature, ZkSignerError> {
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}
