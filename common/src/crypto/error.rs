use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{0}")]
    Common(String),
    #[error("signature failed: {0}")]
    InvalidSignature(String),
    #[error("invalid private key:{0}")]
    InvalidPrivKey(String),
    #[error("invalid seed:{0}")]
    InvalidSeed(String),
}

impl Error {
    pub fn common<T: ToString>(s: T) -> Self {
        Self::Common(s.to_string())
    }
}
