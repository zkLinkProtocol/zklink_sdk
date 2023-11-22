use crate::tx_types::change_pubkey::{ChangePubKey, Create2Data};
use crate::tx_types::contract::auto_deleveraging::AutoDeleveraging;
use crate::tx_types::contract::contract_matching::ContractMatching;
use crate::tx_types::contract::funding::Funding;
use crate::tx_types::contract::liquidation::Liquidation;
use crate::tx_types::forced_exit::ForcedExit;
use crate::tx_types::order_matching::{Order, OrderMatching};
use crate::tx_types::transfer::Transfer;
use crate::tx_types::withdraw::Withdraw;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_interface::signer::Signer as InterfaceSigner;
use zklink_sdk_types::basic_types::ZkLinkAddress;
use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey as TxChangePubKey;
use zklink_sdk_types::tx_type::change_pubkey::Create2Data as ChangePubKeyCreate2Data;
use zklink_sdk_types::tx_type::contract::{
    AutoDeleveraging as TxAutoDeleveraging, ContractMatching as TxContractMatching,
    Funding as TxFunding, Liquidation as TxLiquidation,
};
use zklink_sdk_types::tx_type::forced_exit::ForcedExit as TxForcedExit;
use zklink_sdk_types::tx_type::order_matching::{
    Order as TxOrder, OrderMatching as TxOrderMatching,
};
use zklink_sdk_types::tx_type::transfer::Transfer as TxTransfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw as TxWithdraw;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
use crate::rpc_type_converter::TxZkLinkSignature;

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

    #[wasm_bindgen(js_name=getPubkeyHash)]
    pub fn get_pubkey_hash(&self) -> String {
        self.inner.pubkey_hash().as_hex()
    }

    #[wasm_bindgen(js_name=signChangePubkeyWithEthEcdsaAuth)]
    pub fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        tx: ChangePubKey,
        l1_client_id: u32,
        main_contract: &str,
    ) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
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
        let inner_tx = tx.json_value()?;
        let change_pubkey: TxChangePubKey = serde_wasm_bindgen::from_value(inner_tx)?;
        let inner_data = create2_data.json_value()?;
        let create2_data: ChangePubKeyCreate2Data = serde_wasm_bindgen::from_value(inner_data)?;
        let signature = self
            .inner
            .sign_change_pubkey_with_create2data_auth(change_pubkey, create2_data)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signTransfer)]
    pub fn sign_transfer(&self, tx: Transfer, token_symbol: &str) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let transfer: TxTransfer = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_transfer(transfer, token_symbol)?;
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
    pub fn sign_withdraw(&self, tx: Withdraw, token_symbol: &str) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let withdraw: TxWithdraw = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_withdraw(withdraw, token_symbol)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signForcedExit)]
    pub fn sign_forced_exit(&self, tx: ForcedExit) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let forced_exit: TxForcedExit = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_forced_exit(forced_exit)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signAutoDeleveraging)]
    pub fn sign_auto_deleveraging(&self, tx: AutoDeleveraging) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let auto_deleveraging: TxAutoDeleveraging = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_auto_deleveraging(auto_deleveraging)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signContractMatching)]
    pub fn sign_contract_matching(&self, tx: ContractMatching) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let contract_matching: TxContractMatching = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_contract_matching(contract_matching)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signFunding)]
    pub fn sign_funding(&self, tx: Funding) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let funding: TxFunding = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_funding(funding)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=signLiquidation)]
    pub fn sign_liquidation(&self, tx: Liquidation) -> Result<JsValue, JsValue> {
        let inner_tx = tx.json_value()?;
        let liquidation: TxLiquidation = serde_wasm_bindgen::from_value(inner_tx)?;
        let signature = self.inner.sign_liquidation(liquidation)?;
        Ok(serde_wasm_bindgen::to_value(&signature)?)
    }

    #[wasm_bindgen(js_name=submitterSignature)]
    pub fn submitter_signature(&self, tx: JsValue) -> Result<TxZkLinkSignature, JsValue> {
        let zklink_tx: ZkLinkTx = serde_wasm_bindgen::from_value(tx)?;
        let zklink_signature = self.inner.submitter_signature(&zklink_tx)?;
        Ok(zklink_signature.into())
    }
}
