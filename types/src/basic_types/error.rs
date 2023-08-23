use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AddressError {
    #[error("Invalid zklink address")]
    InvalidAddress,
}
