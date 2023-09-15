use wasm_bindgen::prelude::*;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_signers::eth_signer::eth_signature::TxEthSignature;
use wasm_bindgen_futures::{JsFuture, future_to_promise};
use zklink_signers::eth_signer::pk_signer::PrivateKeySigner;

#[wasm_bindgen]
pub struct EthPrivateKeySignerWasm {
    signer: PrivateKeySigner,
}

#[wasm_bindgen]
impl EthPrivateKeySignerWasm {
    #[wasm_bindgen]
    pub fn new_from_hex_pk(private_key: &str) -> Result<EthPrivateKeySignerWasm,JsValue> {
        let signer = PrivateKeySigner::try_from(private_key)?;
        Ok(Self {signer})
    }

    #[wasm_bindgen]
    pub fn get_address(&self) -> Result<String,JsValue> {
        let address = self.signer.get_address()?;
        Ok(format!("{:?}",address))

    }
    #[wasm_bindgen]
    pub fn sign_message(&self,msg: &[u8]) ->Result<String,JsValue> {
        let signature = self.signer.sign_message(msg)?;
        Ok(signature.as_hex())
    }
}

#[wasm_bindgen]
pub struct ZklinkSignerWasm {
    signer: ZkLinkSigner,
}

#[wasm_bindgen]
impl ZklinkSignerWasm {
    #[wasm_bindgen(js_name=NewRand)]
    pub fn new_rand() -> Result<ZklinkSignerWasm,JsValue> {
        let zklink_signer = ZkLinkSigner::new()?;
        Ok(ZklinkSignerWasm { signer: zklink_signer})
    }

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
