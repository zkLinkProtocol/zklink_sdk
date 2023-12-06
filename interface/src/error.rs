use thiserror::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
use zklink_sdk_signers::eth_signer::error::EthSignerError;
use zklink_sdk_signers::starknet_signer::error::StarkSignerError;
use zklink_sdk_signers::zklink_signer::error::ZkSignerError;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("EthSigning error: {0}")]
    EthSigningError(#[from] EthSignerError),
    #[error("ZkSigning error: {0}")]
    ZkSigningError(#[from] ZkSignerError),
    #[error("Starknet signing error: {0}")]
    StarkSigningError(#[from] StarkSignerError),
    #[error("Incorrect tx format")]
    IncorrectTx,
}

#[cfg(target_arch = "wasm32")]
impl From<SignError> for JsValue {
    fn from(error: SignError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
