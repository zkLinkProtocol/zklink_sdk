use serde_json::Value;
use std::sync::Arc;
use zklink_interface::error::SignError;
use zklink_signers::eth_signer::pk_signer::PrivateKeySigner;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_types::basic_types::ZkLinkAddress;
use zklink_types::tx_builder::{
    ChangePubKeyBuilder, DepositBuilder, ForcedExitBuilder, FullExitBuilder, OrderMatchingBuilder,
    TransferBuilder, WithdrawBuilder,
};
use zklink_types::tx_type::change_pubkey::{ChangePubKey, ChangePubKeyAuthData, Create2Data};
use zklink_types::tx_type::deposit::Deposit;
use zklink_types::tx_type::forced_exit::ForcedExit;
use zklink_types::tx_type::full_exit::FullExit;
use zklink_types::tx_type::order_matching::OrderMatching;
use zklink_types::tx_type::transfer::Transfer;
use zklink_types::tx_type::withdraw::Withdraw;
use zklink_types::tx_type::zklink_tx::ZkLinkTx;

pub fn build_change_pubkey_request_with_create2data(
    private_key: &str,
    builder: ChangePubKeyBuilder,
    create2data: Create2Data,
) -> Result<String, SignError> {
    let mut tx = ChangePubKey::new(builder);
    let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key)?;

    tx.sign(&zklink_signer)?;
    let should_valid = tx.is_signature_valid()?;
    assert!(should_valid);

    // create onchain auth data
    // TODO: check create2data address
    tx.eth_auth_data = ChangePubKeyAuthData::EthCreate2 { data: create2data };

    // create submitter signature
    let bytes = tx.tx_hash();
    let submitter_signature = zklink_signer.sign_musig(&bytes)?;

    // build rpc request
    let zklink_tx: ZkLinkTx = tx.into();
    let request = (
        serde_json::to_value(zklink_tx).unwrap(),
        Value::Null,
        serde_json::to_value(submitter_signature).unwrap(),
    );
    let s = serde_json::to_string(&request).unwrap();
    Ok(s)
}

pub fn build_change_pubkey_request_with_onchain_auth_data(
    private_key: &str,
    builder: ChangePubKeyBuilder,
) -> Result<String, SignError> {
    let mut tx = ChangePubKey::new(builder);
    let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key)?;

    tx.sign(&zklink_signer)?;
    let should_valid = tx.is_signature_valid()?;
    assert!(should_valid);

    // create onchain auth data
    tx.eth_auth_data = ChangePubKeyAuthData::OnChain;

    // create submitter signature
    let bytes = tx.tx_hash();
    let submitter_signature = zklink_signer.sign_musig(&bytes)?;

    // build rpc request
    let zklink_tx: ZkLinkTx = tx.into();
    let request = [
        serde_json::to_value(zklink_tx).unwrap(),
        Value::Null,
        serde_json::to_value(submitter_signature).unwrap(),
    ];
    let s = serde_json::to_string(&request).unwrap();
    Ok(s)
}

pub fn build_change_pubkey_request_with_eth_ecdsa_auth_data(
    private_key: &str,
    builder: ChangePubKeyBuilder,
    l1_client_id: u32,
    main_contract_address: ZkLinkAddress,
) -> Result<String, SignError> {
    let eth_signer = PrivateKeySigner::new(private_key)?;
    let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key)?;
    let mut tx = ChangePubKey::new(builder);
    tx.sign(&zklink_signer)?;
    let should_valid = tx.is_signature_valid()?;
    assert!(should_valid);

    // create auth data
    let typed_data = tx.to_eip712_request_payload(l1_client_id, &main_contract_address)?;
    let eth_signature = eth_signer.sign_byted_data(&typed_data.data_hash)?;
    tx.eth_auth_data = ChangePubKeyAuthData::EthECDSA { eth_signature };

    // create submitter signature
    let bytes = tx.tx_hash();
    let submitter_signature = zklink_signer.sign_musig(&bytes)?;

    // build rpc request
    let zklink_tx: ZkLinkTx = tx.into();
    let request = [
        serde_json::to_value(zklink_tx).unwrap(),
        Value::Null,
        serde_json::to_value(submitter_signature).unwrap(),
    ];
    let s = serde_json::to_string(&request).unwrap();
    Ok(s)
}

pub fn build_transfer_request(
    private_key: &str,
    builder: TransferBuilder,
    token_symbol: &str,
) -> Result<String, SignError> {
    let eth_signer = PrivateKeySigner::new(private_key)?;
    let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key)?;

    // create  Transfer and sign it
    let mut tx = Transfer::new(builder);
    tx.sign(&zklink_signer)?;

    // create the signature
    let eth_signature = tx.eth_signature(Arc::new(eth_signer), token_symbol)?;

    // create submitter signature
    let tx_hash = tx.tx_hash();
    let submitter_signature = zklink_signer.sign_musig(&tx_hash)?;

    let zklink_tx: ZkLinkTx = tx.into();
    let req = [
        serde_json::to_value(zklink_tx).unwrap(),
        serde_json::to_value(eth_signature).unwrap(),
        serde_json::to_value(submitter_signature).unwrap(),
    ];
    let s = serde_json::to_string(&req).unwrap();
    Ok(s)
}

pub fn build_deposit_request(builder: DepositBuilder) -> Result<String, SignError> {
    let tx = Deposit::new(builder);
    let zklink_tx: ZkLinkTx = tx.into();
    let req = [serde_json::to_value(zklink_tx).unwrap(), Value::Null, Value::Null];
    let s = serde_json::to_string(&req).unwrap();
    Ok(s)
}

pub fn build_withdraw_request(
    private_key: &str,
    builder: WithdrawBuilder,
) -> Result<String, SignError> {
    let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key)?;
    let mut tx = Withdraw::new(builder);
    tx.sign(&zk_signer)?;
    let zklink_tx: ZkLinkTx = tx.into();
    let req = [serde_json::to_value(zklink_tx).unwrap(), Value::Null, Value::Null];
    let s = serde_json::to_string(&req).unwrap();
    Ok(s)
}

pub fn build_forced_exit_request(
    private_key: &str,
    builder: ForcedExitBuilder,
) -> Result<String, SignError> {
    let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key)?;
    let mut tx = ForcedExit::new(builder);
    tx.sign(&zklink_signer)?;
    let zklink_tx: ZkLinkTx = tx.into();
    let req = [serde_json::to_value(zklink_tx).unwrap(), Value::Null, Value::Null];
    let s = serde_json::to_string(&req).unwrap();
    Ok(s)
}

pub fn build_full_exit_request(builder: FullExitBuilder) -> Result<String, SignError> {
    let tx = FullExit::new(builder);
    let zklink_tx: ZkLinkTx = tx.into();
    let req = [serde_json::to_value(zklink_tx).unwrap(), Value::Null, Value::Null];
    let s = serde_json::to_string(&req).unwrap();
    Ok(s)
}

pub fn build_order_matching_request(
    private_key: &str,
    builder: OrderMatchingBuilder,
) -> Result<String, SignError> {
    let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key)?;
    let mut tx = OrderMatching::new(builder);
    tx.sign(&zklink_signer)?;
    let zklink_tx: ZkLinkTx = tx.into();
    let req = [serde_json::to_value(zklink_tx).unwrap(), Value::Null, Value::Null];
    let s = serde_json::to_string(&req).unwrap();
    Ok(s)
}
