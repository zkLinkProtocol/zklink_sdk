use thiserror::Error;
use zklink_sdk_signers::eth_signer::error::EthSignerError;
use zklink_sdk_signers::zklink_signer::error::ZkSignerError;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("EthSigning error: {0}")]
    EthSigningError(#[from] EthSignerError),
    #[error("ZkSigning error: {0}")]
    ZkSigningError(#[from] ZkSignerError),
    #[error("Incorrect tx format")]
    IncorrectTx,
}
