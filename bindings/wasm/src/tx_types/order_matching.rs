use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::error::TypeError::InvalidBigIntStr;
use zklink_sdk_types::tx_builder::OrderMatchingBuilder as TxOrderMatchingBuilder;
use zklink_sdk_types::tx_type::contract::{
    ContractPrice as InnerContractPrice, SpotPriceInfo as InnerSpotPriceInfo,
};
use zklink_sdk_types::tx_type::order_matching::{
    Order as OrderTx, OrderMatching as OrderMatchingTx,
};

#[wasm_bindgen]
pub struct Order {
    inner: OrderTx,
}

#[wasm_bindgen]
pub struct OrderMatching {
    inner: OrderMatchingTx,
}
#[wasm_bindgen]
impl Order {
    #[wasm_bindgen(constructor)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        slot_id: u32,
        nonce: u32,
        base_token_id: u32,
        quote_token_id: u32,
        amount: String,
        price: String,
        is_sell: bool,
        maker_fee_rate: u8,
        taker_fee_rate: u8,
        has_subsidy: bool,
    ) -> Result<Order, JsValue> {
        let order = Order {
            inner: OrderTx {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                slot_id: slot_id.into(),
                nonce: nonce.into(),
                base_token_id: base_token_id.into(),
                quote_token_id: quote_token_id.into(),
                amount: BigUint::from_str(&amount).map_err(|e| InvalidBigIntStr(e.to_string()))?,
                price: BigUint::from_str(&price).map_err(|e| InvalidBigIntStr(e.to_string()))?,
                is_sell: is_sell as u8,
                fee_rates: [maker_fee_rate, taker_fee_rate],
                has_subsidy: has_subsidy as u8,
                signature: Default::default(),
            },
        };
        Ok(order)
    }

    #[wasm_bindgen(js_name=jsValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
impl OrderMatching {
    #[wasm_bindgen(js_name=jsValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct OrderMatchingBuilder {
    inner: TxOrderMatchingBuilder,
}

#[wasm_bindgen]
impl OrderMatchingBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        taker: JsValue,
        maker: JsValue,
        fee: String,
        fee_token: u32,
        contract_prices: Vec<JsValue>,
        margin_prices: Vec<JsValue>,
        expect_base_amount: String,
        expect_quote_amount: String,
    ) -> Result<OrderMatchingBuilder, JsValue> {
        let contract_prices = contract_prices
            .iter()
            .map(|p| serde_wasm_bindgen::from_value(p.clone()).unwrap())
            .collect::<Vec<InnerContractPrice>>();
        let margin_prices = margin_prices
            .iter()
            .map(|p| serde_wasm_bindgen::from_value(p.clone()).unwrap())
            .collect::<Vec<InnerSpotPriceInfo>>();
        let maker: OrderTx = serde_wasm_bindgen::from_value(maker)?;
        let taker: OrderTx = serde_wasm_bindgen::from_value(taker)?;
        let inner = TxOrderMatchingBuilder {
            account_id: account_id.into(),
            sub_account_id: sub_account_id.into(),
            taker,
            fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
            fee_token: fee_token.into(),
            expect_base_amount: BigUint::from_str(&expect_base_amount)
                .map_err(|e| InvalidBigIntStr(e.to_string()))?,
            maker,
            expect_quote_amount: BigUint::from_str(&expect_quote_amount)
                .map_err(|e| InvalidBigIntStr(e.to_string()))?,
            contract_prices,
            margin_prices,
        };
        Ok(OrderMatchingBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> OrderMatching {
        OrderMatching {
            inner: self.inner.build(),
        }
    }
}

#[wasm_bindgen(js_name=newOrderMatching)]
pub fn new_order_matching(builder: OrderMatchingBuilder) -> OrderMatching {
    builder.build()
}
