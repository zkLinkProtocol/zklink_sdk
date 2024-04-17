use crate::abi::load_contracts;
use crate::error::WalletError;
use crate::eth::{encode_tx, new_call_typed_tx, new_typed_tx, EthTxOption, EthTxParam};
use bigdecimal::num_bigint::BigUint;
use ethers::abi::{Address, Contract, Detokenize, Token, Tokenize, Uint};
use ethers::contract::encode_function_data;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::BlockNumber;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use wasm_bindgen::prelude::wasm_bindgen;
use zklink_sdk_signers::eth_signer::EthSigner;
use zklink_sdk_types::basic_types::ZkLinkAddress;
use zklink_sdk_types::prelude::{PubKeyHash, H256, U256};

pub struct Wallet {
    pub contracts: HashMap<String, Contract>,
    pub signer: EthSigner,
    pub provider: Arc<Provider<Http>>,
}

#[wasm_bindgen]
pub enum WaitForTxStatus {
    Success,
    Failed,
    Pending,
}

impl Wallet {
    pub fn new(url: &str, private_key: &str) -> Self {
        let signer = EthSigner::try_from(private_key).unwrap();
        let provider = Arc::new(Provider::<Http>::try_from(url).unwrap());
        let contracts = load_contracts();
        Self {
            signer,
            provider,
            contracts,
        }
    }

    pub fn get_l1_contract(&self, is_gateway: bool) -> Contract {
        let contract_name = if is_gateway { "l1_gateway" } else { "zklink" };
        self.contracts.get(&contract_name.to_string()).unwrap().clone()
    }

    pub async fn get_balance(&self) -> Result<U256, WalletError> {
        let from = self.signer.get_address();
        Ok(self.provider.get_balance(from, None).await?)
    }

    pub async fn get_nonce(&self, block_number: String) -> Result<U256, WalletError> {
        let block_number = BlockNumber::from_str(&block_number).map_err(|_e| WalletError::InvalidInputParameter)?;
        let from = self.signer.get_address();
        Ok(self
            .provider
            .get_transaction_count(from, Some(block_number.into()))
            .await?)
    }

    pub async fn wait_for_transaction(
        &self,
        tx_hash: H256,
        retries: Option<u32>,
    ) -> Result<WaitForTxStatus, WalletError> {
        let mut retries = retries.unwrap_or(60);
        loop {
            let receipt = self.provider.get_transaction_receipt(tx_hash).await?;
            if let Some(receipt) = receipt {
                if let Some(status) = receipt.status {
                    if status.as_u64() == 1 {
                        return Ok(WaitForTxStatus::Success);
                    } else {
                        return Ok(WaitForTxStatus::Failed);
                    }
                }
            }
            retries -= 1;
            if retries == 0 {
                return Ok(WaitForTxStatus::Pending);
            }
            async_std::task::sleep(Duration::from_secs(1)).await
        }
    }

    pub async fn tx_call<T: Tokenize>(
        &self,
        eth_params: EthTxParam,
        is_gateway: bool,
        method: &str,
        args: T,
    ) -> Result<Vec<Token>, WalletError> {
        let contract = self.get_l1_contract(is_gateway);
        let function = contract.function(method).map_err(WalletError::EthAbiError)?;
        let encoded_data = encode_function_data(function, args).map_err(WalletError::AbiError)?;
        let params = EthTxParam {
            data: Some(encoded_data.to_vec()),
            ..eth_params.clone()
        };
        let chain_id = self.provider.get_chainid().await?;
        let typed_tx = new_call_typed_tx(params, chain_id.as_u64());
        let data: Vec<u8> = (*(self.provider).call(&typed_tx, None).await?).to_vec();
        let tokens = function.decode_output(&data).map_err(WalletError::EthAbiError)?;
        Ok(tokens)
    }

