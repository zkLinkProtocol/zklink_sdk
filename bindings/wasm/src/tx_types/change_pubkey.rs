use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_sdk_types::basic_types::{BigUint, ZkLinkAddress};
use zklink_sdk_types::error::TypeError;
use zklink_sdk_types::prelude::H256;
use zklink_sdk_types::tx_builder::ChangePubKeyBuilder as TxChangePubKeyBuilder;
use zklink_sdk_types::tx_type::change_pubkey::{
    ChangePubKey as ChangePubkeyTx, Create2Data as ChangePubKeyCreate2Data,
};

#[wasm_bindgen]
pub enum EthAuthType {
    OnChain,
    EthECDSA,
    EthCREATE2,
}

#[wasm_bindgen]
pub struct Create2Data {
    inner: ChangePubKeyCreate2Data,
}

#[wasm_bindgen]
pub struct ChangePubKey {
    inner: ChangePubkeyTx,
}

#[wasm_bindgen]
impl Create2Data {
    #[wasm_bindgen(constructor)]
    pub fn new(creator_address: &str, salt: &str, code_hash: &str) -> Result<Create2Data, JsValue> {
        let create2_data = ChangePubKeyCreate2Data {
            creator_address: ZkLinkAddress::from_hex(creator_address)?,
            salt_arg: H256::from_str(&salt)
                .map_err(|e| TypeError::DecodeFromHexErr(e.to_string()))?,
            code_hash: H256::from_str(code_hash)
                .map_err(|e| TypeError::DecodeFromHexErr(e.to_string()))?,
        };
        Ok(Create2Data {
            inner: create2_data,
        })
    }

    #[wasm_bindgen]
    pub fn salt(&self, pubkey_hash: &str) -> String {
        let salt_bytes = self.inner.salt(pubkey_hash.as_bytes());
        hex::encode(salt_bytes)
    }

    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
impl ChangePubKey {
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }

    pub fn get_change_pubkey_message(
        &self,
        layer_one_chain_id: u32,
        verifying_contract: String,
    ) -> Result<String, JsValue> {
        let contract = ZkLinkAddress::from_str(&verifying_contract)?;
        let typed_data = self
            .inner
            .to_eip712_request_payload(layer_one_chain_id, &contract)?;
        Ok(typed_data.raw_data)
    }
}

#[wasm_bindgen]
pub struct ChangePubKeyBuilder {
    inner: TxChangePubKeyBuilder,
}

#[wasm_bindgen]
impl ChangePubKeyBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        chain_id: u8,
        account_id: u32,
        sub_account_id: u8,
        new_pubkey_hash: String,
        fee_token: u16,
        fee: String,
        nonce: u32,
        eth_signature: Option<String>,
        ts: Option<u32>,
    ) -> Result<ChangePubKeyBuilder, JsValue> {
        let ts = if let Some(time_stamp) = ts {
            time_stamp
        } else {
            std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as u32
        };
        let eth_signature = if let Some(s) = eth_signature {
            Some(PackedEthSignature::from_hex(&s)?)
        } else {
            None
        };
        let inner = TxChangePubKeyBuilder {
            chain_id: chain_id.into(),
            account_id: account_id.into(),
            sub_account_id: sub_account_id.into(),
            new_pubkey_hash: PubKeyHash::from_hex(&new_pubkey_hash)?,
            fee_token: fee_token.into(),
            fee: BigUint::from_str(&fee).unwrap(),
            nonce: nonce.into(),
            eth_signature,
            timestamp: ts.into(),
        };
        Ok(ChangePubKeyBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> ChangePubKey {
        ChangePubKey {
            inner: ChangePubkeyTx::new(self.inner),
        }
    }
}

#[wasm_bindgen(js_name=newChangePubkey)]
pub fn new_change_pubkey(builder: ChangePubKeyBuilder) -> ChangePubKey {
    builder.build()
}
