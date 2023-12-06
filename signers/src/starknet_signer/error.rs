use thiserror::Error;

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
    fn from(error: ZkSignerError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
