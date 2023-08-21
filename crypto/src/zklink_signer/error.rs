use crate::eth_signer::packed_eth_signature::PackedETHSignatureError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("{0}")]
    CustomError(String),
    #[error("signature failed: {0}")]
    InvalidSignature(String),
    #[error("invalid private key:{0}")]
    InvalidPrivKey(String),
    #[error("invalid seed:{0}")]
    InvalidSeed(String),
    #[error("invalid eth signature: {0}")]
    PackedETHSignatureError(#[from] PackedETHSignatureError),
}

impl SignerError {
    pub fn custom_error<T: ToString>(s: T) -> Self {
        Self::CustomError(s.to_string())
    }
    pub fn invalid_signature<T: ToString>(s: T) -> Self {
        Self::InvalidSignature(s.to_string())
    }
}
