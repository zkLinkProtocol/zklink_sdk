use crate::error::SignError;
#[cfg(not(feature = "ffi"))]
use crate::ChangePubKeyAuthRequest;
#[cfg(feature = "ffi")]
use std::sync::Arc;
#[cfg(feature = "ffi")]
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_types::basic_types::GetBytes;
use zklink_sdk_types::basic_types::ZkLinkAddress;
#[cfg(not(feature = "ffi"))]
use zklink_sdk_types::prelude::TxSignature;
#[cfg(feature = "ffi")]
use zklink_sdk_types::tx_type::change_pubkey::Create2Data;
use zklink_sdk_types::tx_type::change_pubkey::{ChangePubKey, ChangePubKeyAuthData};

#[cfg(not(feature = "ffi"))]
pub fn sign_change_pubkey(
    eth_signer: &EthSigner,
    zklink_singer: &ZkLinkSigner,
    mut tx: ChangePubKey,
    main_contract: ZkLinkAddress,
    l1_client_id: u32,
    account_address: ZkLinkAddress,
    auth_request: ChangePubKeyAuthRequest,
) -> Result<TxSignature, SignError> {
    let eth_auth_data: Result<ChangePubKeyAuthData, _> = match auth_request {
        ChangePubKeyAuthRequest::Onchain => Ok(ChangePubKeyAuthData::Onchain),
        ChangePubKeyAuthRequest::EthECDSA => {
            let typed_data = tx.to_eip712_request_payload(l1_client_id, &main_contract)?;
            let eth_signature = eth_signer.sign_hash(typed_data.data_hash.as_ref())?;
            Ok(ChangePubKeyAuthData::EthECDSA { eth_signature })
        }
        ChangePubKeyAuthRequest::EthCreate2 { data } => {
            // check create2 data
            let pubkey_hash = zklink_singer.public_key().public_key_hash();
            let from_address = data.get_address(pubkey_hash.data.as_ref());
            if from_address.as_bytes() != account_address.as_bytes() {
                Err(SignError::IncorrectTx)
            } else {
                Ok(ChangePubKeyAuthData::EthCreate2 { data })
            }
        }
    };
    tx.eth_auth_data = eth_auth_data?;
    tx.signature = zklink_singer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}

#[cfg(feature = "ffi")]
pub fn eth_signature_of_change_pubkey(
    l1_client_id: u32,
    tx: Arc<ChangePubKey>,
    eth_signer: Arc<EthSigner>,
    main_contract: ZkLinkAddress,
) -> Result<PackedEthSignature, SignError> {
    let typed_data = tx.to_eip712_request_payload(l1_client_id, &main_contract)?;
    let eth_signature = eth_signer.sign_hash(typed_data.data_hash.as_bytes())?;
    Ok(eth_signature)
}

#[cfg(feature = "ffi")]
pub fn check_create2data(
    zklink_singer: Arc<ZkLinkSigner>,
    data: Create2Data,
    account_address: ZkLinkAddress,
) -> Result<(), SignError> {
    let pubkey_hash = zklink_singer.public_key().public_key_hash();
    let from_address = data.get_address(&pubkey_hash.data);
    if from_address.as_bytes() != account_address.as_bytes() {
        Err(SignError::IncorrectTx)
    } else {
        Ok(())
    }
}

#[cfg(feature = "ffi")]
pub fn create_signed_change_pubkey(
    zklink_singer: Arc<ZkLinkSigner>,
    tx: Arc<ChangePubKey>,
    eth_auth_data: ChangePubKeyAuthData,
) -> Result<Arc<ChangePubKey>, SignError> {
    let mut tx = (*tx).clone();
    tx.eth_auth_data = eth_auth_data;
    tx.signature = zklink_singer.sign_musig(&tx.get_bytes())?;
    Ok(Arc::new(tx))
}
