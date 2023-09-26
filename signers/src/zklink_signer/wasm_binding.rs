use crate::zklink_signer::error::ZkSignerError;
use wasm_bindgen::JsValue;

impl From<ZkSignerError> for JsValue {
    fn from(error: ZkSignerError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
