use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::prelude::ZkLinkAddress;
use zklink_sdk_wallet::eth::EthTxOption as InnerEthTxOption;
use zklink_sdk_wallet::Wallet as InnerWallet;

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
    ) -> EthTxOption {
        let inner = InnerEthTxOption {
            is_support_eip1559,
            to: ZkLinkAddress::from_str(&to).unwrap(),
            nonce: nonce.map(|n| n as u64),
            value: value.map(|v| BigUint::from_str(&v).unwrap_or_default()),
            gas: gas.map(|g| g as u64),
            gas_price: gas_price.map(|g| BigUint::from_str(&g).unwrap_or_default()),
        };
        EthTxOption { inner }
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
        let contract = ZkLinkAddress::from_str(&contract).unwrap();
        let amount = BigUint::from_str(&amount).unwrap();
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
    ) -> Result<String, JsValue> {
        let deposit_to = ZkLinkAddress::from_str(&deposit_to).unwrap();
        let token_addr = ZkLinkAddress::from_str(&token_addr).unwrap();
        let amount = BigUint::from_str(&amount).unwrap();
        let eth_params: InnerEthTxOption =
            serde_wasm_bindgen::from_value(eth_params.json_value().unwrap()).unwrap();
        let tx_hash = self
            .inner
            .deposit_erc20(
                sub_account_id,
                deposit_to,
                token_addr,
                amount,
                mapping,
                eth_params,
            )
            .await?;
        Ok(tx_hash.to_string())
    }
}
