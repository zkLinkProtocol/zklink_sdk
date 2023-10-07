use crate::crypto::ZklinkSigner;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::{AccountId, BigUint, TokenId, ZkLinkAddress};
use zklink_sdk_types::prelude::{Nonce, SubAccountId, TimeStamp, ZkLinkSignature};
use zklink_sdk_types::tx_builder::TransferBuilder;
use zklink_sdk_types::tx_type::transfer::Transfer as TransferTx;
use zklink_sdk_types::tx_type::TxTrait;

#[wasm_bindgen]
pub struct Transfer {
    inner: TransferTx,
}

#[wasm_bindgen]
impl Transfer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        account_id: u32,
        to_address: String,
        from_sub_account_id: u8,
        to_sub_account_id: u8,
        token: u32,
        fee: String,
        amount: String,
        nonce: u32,
        ts: u32,
    ) -> Result<Transfer, JsValue> {
        let transfer_builder = TransferBuilder {
            account_id: AccountId(account_id),
            to_address: ZkLinkAddress::from_str(&to_address).unwrap(),
            from_sub_account_id: SubAccountId(from_sub_account_id),
            to_sub_account_id: SubAccountId(to_sub_account_id),
            token: TokenId(token),
            fee: BigUint::from_str(&fee).unwrap(),
            nonce: Nonce(nonce),
            timestamp: TimeStamp(ts),
            amount: BigUint::from_str(&amount).unwrap(),
        };
        Ok(Transfer {
            inner: TransferTx::new(transfer_builder),
        })
    }

    #[wasm_bindgen]
    pub fn sign(&mut self, signer: &mut ZklinkSigner) -> Result<String, JsValue> {
        let msg = self.inner.get_bytes();
        Ok(signer.sign(&msg)?)
    }

    #[wasm_bindgen(js_name = getEthSignMessage)]
    pub fn get_eth_sign_message(&self, token_symbol: String) -> String {
        self.inner.get_eth_sign_msg(&token_symbol)
    }

    #[wasm_bindgen(js_name=getTxType)]
    pub fn get_tx_type(&self) -> u8 {
        TransferTx::TX_TYPE
    }

    #[wasm_bindgen(js_name=getTx)]
    pub fn get_tx(&mut self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner).unwrap()
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
