use crate::error::SignError;
use zklink_sdk_signers::zklink_signer::{ZkLinkSignature, ZkLinkSigner};
use zklink_sdk_types::tx_type::change_pubkey::Create2Data;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;

pub mod error;
#[cfg(feature = "web")]
pub mod json_rpc_signer;
pub mod sign_auto_deleveraging;
pub mod sign_change_pubkey;
pub mod sign_contract_matching;
pub mod sign_forced_exit;
pub mod sign_funding;
pub mod sign_liquidation;
pub mod sign_order_matching;
pub mod sign_transfer;
pub mod sign_withdraw;
#[cfg(not(feature = "web"))]
pub mod signer;

pub enum ChangePubKeyAuthRequest {
    Onchain,
    EthECDSA,
    EthCreate2 { data: Create2Data },
}

pub fn do_submitter_signature(
    zklink_signer: &ZkLinkSigner,
    zklink_tx: &ZkLinkTx,
) -> Result<ZkLinkSignature, SignError> {
    let tx_hash = zklink_tx.tx_hash();
    let signature = zklink_signer.sign_musig(tx_hash.as_ref())?;
    Ok(signature)
}
