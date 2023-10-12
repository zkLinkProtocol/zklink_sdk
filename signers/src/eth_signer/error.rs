use thiserror::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[derive(Debug, Error, PartialEq)]
pub enum EthSignerError {
    #[error("invalid eth signer")]
    InvalidEthSigner,
    #[error("Ethereum private key required to perform an operation")]
    MissingEthPrivateKey,
    #[error("EthereumSigner required to perform an operation")]
    MissingEthSigner,
    #[error("Signing failed: {0}")]
    SigningFailed(String),
    #[error("Unlocking failed: {0}")]
    UnlockingFailed(String),
    #[error("Decode raw transaction failed: {0}")]
    InvalidRawTx(String),
    #[error("Eip712 error: {0}")]
    Eip712Failed(String),
    #[error("Signing key is not set in account")]
    NoSigningKey,
    #[error("Address determination error")]
    DefineAddress,
    #[error("Recover address from signature failed: {0}")]
    RecoverAddress(String),
    #[error("Signature length mismatch")]
    LengthMismatched,
    #[error("Crypto Error")]
    CryptoError,
    #[error("Invalid eth signature string")]
    InvalidSignatureStr,
    #[error("{0}")]
    CustomError(String),
}

#[cfg(target_arch = "wasm32")]
impl From<EthSignerError> for JsValue {
    fn from(error: EthSignerError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
