use thiserror::Error;
use zklink_provider::RpcError;
use zklink_signers::eth_signer::error::{EthSignerError, RpcFailure};
use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_types::basic_types::ChainId;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Network '{0}' is not supported")]
    NetworkNotSupported(ChainId),
    #[error("Unable to decode server response: {0}")]
    MalformedResponse(String),
    #[error("RPC error: {0:?}")]
    RpcError(#[from] RpcError),
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
    EthSigningError(#[from] EthSignerError),
    #[error("ZkSigning error: {0}")]
    ZkSigningError(#[from] ZkSignerError),
    #[error("Missing required field for a transaction: {0}")]
    MissingRequiredField(String),

    #[error("Ethereum private key was not provided for this wallet")]
    NoEthereumPrivateKey,

    #[error("Provided value is not packable")]
    NotPackableValue,

    #[error("Incorrect tx format")]
    IncorrectTx,
}
