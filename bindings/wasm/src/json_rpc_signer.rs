use crate::rpc_type_converter::TxZkLinkSignature;
use crate::tx_types::change_pubkey::{ChangePubKey, Create2Data};
use crate::tx_types::forced_exit::ForcedExit;
use crate::tx_types::order_matching::{Order, OrderMatching};
use crate::tx_types::transfer::Transfer;
use crate::tx_types::withdraw::Withdraw;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_interface::json_rpc_signer::JsonRpcSigner as InterfaceJsonRpcSigner;
use zklink_sdk_signers::eth_signer::json_rpc_signer::Provider;
use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey as TxChangePubKey;
use zklink_sdk_types::tx_type::change_pubkey::Create2Data as ChangePubKeyCreate2Data;
use zklink_sdk_types::tx_type::forced_exit::ForcedExit as TxForcedExit;
use zklink_sdk_types::tx_type::order_matching::{
    Order as TxOrder, OrderMatching as TxOrderMatching,
};
use zklink_sdk_types::tx_type::transfer::Transfer as TxTransfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw as TxWithdraw;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
use zklink_sdk_signers::starknet_signer::starknet_json_rpc_signer::Signer;
use zklink_sdk_interface::json_rpc_signer::JsonRpcProvider;

#[wasm_bindgen]
pub struct JsonRpcSigner {
    inner: InterfaceJsonRpcSigner,
}

//#[wasm_bindgen(constructor)]
#[wasm_bindgen(js_name=newRpcSignerWtihProvider)]
pub fn new_with_provider(provider: Provider) -> Result<JsonRpcSigner, JsValue> {
    let inner = InterfaceJsonRpcSigner::new(JsonRpcProvider::Provider(provider),None,None)?;
    Ok(JsonRpcSigner { inner })
}

//#[wasm_bindgen(constructor)]
#[wasm_bindgen(js_name=newRpcSignerWithSigner)]
pub fn new_with_signer(signer: Signer, pub_key: String,chain_id: String) -> Result<JsonRpcSigner, JsValue> {
    let inner = InterfaceJsonRpcSigner::new(JsonRpcProvider::Signer(signer),Some(pub_key),Some(chain_id))?;
    Ok(JsonRpcSigner { inner })
}

#[wasm_bindgen]
impl JsonRpcSigner {
    #[wasm_bindgen(js_name = initZklinkSigner)]
    pub async fn init_zklink_signer(&mut self, signature: Option<String>) -> Result<(), JsValue> {
        Ok(self.inner.init_zklink_signer(signature).await?)
    }

    #[wasm_bindgen(js_name = signTransfer)]
    pub async fn sign_transfer(
        &self,
        tx: Transfer,
        token_symbol: &str,
    ) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let transfer: TxTransfer = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_transfer(transfer, token_symbol).await?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signChangePubkeyWithEthEcdsaAuth)]
    pub async fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        tx: ChangePubKey,
    ) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let change_pubkey: TxChangePubKey = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self
            .inner
            .sign_change_pubkey_with_eth_ecdsa_auth(change_pubkey)
            .await?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signChangePubkeyWithCreate2DataAuth)]
    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        tx: ChangePubKey,
        create2_data: Create2Data,
    ) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let change_pubkey: TxChangePubKey = serde_wasm_bindgen::from_value(inner_tx)?;
        let inner_data = create2_data.json_value()?;
        let create2_data: ChangePubKeyCreate2Data = serde_wasm_bindgen::from_value(inner_data)?;
        let signature = self
            .inner
            .sign_change_pubkey_with_create2data_auth(change_pubkey, create2_data)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=createSignedOrder)]
    pub fn create_signed_order(&self, order: Order) -> Result<JsValue, JsValue> {
        let inner_order = order.json_value()?;
        let mut order: TxOrder = serde_wasm_bindgen::from_value(inner_order)?;
        let signed_order = self.inner.create_signed_order(&mut order)?;
        Ok(serde_wasm_bindgen::to_value(&signed_order)?)
    }

    #[wasm_bindgen(js_name=signOrderMatching)]
    pub fn sign_order_matching(&self, tx: OrderMatching) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let order_matching: TxOrderMatching = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_order_matching(order_matching)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signWithdraw)]
    pub async fn sign_withdraw(
        &self,
        tx: Withdraw,
        token_symbol: &str,
    ) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let withdraw: TxWithdraw = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_withdraw(withdraw, token_symbol).await?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signForcedExit)]
    pub fn sign_forced_exit(&self, tx: ForcedExit) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let forced_exit: TxForcedExit = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_forced_exit(forced_exit)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=submitterSignature)]
    pub fn submitter_signature(&self, tx: JsValue) -> Result<TxZkLinkSignature, JsValue> {
        let zklink_tx: ZkLinkTx = serde_wasm_bindgen::from_value(tx)?;
        let zklink_signature = self.inner.submitter_signature(&zklink_tx)?;
        Ok(zklink_signature.into())
    }
}
