use thiserror::Error;

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
}
