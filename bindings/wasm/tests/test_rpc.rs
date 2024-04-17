#![cfg(target_arch = "wasm32")]
use std::str::FromStr;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;
use zklink_sdk_signers::eth_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::{PubKeyHash, ZkLinkSigner};
use zklink_sdk_types::basic_types::pack::{is_fee_amount_packable, is_token_amount_packable};
use zklink_sdk_types::basic_types::{AccountId, BigUint, ChainId, Nonce, TokenId, ZkLinkAddress};
use zklink_sdk_types::prelude::{SubAccountId, TimeStamp};
use zklink_sdk_types::tx_builder::ChangePubKeyBuilder;
use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx as TypesZkLinkTx;
use zklink_sdk_types::tx_type::ZkSignatureTrait;
use zklink_sdk_wasm::rpc_client::RpcClient;
use zklink_sdk_wasm::rpc_type_converter::{AccountQuery, AccountQueryType};
use zklink_sdk_wasm::utils::{closest_packable_transaction_amount, closest_packable_transaction_fee};

wasm_bindgen_test_configure!(run_in_worker);
#[wasm_bindgen_test]
async fn test_get_tokens() {
    let client = RpcClient::new("testnet");
    let ret = client.tokens().await;
    if let Err(e) = ret {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", e)));
    } else {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", ret.unwrap())));
    }
    // assert!(ret.is_err());
}

#[wasm_bindgen_test]
async fn test_account_query() {
    let client = RpcClient::new("testnet");
    let account_id = AccountQuery::new(AccountQueryType::AccountId, "5".to_string());
    let account_resp = client.account_query(account_id.into(), None, None).await;
    if let Err(e) = account_resp {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", e)));
    } else {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", account_resp.unwrap())));
    }
}

#[wasm_bindgen_test]
fn test_closest_packable_amount() {
    match closest_packable_transaction_amount("10000000012345678") {
        Ok(amount) => {
            web_sys::console::log_1(&JsValue::from_str(&amount));
            let amount = BigUint::from_str(&amount).unwrap();
            assert_eq!(is_token_amount_packable(&amount), true);
        }
        Err(e) => {
            web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", e)));
        }
    }
}

#[wasm_bindgen_test]
fn test_closest_packable_fee() {
    match closest_packable_transaction_fee("1234567876543") {
        Ok(amount) => {
            web_sys::console::log_1(&JsValue::from_str(&amount));
            let amount = BigUint::from_str(&amount).unwrap();
            assert_eq!(is_fee_amount_packable(&amount), true);
        }
        Err(e) => {
            web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", e)));
        }
    }
}

#[wasm_bindgen_test]
async fn test_send_change_pubkey() {
    let client = RpcClient::new("devnet");
    let private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    let eth_signer = EthSigner::try_from(private_key).unwrap();
    let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key).unwrap();
    let main_contract = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
    let l1_client_id = 80001;
    let new_pubkey_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
    let ts = 1696595303;
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
        timestamp: TimeStamp(ts),
    };
    let change_pubkey = ChangePubKey::new(builder);
    let message = change_pubkey
        .to_eip712_request_payload(l1_client_id, &ZkLinkAddress::from_str(&main_contract).unwrap())
        .unwrap();
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
        timestamp: TimeStamp(ts),
    };
    let mut tx = ChangePubKey::new(builder_with_sig);
    tx.sign(&zklink_signer).unwrap();
    //send to zklink
    let ret = client
        .send_transaction(
            serde_wasm_bindgen::to_value(&TypesZkLinkTx::ChangePubKey(Box::new(tx))).unwrap(),
            None,
        )
        .await;
    if let Err(e) = ret {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", e)));
    } else {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", ret.unwrap())));
    }
}
