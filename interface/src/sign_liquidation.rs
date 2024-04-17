use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::ZkSignerError;
use zklink_sdk_types::prelude::{GetBytes, Liquidation, TxSignature};

pub fn sign_liquidation(zklink_signer: &ZkLinkSigner, mut tx: Liquidation) -> Result<TxSignature, ZkSignerError> {
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        layer1_signature: None,
    })
}
