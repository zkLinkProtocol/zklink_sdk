use crate::tx_types::change_pubkey::{ChangePubKey, Create2Data};
use crate::tx_types::transfer::Transfer;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_interface::signer::Signer as InterfaceSigner;
use zklink_sdk_signers::eth_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::ZkLinkSigner;
use zklink_sdk_types::basic_types::ZkLinkAddress;
use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey as TxChangePubKey;
use zklink_sdk_types::tx_type::change_pubkey::Create2Data as ChangePubKeyCreate2Data;
use zklink_sdk_types::tx_type::transfer::Transfer as TxTransfer;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;

#[wasm_bindgen]
pub struct Signer {
    inner: InterfaceSigner,
}

#[wasm_bindgen]
impl Signer {
    #[wasm_bindgen(constructor)]
    pub fn new(private_key: &str) -> Result<Signer, JsValue> {
        let inner = InterfaceSigner::new(private_key)?;
        Ok(Signer { inner })
    }

    #[wasm_bindgen(js_name=signChangePubkeyWithEthEcdsaAuth)]
    pub fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        tx: ChangePubKey,
        l1_client_id: u32,
        main_contract: &str,
    ) -> Result<JsValue, JsValue> {
        let inner_tx = tx.get_inner_tx()?;
        let change_pubkey: TxChangePubKey = serde_wasm_bindgen::from_value(inner_tx)?;
        let contract_address = ZkLinkAddress::from_hex(main_contract)?;
        let signature = self.inner.sign_change_pubkey_with_eth_ecdsa_auth(
            change_pubkey,
            l1_client_id,
            contract_address,
        )?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signChangePubkeyWithCreate2DataAuth)]
    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        tx: ChangePubKey,
        create2_data: Create2Data,
    ) -> Result<JsValue, JsValue> {
        let inner_tx = tx.get_inner_tx()?;
        let change_pubkey: TxChangePubKey = serde_wasm_bindgen::from_value(inner_tx)?;
        let inner_data = create2_data.get_inner_data()?;
        let create2_data: ChangePubKeyCreate2Data = serde_wasm_bindgen::from_value(inner_data)?;
        let signature = self
            .inner
            .sign_change_pubkey_with_create2data_auth(change_pubkey, create2_data)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signTransfer)]
    pub fn sign_transfer(&self, tx: Transfer, token_symbol: &str) -> Result<JsValue, JsValue> {
        let inner_tx = tx.get_inner_tx()?;
        let transfer: TxTransfer = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_transfer(transfer, token_symbol)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=submitterSignature)]
    pub fn submitter_signature(&self, tx: JsValue) -> Result<String, JsValue> {
        let zklink_tx: ZkLinkTx = serde_wasm_bindgen::from_value(tx)?;
        let zklink_signature = self.inner.submitter_signature(&zklink_tx)?;
        Ok(zklink_signature.as_hex())
    }
}
