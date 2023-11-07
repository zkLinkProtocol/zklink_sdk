use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::ZkSignerError;
use zklink_sdk_types::prelude::{ContractMatching, GetBytes, TxSignature};

pub fn sign_contract_matching(
    zklink_signer: &ZkLinkSigner,
    mut tx: ContractMatching,
) -> Result<TxSignature, ZkSignerError> {
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        layer1_signature: None,
    })
}
