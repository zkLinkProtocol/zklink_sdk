use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_provider::response::AccountQuery as RpcAccountQuery;
use zklink_sdk_signers::eth_signer::{EIP1271Signature, PackedEthSignature};
use zklink_sdk_signers::starknet_signer::StarkEip712Signature;
use zklink_sdk_signers::zklink_signer::PackedSignature;
use zklink_sdk_types::basic_types::AccountId;
use zklink_sdk_types::prelude::ZkLinkSignature;
use zklink_sdk_types::prelude::{PackedPublicKey, ZkLinkAddress};
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
pub struct TxZkLinkSignature {
    inner: ZkLinkSignature,
}

#[wasm_bindgen]
pub struct ZkLinkTx {
    tx_type: u8,
    tx: JsValue,
}

#[wasm_bindgen]
impl TxZkLinkSignature {
    #[wasm_bindgen(constructor)]
    pub fn new(pub_key: String, signature: String) -> Result<TxZkLinkSignature, JsValue> {
        let inner = ZkLinkSignature {
            pub_key: PackedPublicKey::from_hex(&pub_key)?,
            signature: PackedSignature::from_hex(&signature)?,
        };
        Ok(TxZkLinkSignature { inner })
    }
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
    #[wasm_bindgen(constructor)]
    pub fn new(sign_type: L1SignatureType, signature: String) -> TxLayer1Signature {
        TxLayer1Signature {
            sign_type,
            signature,
        }
    }
}

#[wasm_bindgen]
impl TxLayer1Signature {
    #[wasm_bindgen(js_name=signType)]
    pub fn sign_type(&self) -> L1SignatureType {
        self.sign_type
    }
    #[wasm_bindgen]
    pub fn signature(&self) -> String {
        self.signature.clone()
    }
}

impl TryFrom<TxLayer1Signature> for TypesTxLayer1Signature {
    type Error = JsValue;

    fn try_from(signature: TxLayer1Signature) -> Result<TypesTxLayer1Signature, Self::Error> {
        match signature.sign_type {
            L1SignatureType::Eth => Ok(TypesTxLayer1Signature::EthereumSignature(
                PackedEthSignature::from_hex(&signature.signature)?,
            )),
            L1SignatureType::Eip1271 => {
                Ok(TypesTxLayer1Signature::EIP1271Signature(EIP1271Signature(
                    hex::decode(signature.signature)
                        .map_err(|e| JsValue::from_str(&format!("error: {e}")))?,
                )))
            }
            L1SignatureType::Stark => {
                let signature = StarkEip712Signature::from_hex(&signature.signature)
                    .map_err(|e| JsValue::from_str(&format!("error: {e}")))?;

                Ok(TypesTxLayer1Signature::StarkEcdsaSignature(signature))
            }
        }
    }
}

#[wasm_bindgen]
impl TxZkLinkSignature {
    #[wasm_bindgen(js_name=pubKey)]
    pub fn pub_key(&self) -> String {
        self.inner.pub_key.as_hex()
    }
    #[wasm_bindgen]
    pub fn signature(&self) -> String {
        self.inner.signature.as_hex()
    }
}

impl From<ZkLinkSignature> for TxZkLinkSignature {
    fn from(tx: ZkLinkSignature) -> TxZkLinkSignature {
        TxZkLinkSignature { inner: tx }
    }
}

impl From<TxZkLinkSignature> for ZkLinkSignature {
    fn from(tx: TxZkLinkSignature) -> ZkLinkSignature {
        tx.inner
    }
}

#[wasm_bindgen]
impl ZkLinkTx {
    #[wasm_bindgen(constructor)]
    pub fn new(tx_type: u8, tx: JsValue) -> ZkLinkTx {
        ZkLinkTx { tx_type, tx }
    }
}

impl TryFrom<ZkLinkTx> for TypesZkLinkTx {
    type Error = JsValue;

    fn try_from(tx: ZkLinkTx) -> Result<TypesZkLinkTx, Self::Error> {
        match tx.tx_type {
            ChangePubKey::TX_TYPE => {
                let change_pubkey: ChangePubKey = serde_wasm_bindgen::from_value(tx.tx)?;
                Ok(TypesZkLinkTx::ChangePubKey(Box::new(change_pubkey)))
            }
            Transfer::TX_TYPE => {
                let transfer: Transfer = serde_wasm_bindgen::from_value(tx.tx)?;
                Ok(TypesZkLinkTx::Transfer(Box::new(transfer)))
            }
            _ => Err(JsValue::from_str(&format!("error: Invalid tx type"))),
        }
    }
}
