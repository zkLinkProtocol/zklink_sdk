use crate::starknet_signer::error::StarkSignerError;
use crate::starknet_signer::typed_data::message::TypedDataMessage;
use crate::starknet_signer::typed_data::TypedData;
use crate::starknet_signer::{StarkEcdsaSignature, StarkEip712Signature};
use crate::RpcErr;
use starknet_core::types::FieldElement;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// Rustfmt removes the 'async' keyword from async functions in extern blocks. It's fixed
// in rustfmt 2.
#[rustfmt::skip]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Signer;

    #[wasm_bindgen(structural,catch, method)]
    async fn signMessage(_: &Signer,msg: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, getter)]
    fn address(this: &Signer) -> String;
}

pub struct StarknetJsonRpcSigner {
    signer: Signer,
    pub_key: String,
    chain_id: String,
}

impl StarknetJsonRpcSigner {
    pub fn new(signer: Signer, pub_key: String, chain_id: String) -> StarknetJsonRpcSigner {
        StarknetJsonRpcSigner {
            signer,
            pub_key,
            chain_id,
        }
    }

    pub fn address(&self) -> String {
        self.signer.address()
    }

    pub async fn sign_message(
        &self,
        message: TypedDataMessage,
    ) -> Result<StarkEip712Signature, StarkSignerError> {
        let typed_data = TypedData::new(message, self.chain_id.clone());
        let typed_data = serde_wasm_bindgen::to_value(&typed_data)
            .map_err(|e| StarkSignerError::SignError(e.to_string()))?;
        let signature = self.signer.signMessage(&typed_data).await.map_err(|e| {
            let err_str = format!("{:?}", e);
            let e = err_str.trim_start_matches("JsValue(").trim_end_matches(')');
            StarkSignerError::RpcSignError(RpcErr {
                code: 1,
                message: e.to_string(),
            })
        })?;
        let signature: Vec<String> = serde_wasm_bindgen::from_value::<Vec<String>>(signature)
            .map_err(|e| StarkSignerError::InvalidSignature(e.to_string()))?;

        let signature = StarkEcdsaSignature::from_rs_str(&signature[0], &signature[1])
            .map_err(|e| StarkSignerError::InvalidSignature(e.to_string()))?;
        let pub_key = FieldElement::from_str(&self.pub_key)
            .map_err(|e| StarkSignerError::SignError(e.to_string()))?;
        Ok(StarkEip712Signature { pub_key, signature })
    }
}
