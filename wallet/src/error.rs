use ethers::abi::AbiError;
use ethers::abi::Error as EthAbiError;
use ethers::providers::ProviderError;
use thiserror::Error;
use wasm_bindgen::JsValue;
use zklink_sdk_signers::eth_signer::EthSignerError;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Invalid network")]
    InvalidNetwork,
    #[error("Invalid input parameter")]
    InvalidInputParameter,
    #[error("Parse json value error: {0}")]
    ParseJsonError(String),
    #[error("Get error result: {0}")]
    GetErrorResult(String),
    #[error("Abi error: {0}")]
    AbiError(#[from] AbiError),
    #[error("Layer1 provider error: {0}")]
    ProviderError(#[from] ProviderError),
    #[error("signer error: {0}")]
    EthSignerError(#[from] EthSignerError),
    #[error("Eth abi error: {0}")]
    EthAbiError(#[from] EthAbiError),
}

impl From<WalletError> for JsValue {
    fn from(error: WalletError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
