use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::contract::{
    AutoDeleveraging as AutoDeleveragingTx, ContractPrice as InnerContractPrice,
    SpotPriceInfo as InnerSpotPriceInfo,
};
use zklink_sdk_types::tx_builder::AutoDeleveragingBuilder as TxAutoDeleveragingBuilder;

#[wasm_bindgen]
pub struct AutoDeleveraging {
    inner: AutoDeleveragingTx,
}

#[wasm_bindgen]
impl AutoDeleveraging {
    #[wasm_bindgen(js_name=jsValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct AutoDeleveragingBuilder {
    inner: TxAutoDeleveragingBuilder,
}

#[wasm_bindgen]
impl AutoDeleveragingBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        sub_account_nonce: u32,
        contract_prices: Vec<JsValue>,
        margin_prices: Vec<JsValue>,
        adl_account_id: u32,
        pair_id: u16,
        adl_size: String,
        adl_price: String,
        fee: String,
        fee_token: u16,
    ) -> Result<AutoDeleveragingBuilder, JsValue> {
        let contract_prices = contract_prices
            .iter()
            .map(|p| serde_wasm_bindgen::from_value(p.clone()).unwrap())
            .collect::<Vec<InnerContractPrice>>();
        let margin_prices = margin_prices
            .iter()
            .map(|p| serde_wasm_bindgen::from_value(p.clone()).unwrap())
            .collect::<Vec<InnerSpotPriceInfo>>();
        let inner = TxAutoDeleveragingBuilder {
            account_id: account_id.into(),
            sub_account_id: sub_account_id.into(),
            sub_account_nonce: sub_account_nonce.into(),
            contract_prices,
            margin_prices,
            adl_account_id: adl_account_id.into(),
            pair_id: pair_id.into(),
            adl_size: BigUint::from_str(&adl_size).unwrap(),
            adl_price: BigUint::from_str(&adl_price).unwrap(),
            fee: BigUint::from_str(&fee).unwrap(),
            fee_token: fee_token.into(),
        };
        Ok(AutoDeleveragingBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> AutoDeleveraging {
        AutoDeleveraging {
            inner: self.inner.build(),
        }
    }
}

#[wasm_bindgen(js_name=newAutoDeleveraging)]
pub fn new_auto_deleveraging(builder: AutoDeleveragingBuilder) -> AutoDeleveraging {
    builder.build()
}
