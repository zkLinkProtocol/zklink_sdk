use crate::rpc_type_converter::TxZkLinkSignature;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_signers::eth_signer::PackedEthSignature;
use zklink_sdk_signers::starknet_signer::StarkEcdsaSignature;
use zklink_sdk_signers::zklink_signer::ZkLinkSigner as InnerZkLinkSigner;

#[wasm_bindgen]
pub struct ZkLinkSigner {
    inner: InnerZkLinkSigner,
}

#[wasm_bindgen]
impl ZkLinkSigner {
    pub(crate) fn new(signer: InnerZkLinkSigner) -> ZkLinkSigner {
        Self { inner: signer }
    }

    pub(crate) fn sign_musig(self, msg: Vec<u8>) -> Result<TxZkLinkSignature, JsValue> {
        Ok(self.inner.sign_musig(&msg)?.into())
    }

    #[wasm_bindgen(js_name=ethSig)]
    pub fn eth_sig(sig: String) -> Result<ZkLinkSigner, JsValue> {
        let signature = PackedEthSignature::from_hex(&sig)?;
        let seed = signature.serialize_packed();
        Ok(ZkLinkSigner {
            inner: InnerZkLinkSigner::new_from_seed(&seed)?,
        })
    }

    #[wasm_bindgen(js_name=starknetSig)]
    pub fn starknet_sig(sig: String) -> Result<ZkLinkSigner, JsValue> {
        let signature = StarkEcdsaSignature::from_hex(&sig)?;
        let seed = signature.to_bytes_be();
        Ok(ZkLinkSigner {
            inner: InnerZkLinkSigner::new_from_seed(&seed)?,
        })
    }
}
