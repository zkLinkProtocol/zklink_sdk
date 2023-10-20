use crate::error::SignError;
#[cfg(feature = "ffi")]
use std::sync::Arc;
#[cfg(feature = "ffi")]
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
#[cfg(not(feature = "web"))]
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_types::basic_types::ZkLinkAddress;
use zklink_sdk_types::signatures::TxSignature;
use zklink_sdk_types::tx_type::change_pubkey::Create2Data;
use zklink_sdk_types::tx_type::change_pubkey::{ChangePubKey, ChangePubKeyAuthData};
use zklink_sdk_types::tx_type::ZkSignatureTrait;

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

pub fn check_create2data(
    #[cfg(feature = "ffi")] zklink_singer: Arc<ZkLinkSigner>,
    #[cfg(not(feature = "ffi"))] zklink_singer: &ZkLinkSigner,
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

pub fn do_sign_change_pubkey_with_onchain_auth_data(
    mut tx: ChangePubKey,
    zklink_signer: &ZkLinkSigner,
) -> Result<TxSignature, SignError> {
    tx.sign(zklink_signer)?;
    let should_valid = tx.is_signature_valid();
    assert!(should_valid);
    // create onchain auth data
    tx.eth_auth_data = ChangePubKeyAuthData::Onchain;
    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}

pub fn do_sign_change_pubkey_with_create2data_auth(
    mut tx: ChangePubKey,
    create2data: Create2Data,
    zklink_signer: &ZkLinkSigner,
) -> Result<TxSignature, SignError> {
    tx.sign(zklink_signer)?;
    let should_valid = tx.is_signature_valid();
    assert!(should_valid);

    // create onchain auth data
    tx.eth_auth_data = ChangePubKeyAuthData::EthCreate2 { data: create2data };
    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}

#[cfg(not(feature = "web"))]
pub fn do_sign_change_pubkey_with_eth_ecdsa_auth(
    eth_signer: &EthSigner,
    zklink_signer: &ZkLinkSigner,
    mut tx: ChangePubKey,
    l1_client_id: u32,
    main_contract_address: ZkLinkAddress,
) -> Result<TxSignature, SignError> {
    tx.sign(zklink_signer)?;
    let should_valid = tx.is_signature_valid();
    assert!(should_valid);

    // create auth data
    let typed_data = tx.to_eip712_request_payload(l1_client_id, &main_contract_address)?;
    let eth_signature = eth_signer.sign_hash(typed_data.data_hash.as_bytes())?;
    tx.eth_auth_data = ChangePubKeyAuthData::EthECDSA { eth_signature };

    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}

#[cfg(not(feature = "ffi"))]
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
