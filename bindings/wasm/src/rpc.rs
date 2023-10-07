use zklink_provider::response::AccountQuery;
use zklink_sdk_types::basic_types::AccountId;
use zklink_sdk_types::prelude::ZkLinkAddress;
use wasm_bindgen::prelude::wasm_bindgen;
use std::str::FromStr;
use zklink_sdk_types::signatures::TxLayer1Signature;
use zklink_sdk_signers::eth_signer::{PackedEthSignature, EIP1271Signature};
use zklink_sdk_signers::starknet_signer::StarkECDSASignature;
use wasm_bindgen::JsValue;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
use zklink_sdk_types::tx_type::deposit::Deposit;
use zklink_sdk_types::tx_type::transfer::Transfer;
use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey;

#[wasm_bindgen]
#[derive(Copy,Clone)]
pub enum AccountQueryType {
    AccountId,
    Address,
}

#[wasm_bindgen]
#[derive(Copy,Clone)]
pub enum L1SignatureType {
    Eth,
    Eip1271,
    Stark
}

#[wasm_bindgen]
pub struct AccountQueryParam {
    query_type: AccountQueryType,
    query_param: String,
}

#[wasm_bindgen]
pub struct TxL1Signature {
    sign_type: L1SignatureType,
    signature: String,
}

#[wasm_bindgen]
pub struct SignedTransaction {
    tx_type: u8,
    tx: JsValue,
}

#[wasm_bindgen]
impl AccountQueryParam {
    #[wasm_bindgen(constructor)]
    pub fn new(query_type:AccountQueryType,query_param: String ) -> AccountQueryParam {
        AccountQueryParam {
            query_type,
            query_param
        }
    }
}

impl From<AccountQueryParam> for AccountQuery {
    fn from(query: AccountQueryParam) -> AccountQuery {
        match query.query_type {
            AccountQueryType::AccountId => {
                AccountQuery::Id(AccountId(query.query_param.parse::<u32>().unwrap()))
            },
            AccountQueryType::Address => {
                AccountQuery::Address(ZkLinkAddress::from_str(&query.query_param).unwrap())
            }
        }
    }
}

#[wasm_bindgen]
impl TxL1Signature {
    #[wasm_bindgen]
    pub fn new(sign_type: L1SignatureType,signature: String) -> TxL1Signature {
        TxL1Signature {
            sign_type,
            signature
        }
    }
}

impl From<TxL1Signature> for TxLayer1Signature {
    fn from(signature: TxL1Signature) -> TxLayer1Signature {
        match signature.sign_type {
            L1SignatureType::Eth => {
                TxLayer1Signature::EthereumSignature(PackedEthSignature::from_hex(&signature.signature).unwrap())
            },
            L1SignatureType::Eip1271 => {
                TxLayer1Signature::EIP1271Signature(EIP1271Signature(hex::decode(signature.signature).unwrap()))
            },
            L1SignatureType::Stark => {
                TxLayer1Signature::StarkSignature(StarkECDSASignature(hex::decode(signature.signature).unwrap()))
            }
        }
    }
}

#[wasm_bindgen]
impl SignedTransaction {
    #[wasm_bindgen(constructor)]
    pub fn new(tx_type: u8,tx: JsValue) -> SignedTransaction {
        SignedTransaction {
            tx_type,
            tx
        }
    }
}
impl From<SignedTransaction> for ZkLinkTx {
    fn from(tx: SignedTransaction) -> ZkLinkTx {
        match tx.tx_type {
            ChangePubKey::TX_TYPE => {
                let change_pubkey: ChangePubKey = serde_wasm_bindgen::from_value(tx.tx).unwrap();
                ZkLinkTx::ChangePubKey(Box::new(change_pubkey))
            },
            Transfer::TX_TYPE => {
                let transfer: Transfer = serde_wasm_bindgen::from_value(tx.tx).unwrap();
                ZkLinkTx::Transfer(Box::new(transfer))
            },
            _ => {
                panic!("Not support tx type!")
            }
        }
    }
}
