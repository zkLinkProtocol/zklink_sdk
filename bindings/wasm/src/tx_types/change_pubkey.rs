use crate::crypto::ZklinkSigner;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_sdk_types::basic_types::{AccountId, BigUint, TokenId, ZkLinkAddress};
use zklink_sdk_types::prelude::{ChainId, Nonce, SubAccountId, TimeStamp, ZkLinkSignature};
use zklink_sdk_types::tx_builder::ChangePubKeyBuilder;
use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey as ChangePubkeyTx;
use zklink_sdk_types::tx_type::TxTrait;

#[wasm_bindgen]
pub enum EthAuthType {
    OnChain,
    EthECDSA,
    EthCREATE2,
}

#[wasm_bindgen]
pub struct ChangePubKey {
    inner: ChangePubkeyTx,
}

#[wasm_bindgen]
impl ChangePubKey {
    #[wasm_bindgen(constructor)]
    pub fn new(
        chain_id: u8,
        account_id: u32,
        sub_account_id: u8,
        new_pubkey_hash: String,
        fee_token: u32,
        fee: String,
        nonce: u32,
        eth_signature: Option<String>,
        ts: u32,
    ) -> Result<ChangePubKey, JsValue> {
        let eth_signature = if let Some(signature) = eth_signature {
            Some(PackedEthSignature::from_hex(&signature)?)
        } else {
            None
        };
        let change_pubkey_builder = ChangePubKeyBuilder {
            chain_id: ChainId(chain_id),
            account_id: AccountId(account_id),
            sub_account_id: SubAccountId(sub_account_id),
            new_pubkey_hash: PubKeyHash::from_hex(&new_pubkey_hash).unwrap(),
            fee_token: TokenId(fee_token),
            fee: BigUint::from_str(&fee).unwrap(),
            nonce: Nonce(nonce),
            eth_signature,
            timestamp: TimeStamp(ts),
        };
        Ok(ChangePubKey {
            inner: ChangePubkeyTx::new(change_pubkey_builder),
        })
    }

    #[wasm_bindgen(js_name=getTxType)]
    pub fn get_tx_type(&self) -> u8 {
        ChangePubkeyTx::TX_TYPE
    }

    #[wasm_bindgen(js_name=getTx)]
    pub fn get_tx(&mut self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner).unwrap()
    }

    #[wasm_bindgen]
    pub fn sign(&mut self, signer: &mut ZklinkSigner) -> Result<String, JsValue> {
        let msg = self.inner.get_bytes();
        Ok(signer.sign(&msg)?)
    }

    #[wasm_bindgen(js_name = getChangePubkeyMessage)]
    pub fn get_change_pubkey_message(
        &self,
        layer_one_chain_id: u32,
        verifying_contract: String,
    ) -> Result<String, JsValue> {
        let contract = ZkLinkAddress::from_str(&verifying_contract).unwrap_or_default();
        let typed_data = self
            .inner
            .to_eip712_request_payload(layer_one_chain_id, &contract)?;
        Ok(typed_data.raw_data)
    }

    #[wasm_bindgen(js_name = setL2Signature)]
    pub fn set_zklink_signature(&mut self, signature: String) -> Result<(), JsValue> {
        self.inner.signature = ZkLinkSignature::from_hex(&signature)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = submitterSign)]
    pub fn submitter_sign(&mut self, signer: &mut ZklinkSigner) -> Result<String, JsValue> {
        let tx_hash = self.inner.tx_hash();
        Ok(signer.sign(&tx_hash)?)
    }
}
