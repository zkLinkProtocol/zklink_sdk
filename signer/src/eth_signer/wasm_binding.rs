use crate::eth_signer::error::EthSignerError;
use wasm_bindgen::JsValue;

impl From<EthSignerError> for JsValue {
    fn from(error: EthSignerError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}

impl From<TxLayer1Signature> for JsValue {
    fn from(signature: TxLayer1Signature) -> Self {
        JsValue::from_str(&format!("{:?}", signature))
    }
}
