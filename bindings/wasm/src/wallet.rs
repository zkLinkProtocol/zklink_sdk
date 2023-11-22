use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::prelude::ZkLinkAddress;
use zklink_sdk_wallet::eth::EthTxOption as InnerEthTxOption;
use zklink_sdk_wallet::wallet::Wallet as InnerWallet;

#[wasm_bindgen]
pub struct EthTxOption {
    inner: InnerEthTxOption,
}

#[wasm_bindgen]
impl EthTxOption {
    #[wasm_bindgen(constructor)]
    pub fn new(
        is_support_eip1559: bool,
        to: String,
        nonce: Option<f64>,
        value: Option<String>,
        gas: Option<f64>,
        gas_price: Option<String>,
    ) -> Result<EthTxOption, JsValue> {
        let value = if let Some(v) = value {
            Some(
                BigUint::from_str(&v)
                    .map_err(|error| JsValue::from_str(&format!("error: {error}")))?,
            )
        } else {
            None
        };

        let gas_price = if let Some(g) = gas_price {
            Some(
                BigUint::from_str(&g)
                    .map_err(|error| JsValue::from_str(&format!("error: {error}")))?,
            )
        } else {
            None
        };
        let inner = InnerEthTxOption {
            is_support_eip1559,
            to: ZkLinkAddress::from_str(&to)
                .map_err(|error| JsValue::from_str(&format!("error: {error}")))?,
            nonce: nonce.map(|n| n as u64),
            value,
            gas: gas.map(|g| g as u64),
            gas_price,
        };
        Ok(EthTxOption { inner })
    }

    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct Wallet {
    inner: InnerWallet,
}

#[wasm_bindgen]
impl Wallet {
    #[wasm_bindgen(constructor)]
    pub fn new(url: &str, private_key: &str) -> Result<Wallet, JsValue> {
        let inner = InnerWallet::new(url, private_key);
        Ok(Wallet { inner })
    }

    #[wasm_bindgen(js_name=approveERC20)]
    pub async fn approve_erc20(
        &self,
        contract: String,
        amount: String,
        eth_params: EthTxOption,
    ) -> Result<String, JsValue> {
        let contract = ZkLinkAddress::from_str(&contract)
            .map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let amount = BigUint::from_str(&amount)
            .map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let eth_params: InnerEthTxOption =
            serde_wasm_bindgen::from_value(eth_params.json_value().unwrap()).unwrap();
        let tx_hash = self
            .inner
            .approve_erc20(contract, amount, eth_params)
            .await?;
        Ok(tx_hash.to_string())
    }

    #[wasm_bindgen(js_name=depositERC20)]
    pub async fn deposit_erc20(
        &self,
        sub_account_id: u8,
        deposit_to: String,
        token_addr: String,
        amount: String,
        mapping: bool,
        eth_params: EthTxOption,
        is_gateway: bool,
    ) -> Result<String, JsValue> {
        let deposit_to = ZkLinkAddress::from_str(&deposit_to)
            .map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let token_addr = ZkLinkAddress::from_str(&token_addr)
            .map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let amount = BigUint::from_str(&amount)
            .map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let eth_params: InnerEthTxOption =
            serde_wasm_bindgen::from_value(eth_params.json_value().unwrap()).unwrap();
        let tx_hash = if !is_gateway {
            self.inner
                .deposit_erc20_to_layer1(
                    sub_account_id,
                    deposit_to,
                    token_addr,
                    amount,
                    mapping,
                    eth_params,
                )
                .await?
        } else {
            self.inner
                .deposit_erc20_to_gateway(
                    sub_account_id,
                    deposit_to,
                    token_addr,
                    amount,
                    mapping,
                    eth_params,
                )
                .await?
        };
        Ok(tx_hash.to_string())
    }

    #[wasm_bindgen(js_name=depositETH)]
    pub async fn deposit_eth(
        &self,
        sub_account_id: u8,
        deposit_to: String,
        eth_params: EthTxOption,
        is_gateway: bool,
    ) -> Result<String, JsValue> {
        let deposit_to = ZkLinkAddress::from_str(&deposit_to)
            .map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let eth_params: InnerEthTxOption =
            serde_wasm_bindgen::from_value(eth_params.json_value().unwrap()).unwrap();
        let tx_hash = if !is_gateway {
            self.inner
                .deposit_eth_to_layer1(sub_account_id, deposit_to, eth_params)
                .await?
        } else {
            self.inner
                .deposit_eth_to_gateway(sub_account_id, deposit_to, eth_params)
                .await?
        };
        Ok(tx_hash.to_string())
    }
}
