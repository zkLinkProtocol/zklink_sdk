use thiserror::Error;
use zklink_signers::eth_signer::error::EthSignerError;
use zklink_signers::zklink_signer::error::ZkSignerError;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("EthSigning error: {0}")]
    EthSigningError(EthSignerError),
    #[error("ZkSigning error: {0}")]
    ZkSigningError(ZkSignerError),
    #[error("Incorrect tx format")]
    IncorrectTx,
}

impl From<EthSignerError> for SignError {
    fn from(err: EthSignerError) -> Self {
        SignError::EthSigningError(err)
    }
}

impl From<ZkSignerError> for SignError {
    fn from(err: ZkSignerError) -> Self {
        SignError::ZkSigningError(err)
    }
}
