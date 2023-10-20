use thiserror::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[derive(Debug, Error, PartialEq)]
pub enum TypeError {
    #[error("Invalid zklink address")]
    InvalidAddress,
    #[error("Invalid transaction hash")]
    InvalidTxHash,
    #[error("Not start with 0x")]
    NotStartWithZerox,
    #[error("Size mismatch")]
    SizeMismatch,
    #[error("{0}")]
    DecodeFromHexErr(String),
    #[error("Integer is too big")]
    TooBigInteger,
    #[error("{0}")]
    InvalidBigIntStr(String),
}

#[cfg(target_arch = "wasm32")]
impl From<TypeError> for JsValue {
    fn from(error: TypeError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
