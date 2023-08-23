use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FloatConvertError {
    #[error("Integer is too big")]
    TooBigInteger,
}
