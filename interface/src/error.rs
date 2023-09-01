use thiserror::Error;
use zklink_crypto::eth_signer::error::EthSignerError;
use zklink_crypto::zklink_signer::error::ZkSignerError;
use zklink_types::basic_types::ChainId;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("EthSigning error: {0}")]
    EthSigningError(EthSignerError),
    #[error("ZkSigning error: {0}")]
    ZkSigningError(ZkSignerError),
    #[error("Incorrect tx format")]
    IncorrectTx,
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
