use crate::eth_signer::{EthSignerError, PackedEthSignature};
use crate::RpcErr;
use hex;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
struct RequestArguments {
    method: String,
    params: Vec<serde_json::Value>,
}

#[wasm_bindgen]
// Rustfmt removes the 'async' keyword from async functions in extern blocks. It's fixed
// in rustfmt 2.
#[rustfmt::skip]
extern "C" {
    #[derive(Clone, Debug)]
    /// An EIP-1193 provider object. Available by convention at `window.ethereum`
    pub type Provider;

    #[wasm_bindgen(structural,catch, method)]
    async fn request(_: &Provider, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method,getter)]
    fn selectedAddress(this: &Provider) -> Option<String>;
}

pub struct JsonRpcSigner {
    provider: Provider,
}

impl JsonRpcSigner {
    pub fn new(provider: Provider) -> JsonRpcSigner {
        JsonRpcSigner { provider }
    }

    pub fn address(&self) -> Option<String> {
        self.provider.selectedAddress()
    }

    // https://docs.metamask.io/wallet/reference/provider-api/#legacy-properties
    // https://docs.metamask.io/whats-new/#march-2024
    // selectedAddress is deprecated and not all wallet extensions provide this method, use eth_requestAccounts instead
    pub async fn get_request_account(&self) -> Result<String, EthSignerError> {
        let req_params = RequestArguments {
            method: "eth_requestAccounts".to_string(),
            params: Vec::new(),
        };
        let params = serde_wasm_bindgen::to_value(&req_params)
            .map_err(|e| EthSignerError::CustomError(e.to_string()))?;
        let address = self.provider.request(params).await.map_err(|e| {
            EthSignerError::RpcSignError(serde_wasm_bindgen::from_value::<RpcErr>(e).unwrap())
        })?;
        let address = serde_wasm_bindgen::from_value::<Vec<String>>(address)
            .map_err(|e| EthSignerError::SigningFailed(e.to_string()))?;
        Ok(address[0].clone())
    }

    pub async fn sign_message(&self, message: &[u8]) -> Result<PackedEthSignature, EthSignerError> {
        let provider_address = self.get_request_account().await?;
        let mut params = Vec::new();

        // https://docs.metamask.io/wallet/reference/personal_sign/
        // Convert message to hex string since non-hex string are not well supported by all wallet extensions, result signature should be the same.
        let msg_str = format! { "0x{}", hex::encode(message) };

        params.push(serde_json::to_value(msg_str).unwrap());
        params.push(serde_json::to_value(provider_address).unwrap());
        let req_params = RequestArguments {
            method: "personal_sign".to_string(),
            params,
        };
        let params = serde_wasm_bindgen::to_value(&req_params)
            .map_err(|e| EthSignerError::CustomError(e.to_string()))?;
        let signature = self.provider.request(params).await.map_err(|e| {
            EthSignerError::RpcSignError(serde_wasm_bindgen::from_value::<RpcErr>(e).unwrap())
        })?;
        let signature = serde_wasm_bindgen::from_value::<String>(signature)
            .map_err(|e| EthSignerError::SigningFailed(e.to_string()))?;
        PackedEthSignature::from_hex(&signature)
    }

    pub async fn sign_message_eip712(
        &self,
        message: &[u8],
    ) -> Result<PackedEthSignature, EthSignerError> {
        let provider_address = self.get_request_account().await?;
        let mut params = Vec::new();
        let msg_str =
            std::str::from_utf8(message).map_err(|e| EthSignerError::CustomError(e.to_string()))?;
        params.push(serde_json::to_value(provider_address).unwrap());
        params.push(serde_json::to_value(msg_str).unwrap());
        let req_params = RequestArguments {
            method: "eth_signTypedData_v4".to_string(),
            params,
        };
        let params = serde_wasm_bindgen::to_value(&req_params)
            .map_err(|e| EthSignerError::CustomError(e.to_string()))?;
        let signature = self.provider.request(params).await.map_err(|e| {
            EthSignerError::SigningFailed(
                serde_wasm_bindgen::from_value::<String>(e).unwrap_or_default(),
            )
        })?;
        let signature = serde_wasm_bindgen::from_value::<String>(signature)
            .map_err(|e| EthSignerError::SigningFailed(e.to_string()))?;
        PackedEthSignature::from_hex(&signature)
    }
}
