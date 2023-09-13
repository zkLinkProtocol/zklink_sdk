#![cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
use crate::eth_signer::error::EthSignerError;
use crate::eth_signer::eth_signature::TxEthSignature;

impl From<EthSignerError> for JsValue {
    fn from(error: EthSignerError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}

impl From<TxEthSignature> for JsValue {
    fn from(signature: TxEthSignature) -> Self {
        JsValue::from_str(&format!("{:?}",signature))
    }
}