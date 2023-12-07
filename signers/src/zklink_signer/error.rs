use crate::eth_signer::error::EthSignerError;
use crate::starknet_signer::error::StarkSignerError;
use thiserror::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[derive(Debug, Error)]
pub enum ZkSignerError {
    #[error("{0}")]
    CustomError(String),
    #[error("signature failed: {0}")]
    InvalidSignature(String),
    #[error("invalid private key:{0}")]
    InvalidPrivKey(String),
    #[error("invalid seed:{0}")]
    InvalidSeed(String),
    #[error("invalid public key:{0}")]
    InvalidPubkey(String),
    #[error("invalid public key hash:{0}")]
    InvalidPubkeyHash(String),
    #[error("{0}")]
    EthSignerError(#[from] EthSignerError),
    #[error("{0}")]
    StarkSignerError(#[from] StarkSignerError),
}

impl ZkSignerError {
    pub fn custom_error<T: ToString>(s: T) -> Self {
        Self::CustomError(s.to_string())
    }
    pub fn invalid_signature<T: ToString>(s: T) -> Self {
        Self::InvalidSignature(s.to_string())
    }
    pub fn invalid_privkey<T: ToString>(s: T) -> Self {
        Self::InvalidPrivKey(s.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
impl From<ZkSignerError> for JsValue {
    fn from(error: ZkSignerError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
