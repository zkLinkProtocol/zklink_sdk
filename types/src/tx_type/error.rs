use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FloatConvertError {
    #[error("Integer is too big")]
    TooBigInteger,
}

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Integer is too big")]
    TooBigInteger,
}
