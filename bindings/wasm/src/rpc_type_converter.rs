use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_provider::response::AccountQuery as RpcAccountQuery;
use zklink_sdk_signers::eth_signer::{EIP1271Signature, PackedEthSignature};
use zklink_sdk_signers::starknet_signer::StarkECDSASignature;
use zklink_sdk_types::basic_types::AccountId;
use zklink_sdk_types::prelude::ZkLinkAddress;
use zklink_sdk_types::signatures::TxLayer1Signature as TypesTxLayer1Signature;
use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey;
use zklink_sdk_types::tx_type::transfer::Transfer;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx as TypesZkLinkTx;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum AccountQueryType {
    AccountId,
    Address,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum L1SignatureType {
    Eth,
    Eip1271,
    Stark,
}

#[wasm_bindgen]
pub struct AccountQuery {
    query_type: AccountQueryType,
    query_param: String,
}

#[wasm_bindgen]
pub struct TxLayer1Signature {
    sign_type: L1SignatureType,
    signature: String,
}

#[wasm_bindgen]
pub struct ZkLinkTx {
    tx_type: u8,
    tx: JsValue,
}

#[wasm_bindgen]
impl AccountQuery {
    #[wasm_bindgen(constructor)]
    pub fn new(query_type: AccountQueryType, query_param: String) -> AccountQuery {
        AccountQuery {
            query_type,
            query_param,
        }
    }
}

impl From<AccountQuery> for RpcAccountQuery {
    fn from(query: AccountQuery) -> RpcAccountQuery {
        match query.query_type {
            AccountQueryType::AccountId => {
                RpcAccountQuery::Id(AccountId(query.query_param.parse::<u32>().unwrap()))
            }
            AccountQueryType::Address => {
                RpcAccountQuery::Address(ZkLinkAddress::from_str(&query.query_param).unwrap())
            }
        }
    }
}

#[wasm_bindgen]
impl TxLayer1Signature {
    #[wasm_bindgen]
    pub fn new(sign_type: L1SignatureType, signature: String) -> TxLayer1Signature {
        TxLayer1Signature {
            sign_type,
            signature,
        }
    }
}

impl From<TxLayer1Signature> for TypesTxLayer1Signature {
    fn from(signature: TxLayer1Signature) -> TypesTxLayer1Signature {
        match signature.sign_type {
            L1SignatureType::Eth => TypesTxLayer1Signature::EthereumSignature(
                PackedEthSignature::from_hex(&signature.signature).unwrap(),
            ),
            L1SignatureType::Eip1271 => TypesTxLayer1Signature::EIP1271Signature(EIP1271Signature(
                hex::decode(signature.signature).unwrap(),
            )),
            L1SignatureType::Stark => TypesTxLayer1Signature::StarkSignature(StarkECDSASignature(
                hex::decode(signature.signature).unwrap(),
            )),
        }
    }
}

#[wasm_bindgen]
impl ZkLinkTx {
    #[wasm_bindgen(constructor)]
    pub fn new(tx_type: u8, tx: JsValue) -> ZkLinkTx {
        ZkLinkTx { tx_type, tx }
    }
}

impl From<ZkLinkTx> for TypesZkLinkTx {
    fn from(tx: ZkLinkTx) -> TypesZkLinkTx {
        match tx.tx_type {
            ChangePubKey::TX_TYPE => {
                let change_pubkey: ChangePubKey = serde_wasm_bindgen::from_value(tx.tx).unwrap();
                TypesZkLinkTx::ChangePubKey(Box::new(change_pubkey))
            }
            Transfer::TX_TYPE => {
                let transfer: Transfer = serde_wasm_bindgen::from_value(tx.tx).unwrap();
                TypesZkLinkTx::Transfer(Box::new(transfer))
            }
            _ => {
                panic!("Not support tx type!")
            }
        }
    }
}
