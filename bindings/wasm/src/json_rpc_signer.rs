use crate::tx_types::change_pubkey::{ChangePubKey, Create2Data};
use crate::tx_types::forced_exit::ForcedExit;
use crate::tx_types::order_matching::{Order, OrderMatching};
use crate::tx_types::transfer::Transfer;
use crate::tx_types::withdraw::Withdraw;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_interface::json_rpc_signer::JsonRpcSigner as InterfaceJsonRpcSigner;
use zklink_sdk_types::basic_types::ZkLinkAddress;
use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey as TxChangePubKey;
use zklink_sdk_types::tx_type::change_pubkey::Create2Data as ChangePubKeyCreate2Data;
use zklink_sdk_types::tx_type::forced_exit::ForcedExit as TxForcedExit;
use zklink_sdk_types::tx_type::order_matching::{
    Order as TxOrder, OrderMatching as TxOrderMatching,
};
use zklink_sdk_types::tx_type::transfer::Transfer as TxTransfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw as TxWithdraw;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;

#[wasm_bindgen]
pub struct JsonRpcSigner {
    inner: InterfaceJsonRpcSigner,
}

#[wasm_bindgen]
impl JsonRpcSigner {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsonRpcSigner, JsValue> {
        let inner = InterfaceJsonRpcSigner::new()?;
        Ok(JsonRpcSigner { inner })
    }

    #[wasm_bindgen(js_name = initZklinkSigner)]
    pub async fn init_zklink_signer(&mut self) -> Result<(), JsValue> {
        Ok(self.inner.init_zklink_signer().await?)
    }

    #[wasm_bindgen(js_name = signTransfer)]
    pub async fn sign_transfer(
        &self,
        tx: Transfer,
        token_symbol: &str,
    ) -> Result<JsValue, JsValue> {
        let inner_tx = tx.get_inner_tx()?;
        let transfer: TxTransfer = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_transfer(transfer, token_symbol).await?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signChangePubkeyWithEthEcdsaAuth)]
    pub async fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        tx: ChangePubKey,
        l1_client_id: u32,
        main_contract: &str,
    ) -> Result<JsValue, JsValue> {
        let inner_tx = tx.get_inner_tx()?;
        let change_pubkey: TxChangePubKey = serde_wasm_bindgen::from_value(inner_tx)?;
        let contract_address = ZkLinkAddress::from_hex(main_contract)?;
        let signature = self
            .inner
            .sign_change_pubkey_with_eth_ecdsa_auth(change_pubkey, l1_client_id, contract_address)
            .await?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signChangePubkeyWithCreate2DataAuth)]
    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        tx: ChangePubKey,
        create2_data: Create2Data,
        from_account: String,
    ) -> Result<JsValue, JsValue> {
        let inner_tx = tx.get_inner_tx()?;
        let change_pubkey: TxChangePubKey = serde_wasm_bindgen::from_value(inner_tx)?;
        let inner_data = create2_data.get_inner_data()?;
        let create2_data: ChangePubKeyCreate2Data = serde_wasm_bindgen::from_value(inner_data)?;
        let signature = self.inner.sign_change_pubkey_with_create2data_auth(
            change_pubkey,
            create2_data,
            ZkLinkAddress::from_hex(&from_account)?,
        )?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=createSignedOrder)]
    pub fn create_signed_order(&self, order: Order) -> Result<JsValue, JsValue> {
        let inner_order = order.get_inner_order()?;
        let mut order: TxOrder = serde_wasm_bindgen::from_value(inner_order)?;
        let signed_order = self.inner.sign_order(&mut order)?;
        Ok(serde_wasm_bindgen::to_value(&signed_order)?)
    }

    #[wasm_bindgen(js_name=signOrderMatching)]
    pub fn sign_order_matching(&self, tx: OrderMatching) -> Result<JsValue, JsValue> {
        let inner_tx = tx.get_inner_tx()?;
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
        let inner_tx = tx.get_inner_tx()?;
        let withdraw: TxWithdraw = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_withdraw(withdraw, token_symbol).await?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signForcedExit)]
    pub fn sign_forced_exit(&self, tx: ForcedExit) -> Result<JsValue, JsValue> {
        let inner_tx = tx.get_inner_tx()?;
        let forced_exit: TxForcedExit = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_forced_exit(forced_exit)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=submitterSignature)]
    pub fn submitter_signature(&self, tx: JsValue) -> Result<String, JsValue> {
        let zklink_tx: ZkLinkTx = serde_wasm_bindgen::from_value(tx)?;
        let zklink_signature = self.inner.submitter_signature(&zklink_tx)?;
        Ok(zklink_signature.as_hex())
    }
}
