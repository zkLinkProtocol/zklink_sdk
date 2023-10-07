use wasm_bindgen::prelude::*;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner as Signer;
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature as Signature;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::zklink_signer::ZkLinkSigner;

#[wasm_bindgen]
pub struct EthPrivateKeySigner {
    inner: EthSigner,
}

#[wasm_bindgen]
impl EthPrivateKeySigner {
    #[wasm_bindgen(js_name=newFromHexPrivateKey)]
    pub fn new_from_hex_pk(private_key: &str) -> Result<EthPrivateKeySigner, JsValue> {
        let signer = EthSigner::try_from(private_key)?;
        Ok(Self { inner: signer })
    }

    #[wasm_bindgen(js_name=getAddress)]
    pub fn get_address(&self) -> Result<String, JsValue> {
        let address = self.inner.get_address()?;
        Ok(format!("{:?}", address))
    }
    #[wasm_bindgen(js_name=signMessage)]
    pub fn sign_message(&self, msg: &[u8]) -> Result<String, JsValue> {
        let signature = self.inner.sign_message(msg)?;
        Ok(signature.as_hex())
    }
}

#[wasm_bindgen]
pub struct EthSignature {
    inner: PackedEthSignature,
}

#[wasm_bindgen]
pub struct ZklinkSigner {
    inner: Signer,
}

#[wasm_bindgen]
impl ZklinkSigner {
    #[wasm_bindgen(js_name=newRand)]
    pub fn new_rand() -> Result<ZklinkSigner, JsValue> {
        let zklink_signer = Signer::new()?;
        Ok(ZklinkSigner {
            inner: zklink_signer,
        })
    }

    #[wasm_bindgen(js_name=newFromEthSigner)]
    pub fn new_from_hex_eth_signer(eth_hex_private_key: &str) -> Result<ZklinkSigner, JsValue> {
        let zklink_signer = Signer::new_from_hex_eth_signer(eth_hex_private_key)?;
        Ok(ZklinkSigner {
            inner: zklink_signer,
        })
    }

    #[wasm_bindgen]
    pub fn sign(&self, msg: &[u8]) -> Result<String, JsValue> {
        let signature = self.inner.sign_musig(msg)?;
        Ok(signature.as_hex())
    }
}

#[wasm_bindgen]
pub struct ZklinkSignature {
    inner: Signature,
}

#[wasm_bindgen]
impl ZklinkSignature {
    #[wasm_bindgen(js_name=newFromHexStr)]
    pub fn new_from_hex_str(signature_str: &str) -> Result<ZklinkSignature, JsValue> {
        let signature = Signature::from_hex(signature_str)?;
        Ok(ZklinkSignature { inner: signature })
    }
    #[wasm_bindgen]
    pub fn verify(&self, msg: &[u8]) -> Result<bool, JsValue> {
        Ok(self.inner.verify_musig(msg)?)
    }
}