    pub async fn sign_and_send_raw_tx(&self, params: EthTxParam) -> Result<H256, WalletError> {
        let from = self.signer.get_address();
        let gas_price = if let Some(gas_price) = params.gas_price {
            gas_price
        } else {
            self.provider.get_gas_price().await?
        };
        let nonce = if let Some(nonce) = params.nonce {
            nonce
        } else {
            self.provider
                .get_transaction_count(from, Some(ethers::types::BlockNumber::Pending.into()))
                .await?
        };

        let chain_id = self.provider.get_chainid().await?;
        let tx_params = EthTxParam {
            nonce: Some(nonce),
            gas_price: Some(gas_price),
            ..params.clone()
        };
        let mut typed_tx = new_typed_tx(from, tx_params, chain_id.as_u64());
        if typed_tx.gas().is_none() {
            let gas_limit = self.provider.estimate_gas(&typed_tx, None).await?;
            typed_tx.set_gas(gas_limit);
        }
        let signature = self.signer.sign_transaction(&typed_tx)?;
        let raw_tx = typed_tx.rlp_signed(&signature.0).to_vec();
        let pending_tx = *(self.provider).send_raw_transaction(raw_tx.into()).await?;
        Ok(pending_tx)
    }

    pub async fn inner_approve_erc20(
        &self,
        zklink_addr: Address,
        amount: BigUint,
        eth_params: EthTxParam,
    ) -> Result<H256, WalletError> {
        let params = vec![
            Token::Address(zklink_addr),
            Token::Uint(ethers::types::U256::from_str(&amount.to_string()).unwrap()),
        ];
        let contract = self.contracts.get(&"erc20".to_owned()).unwrap();
        let tx_data = encode_tx(contract.clone(), "approve", params)?;
        let tx_params = EthTxParam {
            data: Some(tx_data),
            ..eth_params.clone()
        };
        let tx_hash = self.sign_and_send_raw_tx(tx_params).await?;
        Ok(tx_hash)
    }

