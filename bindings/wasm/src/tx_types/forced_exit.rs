use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::{BigUint, ZkLinkAddress};
use zklink_sdk_types::tx_builder::ForcedExitBuilder as TxForcedExitBuilder;
use zklink_sdk_types::tx_type::forced_exit::ForcedExit as ForcedExitTx;

#[wasm_bindgen]
pub struct ForcedExit {
    inner: ForcedExitTx,
}

#[wasm_bindgen]
impl ForcedExit {
    pub fn get_inner_tx(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct ForcedExitBuilder {
    inner: TxForcedExitBuilder,
}

#[wasm_bindgen]
impl ForcedExitBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        to_chain_id: u8,
        initiator_account_id: u32,
        initiator_sub_account_id: u8,
        target_sub_account_id: u8,
        target: String,
        l2_source_token: u32,
        l1_target_token: u32,
        exit_amount: String,
        initiator_nonce: u32,
        ts: u32,
    ) -> Result<ForcedExitBuilder, JsValue> {
        let inner = TxForcedExitBuilder {
            to_chain_id: to_chain_id.into(),
            initiator_account_id: initiator_account_id.into(),
            initiator_sub_account_id: initiator_sub_account_id.into(),
            target: ZkLinkAddress::from_hex(&target)?,
            l2_source_token: l2_source_token.into(),
            timestamp: ts.into(),
            l1_target_token: l1_target_token.into(),
            initiator_nonce: initiator_nonce.into(),
            target_sub_account_id: target_sub_account_id.into(),
            exit_amount: BigUint::from_str(&exit_amount).unwrap(),
        };
        Ok(ForcedExitBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> ForcedExit {
        ForcedExit {
            inner: ForcedExitTx::new(self.inner),
        }
    }
}

#[wasm_bindgen(js_name=newForcedExit)]
pub fn new_forced_exit(builder: ForcedExitBuilder) -> ForcedExit {
    builder.build()
}