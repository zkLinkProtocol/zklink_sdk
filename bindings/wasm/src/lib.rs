use wasm_bindgen::prelude::*;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_signers::eth_signer::json_rpc_signer::JsonRpcSigner;
use zklink_signers::eth_signer::eth_signature::TxEthSignature;
use wasm_bindgen_futures::{JsFuture, future_to_promise};

#[wasm_bindgen]
pub struct ZklinkSignerWasm {
    signer: ZkLinkSigner,
}

#[wasm_bindgen]
impl ZklinkSignerWasm {
    #[wasm_bindgen(js_name=NewFromEthSigner)]
    pub fn new_from_hex_eth_signer(eth_hex_private_key: &str) -> Result<ZklinkSignerWasm,JsValue> {
        let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(eth_hex_private_key)?;
        Ok(ZklinkSignerWasm { signer: zklink_signer})
    }

    #[wasm_bindgen]
    pub fn sign(&self,msg: &[u8]) -> Result<String,JsValue>{
        let signature = self.signer.sign_musig(msg)?;
        Ok(signature.as_hex())
    }
}

// #[wasm_bindgen]
// pub struct EthJsonRpcSignerWasm {
//     signer: JsonRpcSigner,
// }
//
// #[wasm_bindgen]
// impl EthJsonRpcSignerWasm {
//     #[wasm_bindgen(constructor)]
//     pub fn new(rpc_addr: String) -> Self {
//         //todo: address_or_index,signer_type,password_to_unlock set to None first
//         let signer = JsonRpcSigner::new(rpc_addr,None,None,None);
//         Self {
//             signer
//         }
//     }
//
//     #[wasm_bindgen]
//     pub async fn sign_message(&self,msg: &[u8]) -> js_sys::Promise {
//         let future = self.signer.sign_message(msg);
//         future_to_promise(future)
//     }
// }