    pub async fn inner_deposit_eth(
        &self,
        sub_account_id: u8,
        deposit_to: Address,
        is_gateway: bool,
        eth_params: EthTxParam,
    ) -> Result<H256, WalletError> {
        let mut bytes = [0; 32];
        bytes[12..].copy_from_slice(deposit_to.as_bytes());
        let params = vec![
            Token::FixedBytes(bytes.to_vec()),
            Token::Uint(ethers::types::U256::from(sub_account_id)),
        ];
        let contract = self.get_l1_contract(is_gateway);
        let tx_data = encode_tx(contract.clone(), "depositETH", params)?;
        let tx_params = EthTxParam {
            data: Some(tx_data),
            ..eth_params.clone()
        };
        let tx_hash = self.sign_and_send_raw_tx(tx_params).await?;
        Ok(tx_hash)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn inner_full_exit(
        &self,
        account_id: u32,
        sub_account_id: u8,
        token_id: u16,
        mapping: bool,
        eth_params: EthTxParam,
    ) -> Result<H256, WalletError> {
        let params = vec![
            Token::Uint(ethers::types::U256::from(account_id)),
            Token::Uint(ethers::types::U256::from(sub_account_id)),
            Token::Uint(ethers::types::U256::from(token_id)),
            Token::Bool(mapping),
        ];
        let contract = self.get_l1_contract(false);
        let tx_data = encode_tx(contract.clone(), "requestFullExit", params)?;
        let tx_params = EthTxParam {
            data: Some(tx_data),
            ..eth_params.clone()
        };
        let tx_hash = self.sign_and_send_raw_tx(tx_params).await?;
        Ok(tx_hash)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn inner_deposit_erc20(
        &self,
        sub_account_id: u8,
        deposit_to: Address,
        token_addr: Address,
        amount: BigUint,
        mapping: bool,
        is_gateway: bool,
        eth_params: EthTxParam,
    ) -> Result<H256, WalletError> {
        let mut bytes = [0; 32];
        // Token::FixedBytes will encode the bytes to right padded,eg. address is "0x1234...4321",
        // the encode result will be incorrect "0x1234...4321000...000".So there must left padded first.
        bytes[12..].copy_from_slice(deposit_to.as_bytes());
        let params = vec![
            Token::Address(token_addr),
            Token::Uint(ethers::types::U256::from_str_radix(&amount.to_string(), 10).unwrap()),
            Token::FixedBytes(bytes.to_vec()),
            Token::Uint(ethers::types::U256::from(sub_account_id)),
            Token::Bool(mapping),
        ];
        let contract = self.get_l1_contract(is_gateway);
        let tx_data = encode_tx(contract.clone(), "depositERC20", params)?;
        let tx_params = EthTxParam {
            data: Some(tx_data),
            ..eth_params.clone()
        };
        let tx_hash = self.sign_and_send_raw_tx(tx_params).await?;
        Ok(tx_hash)
    }

    pub async fn inner_set_auth_pubkey_hash(
        &self,
        nonce: u64,
        new_pubkey_hash: PubKeyHash,
        eth_params: EthTxParam,
    ) -> Result<H256, WalletError> {
        let mut bytes = [0; 32];
        bytes[12..].copy_from_slice(new_pubkey_hash.as_ref());
        let params = vec![
            Token::Bytes(bytes.to_vec()),
            Token::Uint(ethers::types::U256::from(nonce)),
        ];
        let contract = self.get_l1_contract(false);
        let tx_data = encode_tx(contract.clone(), "setAuthPubkeyHash", params)?;
        let tx_params = EthTxParam {
            data: Some(tx_data),
            ..eth_params.clone()
        };
        let tx_hash = self.sign_and_send_raw_tx(tx_params).await?;
        Ok(tx_hash)
    }

    pub async fn approve_erc20(
        &self,
        zklink_addr: ZkLinkAddress,
        amount: BigUint,
        eth_params: EthTxOption,
    ) -> Result<H256, WalletError> {
        let zklink_addr = Address::from_slice(zklink_addr.as_bytes());
        self.inner_approve_erc20(zklink_addr, amount, eth_params.into()).await
    }

    pub async fn deposit_eth_to_layer1(
        &self,
        sub_account_id: u8,
        deposit_to: ZkLinkAddress,
        eth_params: EthTxOption,
    ) -> Result<H256, WalletError> {
        let deposit_to = Address::from_slice(deposit_to.as_bytes());
        self.inner_deposit_eth(sub_account_id, deposit_to, false, eth_params.into())
            .await
    }

    pub async fn deposit_eth_to_gateway(
        &self,
        sub_account_id: u8,
        deposit_to: ZkLinkAddress,
        eth_params: EthTxOption,
    ) -> Result<H256, WalletError> {
        let deposit_to = Address::from_slice(deposit_to.as_bytes());
        self.inner_deposit_eth(sub_account_id, deposit_to, true, eth_params.into())
            .await
    }

    pub async fn deposit_erc20_to_layer1(
        &self,
        sub_account_id: u8,
        deposit_to: ZkLinkAddress,
        token_addr: ZkLinkAddress,
        amount: BigUint,
        mapping: bool,
        eth_params: EthTxOption,
    ) -> Result<H256, WalletError> {
        let deposit_to = Address::from_slice(deposit_to.as_bytes());
        let token_addr = Address::from_slice(token_addr.as_bytes());
        self.inner_deposit_erc20(
            sub_account_id,
            deposit_to,
            token_addr,
            amount,
            mapping,
            false,
            eth_params.into(),
        )
        .await
    }

    pub async fn deposit_erc20_to_gateway(
        &self,
        sub_account_id: u8,
        deposit_to: ZkLinkAddress,
        token_addr: ZkLinkAddress,
        amount: BigUint,
        mapping: bool,
        eth_params: EthTxOption,
    ) -> Result<H256, WalletError> {
        let deposit_to = Address::from_slice(deposit_to.as_bytes());
        let token_addr = Address::from_slice(token_addr.as_bytes());
        self.inner_deposit_erc20(
            sub_account_id,
            deposit_to,
            token_addr,
            amount,
            mapping,
            true,
            eth_params.into(),
        )
        .await
    }

    pub async fn set_auth_pubkey_hash(
        &self,
        nonce: u64,
        new_pubkey_hash: PubKeyHash,
        eth_params: EthTxOption,
    ) -> Result<H256, WalletError> {
        self.inner_set_auth_pubkey_hash(nonce, new_pubkey_hash, eth_params.into())
            .await
    }

    pub async fn full_exit(
        &self,
        account_id: u32,
        sub_account_id: u8,
        token_id: u16,
        mapping: bool,
        eth_params: EthTxOption,
    ) -> Result<H256, WalletError> {
        self.inner_full_exit(account_id, sub_account_id, token_id, mapping, eth_params.into())
            .await
    }

    pub async fn inner_get_fee(&self, eth_params: EthTxParam) -> Result<BigUint, WalletError> {
        let tokens = self.tx_call(eth_params, true, "fee", ()).await?;
        let fee = Uint::from_tokens(tokens).map_err(|e| WalletError::GetErrorResult(e.0))?;
        Ok(BigUint::from_str(&fee.to_string()).unwrap())
    }

    pub async fn get_fee(&self, eth_params: EthTxOption) -> Result<BigUint, WalletError> {
        self.inner_get_fee(eth_params.into()).await
    }
}
