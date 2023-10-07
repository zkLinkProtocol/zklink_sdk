#![cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen_test::wasm_bindgen_test;
use zklink_sdk_wasm::provider::Provider;
use zklink_provider::rpc_wasm::WasmRpcClient;
use wasm_bindgen::JsValue;
use zklink_sdk_wasm::rpc::{AccountQueryParam, AccountQueryType, SignedTransaction};
use zklink_sdk_types::prelude::{SubAccountId, TimeStamp};
use zklink_sdk_signers::eth_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::{ZkLinkSigner, PubKeyHash};
use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey;
use zklink_sdk_types::tx_builder::ChangePubKeyBuilder;
use zklink_sdk_types::basic_types::{ChainId, AccountId, TokenId, BigUint, Nonce, ZkLinkAddress};
use zklink_sdk_types::tx_type::ZkSignatureTrait;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
use std::str::FromStr;

wasm_bindgen_test_configure!(run_in_worker);
#[wasm_bindgen_test]
async fn test_get_tokens() {
    web_sys::console::log_1(&JsValue::from_str("123"));
    let client = WasmRpcClient::new("https://api-v1.zk.link".to_owned());
    let ret = client.tokens().await;
    if let Err(e) = ret {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}",e)));
    } else {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}",ret.unwrap())));
    }
    // assert!(ret.is_err());

}

#[wasm_bindgen_test]
async fn test_account_query() {
    let provider = Provider::new("testnet");
    let account_id = AccountQueryParam::new(AccountQueryType::AccountId,
        "5".to_string()
    );
    let account_resp = provider.account_query(account_id.into(),None,None).await;
    if let Err(e) = account_resp {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}",e)));
    } else {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}",account_resp.unwrap())));
    }

}

#[wasm_bindgen_test]
async fn test_send_change_pubkey() {
    web_sys::console::log_1(&JsValue::from_str("123"));
    let provider = Provider::new("testnet");
    let private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    let eth_signer = EthSigner::try_from(private_key).unwrap();
    let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key).unwrap();
    let main_contract = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
    let l1_client_id = 80001;
    let new_pubkey_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
    let ts  = 1696595303;
    //auth type 'ECDSA'
    let builder = ChangePubKeyBuilder {
        chain_id: ChainId(1),
        account_id: AccountId(10),
        sub_account_id: SubAccountId(1),
        new_pubkey_hash: PubKeyHash::from_hex(new_pubkey_hash).unwrap(),
        fee_token: TokenId(18),
        fee: BigUint::from(100000000000000u64),
        nonce: Nonce(1),
        eth_signature: None,
        timestamp: TimeStamp(ts)
    };
    let mut change_pubkey = ChangePubKey::new(builder);
    let message = change_pubkey.to_eip712_request_payload(
        l1_client_id,
        &ZkLinkAddress::from_str(&main_contract).unwrap()).unwrap();
    let signature = eth_signer.sign_message(message.raw_data.as_bytes()).unwrap();
    let builder_with_sig = ChangePubKeyBuilder {
        chain_id: ChainId(1),
        account_id: AccountId(10),
        sub_account_id: SubAccountId(1),
        new_pubkey_hash: PubKeyHash::from_hex(new_pubkey_hash).unwrap(),
        fee_token: TokenId(18),
        fee: BigUint::from(100000000000000u64),
        nonce: Nonce(1),
        eth_signature: Some(signature),
        timestamp: TimeStamp(ts)
    };
    let mut tx = ChangePubKey::new(builder_with_sig);
    tx.sign(&zklink_signer).unwrap();
    let submitter_signature = tx.signature.as_hex();
    //send to zklink
    let ret = provider.send_transaction(SignedTransaction::new(
        ChangePubKey::TX_TYPE, serde_wasm_bindgen::to_value(&tx).unwrap()), None, Some(l2_signature)).await;
    if let Err(e) = ret {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}",e)));
    } else {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}",ret.unwrap())));
    }

}