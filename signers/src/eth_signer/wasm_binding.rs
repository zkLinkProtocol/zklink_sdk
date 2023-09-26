use crate::eth_signer::error::EthSignerError;
use wasm_bindgen::JsValue;

impl From<EthSignerError> for JsValue {
    fn from(error: EthSignerError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
