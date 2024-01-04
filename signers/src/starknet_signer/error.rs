use crate::RpcErr;
use thiserror::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[derive(Debug, Error)]
pub enum StarkSignerError {
    #[error("invalid starknet signer")]
    InvalidStarknetSigner,
    #[error("signature failed: {0}")]
    InvalidSignature(String),
    #[error("invalid private key:{0}")]
    InvalidPrivKey(String),
    #[error("sign error: {0}")]
    SignError(String),
    #[error("{0}")]
    RpcSignError(RpcErr),
}

impl StarkSignerError {
    pub fn invalid_signature<T: ToString>(s: T) -> Self {
        Self::InvalidSignature(s.to_string())
    }
    pub fn invalid_privkey<T: ToString>(s: T) -> Self {
        Self::InvalidPrivKey(s.to_string())
    }
    pub fn sign_error<T: ToString>(s: T) -> Self {
        Self::SignError(s.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
impl From<StarkSignerError> for JsValue {
    fn from(error: StarkSignerError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
