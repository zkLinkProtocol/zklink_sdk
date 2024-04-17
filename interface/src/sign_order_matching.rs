use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_types::basic_types::GetBytes;
use zklink_sdk_types::prelude::TxSignature;
use zklink_sdk_types::tx_type::order_matching::OrderMatching;

pub fn sign_order_matching(zklink_signer: &ZkLinkSigner, mut tx: OrderMatching) -> Result<TxSignature, ZkSignerError> {
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        layer1_signature: None,
    })
}
