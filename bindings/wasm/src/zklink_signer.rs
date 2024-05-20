use crate::rpc_type_converter::TxZkLinkSignature;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_signers::zklink_signer::ZkLinkSigner as InnerZkLinkSigner;

#[wasm_bindgen]
pub struct ZkLinkSigner {
    inner: InnerZkLinkSigner,
}

impl ZkLinkSigner {
    pub fn new(signer: InnerZkLinkSigner) -> Self {
        Self { inner: signer }
    }

    pub fn sign_musig(&self, msg: &Vec<u8>) -> Result<TxZkLinkSignature, JsValue> {
        Ok(self.inner.sign_musig(msg)?.into())
    }
}
