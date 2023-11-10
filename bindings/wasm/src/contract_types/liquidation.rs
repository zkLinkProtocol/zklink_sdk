use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::contract::Liquidation as LiquidationTx;
use zklink_sdk_types::contract::{
    ContractPrice as InnerContractPrice, SpotPriceInfo as InnerSpotPriceInfo,
};
use zklink_sdk_types::tx_builder::LiquidationBuilder as TxLiquidationBuilder;

#[wasm_bindgen]
pub struct Liquidation {
    inner: LiquidationTx,
}
#[wasm_bindgen]
impl Liquidation {
    #[wasm_bindgen(js_name=jsValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct LiquidationBuilder {
    inner: TxLiquidationBuilder,
}

#[wasm_bindgen]
impl LiquidationBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        sub_account_nonce: u32,
        contract_prices: Vec<JsValue>,
        margin_prices: Vec<JsValue>,
        liquidation_account_id: u32,
        fee: String,
        fee_token: u16,
    ) -> Result<LiquidationBuilder, JsValue> {
        let contract_prices = contract_prices
            .iter()
            .map(|p| serde_wasm_bindgen::from_value(p.clone()).unwrap())
            .collect::<Vec<InnerContractPrice>>();
        let margin_prices = margin_prices
            .iter()
            .map(|p| serde_wasm_bindgen::from_value(p.clone()).unwrap())
            .collect::<Vec<InnerSpotPriceInfo>>();
        let inner = TxLiquidationBuilder {
            account_id: account_id.into(),
            sub_account_id: sub_account_id.into(),
            sub_account_nonce: sub_account_nonce.into(),
            contract_prices,
            margin_prices,
            liquidation_account_id: liquidation_account_id.into(),
            fee: BigUint::from_str(&fee).unwrap(),
            fee_token: fee_token.into(),
        };
        Ok(LiquidationBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> Liquidation {
        Liquidation {
            inner: self.inner.build(),
        }
    }
}

#[wasm_bindgen(js_name=newLiquidation)]
pub fn new_liquidation(builder: LiquidationBuilder) -> Liquidation {
    builder.build()
}
