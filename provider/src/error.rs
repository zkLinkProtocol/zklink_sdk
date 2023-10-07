use thiserror::Error;
use jsonrpsee::core::error::Error as jsonrpseeError;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("Parse params error: {0}")]
    ParseParamsError(jsonrpseeError),
    #[error("HTTP request error: {0}")]
    RequestError(reqwest::Error),
    #[error("Parse response error: {0}")]
    ResponseError(reqwest::Error),
    #[error("Parse json value error")]
    ParseJsonError,
}
