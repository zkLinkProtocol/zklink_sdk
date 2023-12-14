use wasm_bindgen::prelude::*;
use crate::starknet_signer::StarkSignature;
use crate::starknet_signer::error::StarkSignerError;
use crate::starknet_signer::typed_data::{message::TypedDataMessage, TypedData};

#[wasm_bindgen]
// Rustfmt removes the 'async' keyword from async functions in extern blocks. It's fixed
// in rustfmt 2.
#[rustfmt::skip]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Signer;

    #[wasm_bindgen(structural,catch, method)]
    async fn signMessage(_: &Signer,msg: &JsValue) -> Result<JsValue, JsValue>;
}

pub struct StarknetJsonRpcSigner {
    signer: Signer,
}

impl StarknetJsonRpcSigner {
    pub fn new(signer: Signer) -> StarknetJsonRpcSigner{
        StarknetJsonRpcSigner { signer }
    }

    pub async fn sign_message(
        &self,
        message: TypedDataMessage,
    ) -> Result<StarkSignature, StarkSignerError> {
        let typed_data = TypedData::new(message);
        let typed_data = serde_wasm_bindgen::to_value(&typed_data)
            .map_err(|e| StarkSignerError::SignError(e.to_string()))?;
        let signature = self.signer.signMessage(&typed_data).await.map_err(|e| {
            StarkSignerError::SignError(
                serde_wasm_bindgen::from_value::<String>(e).unwrap_or_default(),
            )
        })?;
        let signature: Vec<String> = serde_wasm_bindgen::from_value::<Vec<String>>(signature)
            .map_err(|e| StarkSignerError::InvalidSignature(e.to_string()))?;
        StarkSignature::from_str(&signature[0],&signature[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_typed_data() {
        let typed_data = TypedData::new("123".to_string());
        let typed_data = serde_json::to_string(&typed_data)
            .unwrap();
        println!("{:?}",typed_data);
    }
}