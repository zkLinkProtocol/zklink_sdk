use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("Invalid network")]
    InvalidNetwork,
    #[error("Invalid input parameter")]
    InvalidInputParameter,
    #[error("Invalid argument")]
    InvalidArgument(serde_json::Error),
    #[error("HTTP request error: {0}")]
    RequestError(reqwest::Error),
    #[error("Parse response error: {0}")]
    ResponseError(reqwest::Error),
    #[error("Parse json value error: {0}")]
    ParseJsonError(String),
    #[error("Get error result: {0}")]
    GetErrorResult(String),
}

impl From<RpcError> for JsValue {
    fn from(error: RpcError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
