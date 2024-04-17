use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::prelude::{PubKeyHash, ZkLinkAddress, H256};
use zklink_sdk_wallet::eth::EthTxOption as InnerEthTxOption;
use zklink_sdk_wallet::wallet::{WaitForTxStatus, Wallet as InnerWallet};

#[wasm_bindgen]
pub enum BlockNumber {
    Latest,
    Finalized,
    Safe,
    Earliest,
    Pending,
    Number,
}

impl std::fmt::Display for BlockNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockNumber::Latest => write!(f, "latest"),
            BlockNumber::Finalized => write!(f, "finalized"),
            BlockNumber::Safe => write!(f, "safe"),
            BlockNumber::Earliest => write!(f, "earliest"),
            BlockNumber::Pending => write!(f, "pending"),
            BlockNumber::Number => write!(f, "number"),
        }
    }
}

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
            Some(BigUint::from_str(&v).map_err(|error| JsValue::from_str(&format!("error: {error}")))?)
        } else {
            None
        };

        let gas_price = if let Some(g) = gas_price {
            Some(BigUint::from_str(&g).map_err(|error| JsValue::from_str(&format!("error: {error}")))?)
        } else {
            None
        };
        let inner = InnerEthTxOption {
            is_support_eip1559,
            to: ZkLinkAddress::from_str(&to).map_err(|error| JsValue::from_str(&format!("error: {error}")))?,
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

    #[wasm_bindgen(js_name=getBalance)]
    pub async fn get_balance(&self) -> Result<String, JsValue> {
        let balance = self.inner.get_balance().await?;
        Ok(balance.to_string())
    }

    #[wasm_bindgen(js_name=getNonce)]
    pub async fn get_nonce(&self, block_number: BlockNumber, block: Option<f64>) -> Result<f64, JsValue> {
        let block_number = if matches!(block_number, BlockNumber::Number) {
            format!("{}", block.unwrap_or_default())
        } else {
            block_number.to_string()
        };
        let nonce = self.inner.get_nonce(block_number).await?;
        Ok(nonce.as_u64() as f64)
    }

    #[wasm_bindgen(js_name=getDepositFee)]
    pub async fn get_deposit_fee(&self, eth_params: EthTxOption) -> Result<String, JsValue> {
        let eth_params: InnerEthTxOption = serde_wasm_bindgen::from_value(eth_params.json_value().unwrap()).unwrap();
        let fee = self.inner.get_fee(eth_params).await?;
        Ok(fee.to_string())
    }

    #[wasm_bindgen(js_name=waitForTransaction)]
    pub async fn wait_for_transaction(
        &self,
        tx_hash: String,
        timeout: Option<u32>,
    ) -> Result<WaitForTxStatus, JsValue> {
        let tx_hash = H256::from_str(&tx_hash).map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let status = self.inner.wait_for_transaction(tx_hash, timeout).await?;
        Ok(status)
    }

    #[wasm_bindgen(js_name=approveERC20)]
    pub async fn approve_erc20(
        &self,
        contract: String,
        amount: String,
        eth_params: EthTxOption,
    ) -> Result<String, JsValue> {
        let contract =
            ZkLinkAddress::from_str(&contract).map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let amount = BigUint::from_str(&amount).map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let eth_params: InnerEthTxOption = serde_wasm_bindgen::from_value(eth_params.json_value().unwrap()).unwrap();
        let tx_hash = self.inner.approve_erc20(contract, amount, eth_params).await?;
        Ok(hex::encode(tx_hash.as_bytes()))
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
        let deposit_to =
            ZkLinkAddress::from_str(&deposit_to).map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let token_addr =
            ZkLinkAddress::from_str(&token_addr).map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let amount = BigUint::from_str(&amount).map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let eth_params: InnerEthTxOption = serde_wasm_bindgen::from_value(eth_params.json_value().unwrap()).unwrap();
        let tx_hash = if !is_gateway {
            self.inner
                .deposit_erc20_to_layer1(sub_account_id, deposit_to, token_addr, amount, mapping, eth_params)
                .await?
        } else {
            self.inner
                .deposit_erc20_to_gateway(sub_account_id, deposit_to, token_addr, amount, mapping, eth_params)
                .await?
        };
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    #[wasm_bindgen(js_name=depositETH)]
    pub async fn deposit_eth(
        &self,
        sub_account_id: u8,
        deposit_to: String,
        eth_params: EthTxOption,
        is_gateway: bool,
    ) -> Result<String, JsValue> {
        let deposit_to =
            ZkLinkAddress::from_str(&deposit_to).map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let eth_params: InnerEthTxOption = serde_wasm_bindgen::from_value(eth_params.json_value().unwrap()).unwrap();
        let tx_hash = if !is_gateway {
            self.inner
                .deposit_eth_to_layer1(sub_account_id, deposit_to, eth_params)
                .await?
        } else {
            self.inner
                .deposit_eth_to_gateway(sub_account_id, deposit_to, eth_params)
                .await?
        };
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    #[wasm_bindgen(js_name=setAuthPubkeyHash)]
    pub async fn set_auth_pubkey_hash(
        &self,
        nonce: f64,
        new_pubkey_hash: String,
        eth_params: EthTxOption,
    ) -> Result<String, JsValue> {
        let eth_params: InnerEthTxOption = serde_wasm_bindgen::from_value(eth_params.json_value().unwrap()).unwrap();
        let new_pubkey_hash =
            PubKeyHash::from_hex(&new_pubkey_hash).map_err(|error| JsValue::from_str(&format!("error: {error}")))?;
        let tx_hash = self
            .inner
            .set_auth_pubkey_hash(nonce as u64, new_pubkey_hash, eth_params)
            .await?;
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    #[wasm_bindgen(js_name=fullExit)]
    pub async fn full_exit(
        &self,
        account_id: u32,
        sub_account_id: u8,
        token_id: u16,
        mapping: bool,
        eth_params: EthTxOption,
    ) -> Result<String, JsValue> {
        let eth_params: InnerEthTxOption = serde_wasm_bindgen::from_value(eth_params.json_value().unwrap()).unwrap();
        let tx_hash = self
            .inner
            .full_exit(account_id, sub_account_id, token_id, mapping, eth_params)
            .await?;
        Ok(hex::encode(tx_hash.as_bytes()))
    }
}
