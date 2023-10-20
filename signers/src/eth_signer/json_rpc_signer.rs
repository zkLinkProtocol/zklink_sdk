use crate::eth_signer::{EthSignerError, PackedEthSignature};
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

    #[wasm_bindgen(catch, method)]
    async fn request(_: &Provider, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method,getter)]
    fn selectedAddress(this: &Provider) -> Option<String>;
}

#[wasm_bindgen(inline_js = "export function get_provider_js() {return window.ethereum}")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn get_provider_js() -> Result<Option<Provider>, JsValue>;
}

pub fn get_provider() -> Result<Option<Provider>, EthSignerError> {
    get_provider_js().map_err(|_e| EthSignerError::MissingEthSigner)
}

pub struct JsonRpcSigner {
    provider: Provider,
}

impl JsonRpcSigner {
    pub fn new() -> Result<JsonRpcSigner, EthSignerError> {
        let provider = get_provider()?;
        if provider.is_none() {
            return Err(EthSignerError::MissingEthSigner);
        }
        Ok(JsonRpcSigner {
            provider: provider.unwrap(),
        })
    }

    pub async fn sign_message(&self, message: &[u8]) -> Result<PackedEthSignature, EthSignerError> {
        let provider_address = self.provider.selectedAddress();
        let mut params = Vec::new();
        let msg_str =
            std::str::from_utf8(message).map_err(|e| EthSignerError::CustomError(e.to_string()))?;
        params.push(serde_json::to_value(msg_str).unwrap());
        params.push(serde_json::to_value(provider_address).unwrap());
        let req_params = RequestArguments {
            method: "personal_sign".to_string(),
            params,
        };
        let params = serde_wasm_bindgen::to_value(&req_params)
            .map_err(|e| EthSignerError::CustomError(e.to_string()))?;
        let signature = self.provider.request(params).await.map_err(|e| {
            EthSignerError::SigningFailed(serde_wasm_bindgen::from_value::<String>(e).unwrap())
        })?;
        let signature = serde_wasm_bindgen::from_value::<String>(signature).unwrap();
        PackedEthSignature::from_hex(&signature)
    }

    pub async fn sign_message_eip712(
        &self,
        message: &[u8],
    ) -> Result<PackedEthSignature, EthSignerError> {
        let provider_address = self.provider.selectedAddress();
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
            EthSignerError::SigningFailed(serde_wasm_bindgen::from_value::<String>(e).unwrap())
        })?;
        let signature = serde_wasm_bindgen::from_value::<String>(signature).unwrap();
        PackedEthSignature::from_hex(&signature)
    }
}