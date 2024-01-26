use crate::eth_signer::{EthSignerError, PackedEthSignature};
use crate::RpcErr;
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
    pub type Signer;

    #[wasm_bindgen(structural,catch, method)]
    async fn signMessage(_: &Signer, msg: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method,getter)]
    fn getAddress(this: &Signer) -> Option<String>;
}

pub struct JsonRpcSigner {
    signer: Signer,
}

impl JsonRpcSigner {
    pub fn new(signer: Signer) -> JsonRpcSigner {
        JsonRpcSigner { signer }
    }

    pub fn address(&self) -> Option<String> {
        self.signer.getAddress()
    }

    pub async fn sign_message(&self, message: &[u8]) -> Result<PackedEthSignature, EthSignerError> {
        let msg_str =
            std::str::from_utf8(message).map_err(|e| EthSignerError::CustomError(e.to_string()))?;
        let signature = self
            .signer
            .signMessage(JsValue::from_str(msg_str))
            .await
            .map_err(|e| {
                EthSignerError::RpcSignError(serde_wasm_bindgen::from_value::<RpcErr>(e).unwrap())
            })?;
        let signature = serde_wasm_bindgen::from_value::<String>(signature)
            .map_err(|e| EthSignerError::SigningFailed(e.to_string()))?;
        PackedEthSignature::from_hex(&signature)
    }
}
