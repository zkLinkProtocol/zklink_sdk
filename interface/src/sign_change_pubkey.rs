use crate::error::SignError;
use crate::{ChangePubKeyAuthRequest, TxSignature};
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_crypto::eth_signer::pk_signer::PrivateKeySigner;
use zklink_crypto::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_types::basic_types::ZkLinkAddress;
use zklink_types::tx_type::change_pubkey::{ChangePubKey, ChangePubKeyAuthData};

#[cfg(feature = "sync")]
pub fn sign_change_pubkey(
    eth_signer: &PrivateKeySigner,
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
            let eth_signature = eth_signer.sign_typed_data(&typed_data)?;
            Ok(ChangePubKeyAuthData::EthECDSA { eth_signature })
        }
        ChangePubKeyAuthRequest::EthCreate2 { data } => {
            // check create2 data
            let pubkey_hash = zklink_singer.public_key().public_key_hash();
            let from_address = data.get_address(pubkey_hash.data.to_vec());
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
pub fn sign_change_pubkey(
    eth_signer: Arc<PrivateKeySigner>,
    zklink_singer: Arc<ZkLinkSigner>,
    tx: Arc<ChangePubKey>,
    main_contract: ZkLinkAddress,
    l1_client_id: u32,
    account_address: ZkLinkAddress,
    auth_request: ChangePubKeyAuthRequest,
) -> Result<TxSignature, SignError> {
    let mut tx = (*tx).clone();
    let eth_auth_data: Result<ChangePubKeyAuthData, _> = match auth_request {
        ChangePubKeyAuthRequest::Onchain => Ok(ChangePubKeyAuthData::Onchain),
        ChangePubKeyAuthRequest::EthECDSA => {
            let typed_data = tx.to_eip712_request_payload(l1_client_id, &main_contract)?;
            let eth_signature = eth_signer.sign_typed_data(&typed_data)?;

            Ok(ChangePubKeyAuthData::EthECDSA { eth_signature })
        }
        ChangePubKeyAuthRequest::EthCreate2 { data } => {
            // check create2 data
            let pubkey_hash = zklink_singer.public_key().public_key_hash();
            let from_address = data.get_address(pubkey_hash.data.to_vec());
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
