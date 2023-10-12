use crate::error::SignError;
#[cfg(not(feature = "ffi"))]
use crate::ChangePubKeyAuthRequest;
#[cfg(feature = "ffi")]
use std::sync::Arc;
#[cfg(feature = "ffi")]
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_types::basic_types::ZkLinkAddress;
#[cfg(not(feature = "ffi"))]
use zklink_sdk_types::prelude::TxSignature;
use zklink_sdk_types::tx_type::change_pubkey::Create2Data;
use zklink_sdk_types::tx_type::change_pubkey::{ChangePubKey, ChangePubKeyAuthData};
use zklink_sdk_types::tx_type::TxTrait;

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

#[cfg(not(feature = "ffi"))]
pub fn check_create2data(
    zklink_singer: &ZkLinkSigner,
    data: Create2Data,
    account_address: ZkLinkAddress,
) -> Result<(), SignError> {
    let pubkey_hash = zklink_singer.public_key().public_key_hash();
    let from_address = data.get_address(&pubkey_hash.data);
    println!("{:?} {:?}", from_address, account_address);
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

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;
    use zklink_sdk_types::prelude::H256;

    #[test]
    fn test_check_create2() {
        let creator_address =
            ZkLinkAddress::from_hex("0x6E253C951A40fAf4032faFbEc19262Cd1531A5F5").unwrap();
        let salt_arg =
            H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap();
        let code_hash =
            H256::from_str("0x4f063cd4b2e3a885f61fefb0988cc12487182c4f09ff5de374103f5812f33fe7")
                .unwrap();
        let create2_data = Create2Data {
            creator_address,
            code_hash,
            salt_arg,
        };
        let from_account =
            ZkLinkAddress::from_hex("0x4504d5BE8634e3896d42784A5aB89fc41C3d4511").unwrap();
        let eth_private_key = "43be0b8bdeccb5a13741c8fd076bf2619bfc9f6dcc43ad6cf965ab489e156ced";
        let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(eth_private_key).unwrap();

        if let Err(e) = check_create2data(&zk_signer, create2_data, from_account) {
            println!("{:?}", e)
        }
    }
}
