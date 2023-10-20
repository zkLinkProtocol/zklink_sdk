use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::{BigUint, ZkLinkAddress};
use zklink_sdk_types::tx_builder::WithdrawBuilder as TxWithdrawBuilder;
use zklink_sdk_types::tx_type::withdraw::Withdraw as WithdrawTx;

#[wasm_bindgen]
pub struct Withdraw {
    inner: WithdrawTx,
}

#[wasm_bindgen]
impl Withdraw {
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct WithdrawBuilder {
    inner: TxWithdrawBuilder,
}

#[wasm_bindgen]
impl WithdrawBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        to_chain_id: u8,
        to_address: String,
        l2_source_token: u32,
        fee: String,
        fast_withdraw: bool,
        withdraw_fee_ratio: u16,
        l1_target_token: u32,
        amount: String,
        nonce: u32,
        withdraw_to_l1: bool,
        ts: Option<u32>,
    ) -> Result<WithdrawBuilder, JsValue> {
        let ts = if let Some(time_stamp) = ts {
            time_stamp
        } else {
            std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as u32
        };
        let inner = TxWithdrawBuilder {
            account_id: account_id.into(),
            sub_account_id: sub_account_id.into(),
            to_chain_id: to_chain_id.into(),
            to_address: ZkLinkAddress::from_hex(&to_address)?,
            l2_source_token: l2_source_token.into(),
            fee: BigUint::from_str(&fee).unwrap(),
            nonce: nonce.into(),
            fast_withdraw,
            withdraw_fee_ratio,
            timestamp: ts.into(),
            amount: BigUint::from_str(&amount).unwrap(),
            l1_target_token: l1_target_token.into(),
            withdraw_to_l1,
        };
        Ok(WithdrawBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> Withdraw {
        Withdraw {
            inner: self.inner.build(),
        }
    }
}

#[wasm_bindgen(js_name=newWithdraw)]
pub fn new_withdraw(builder: WithdrawBuilder) -> Withdraw {
    builder.build()
}
