pub use jsonrpsee::core::error::Error as RpcFailure;
use thiserror::Error;
use zklink_crypto::eth_signer::error::EthSignerError;
use zklink_crypto::zklink_signer::error::ZkSignerError;
use zklink_types::basic_types::ChainId;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Network '{0}' is not supported")]
    NetworkNotSupported(ChainId),
    #[error("Unable to decode server response: {0}")]
    MalformedResponse(String),
    #[error("RPC error: {0:?}")]
    RpcError(RpcFailure),
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Provided account credentials are incorrect")]
    IncorrectCredentials,
    #[error("Seed too short, must be at least 32 bytes long")]
    SeedTooShort,
    #[error("Token is not supported by zkSync")]
    UnknownToken,
    #[error("Incorrect address")]
    IncorrectAddress,

    #[error("Operation timeout")]
    OperationTimeout,
    #[error("Polling interval is too small")]
    PollingIntervalIsTooSmall,

    #[error("EthSigning error: {0}")]
    EthSigningError(EthSignerError),
    #[error("ZkSigning error: {0}")]
    ZkSigningError(ZkSignerError),
    #[error("Missing required field for a transaction: {0}")]
    MissingRequiredField(String),

    #[error("Ethereum private key was not provided for this wallet")]
    NoEthereumPrivateKey,

    #[error("Provided value is not packable")]
    NotPackableValue,

    #[error("Incorrect tx format")]
    IncorrectTx,
}

impl From<RpcFailure> for ClientError {
    fn from(err: RpcFailure) -> Self {
        ClientError::RpcError(err)
    }
}

impl From<EthSignerError> for ClientError {
    fn from(err: EthSignerError) -> Self {
        ClientError::EthSigningError(err)
    }
}

impl From<ZkSignerError> for ClientError {
    fn from(err: ZkSignerError) -> Self {
        ClientError::ZkSigningError(err)
    }
}
