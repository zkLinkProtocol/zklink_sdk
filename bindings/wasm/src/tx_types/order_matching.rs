use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::tx_builder::OrderMatchingBuilder as TxOrderMatchingBuilder;
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
        fee_ratio1: u8,
        fee_ratio2: u8,
    ) -> Order {
        Order {
            inner: OrderTx {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                slot_id: slot_id.into(),
                nonce: nonce.into(),
                base_token_id: base_token_id.into(),
                quote_token_id: quote_token_id.into(),
                amount: BigUint::from_str(&amount).unwrap(),
                price: BigUint::from_str(&price).unwrap(),
                is_sell: is_sell as u8,
                fee_ratio1,
                fee_ratio2,
                signature: Default::default(),
            },
        }
    }

    pub fn get_inner_order(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
impl OrderMatching {
    pub fn get_inner_tx(&self) -> Result<JsValue, JsValue> {
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
        expect_base_amount: String,
        expect_quote_amount: String,
    ) -> Result<OrderMatchingBuilder, JsValue> {
        let maker: OrderTx = serde_wasm_bindgen::from_value(maker)?;
        let taker: OrderTx = serde_wasm_bindgen::from_value(taker)?;
        let inner = TxOrderMatchingBuilder {
            account_id: account_id.into(),
            sub_account_id: sub_account_id.into(),
            taker,
            fee: BigUint::from_str(&fee).unwrap(),
            fee_token: fee_token.into(),
            expect_base_amount: BigUint::from_str(&expect_base_amount).unwrap(),
            maker,
            expect_quote_amount: BigUint::from_str(&expect_quote_amount).unwrap(),
        };
        Ok(OrderMatchingBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> OrderMatching {
        OrderMatching {
            inner: OrderMatchingTx::new(self.inner),
        }
    }
}

#[wasm_bindgen(js_name=newOrderMatching)]
pub fn new_order_matching(builder: OrderMatchingBuilder) -> OrderMatching {
    builder.build()
}
