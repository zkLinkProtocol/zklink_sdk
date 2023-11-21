use crate::abi::load_contracts;
use crate::error::WalletError;
use crate::eth::{encode_tx, new_typed_tx, EthTxOption, EthTxParam};
use bigdecimal::num_bigint::BigUint;
use ethers::abi::{Address, Contract, Token};
use ethers::providers::{Http, Middleware, Provider};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use zklink_sdk_signers::eth_signer::EthSigner;
use zklink_sdk_types::basic_types::ZkLinkAddress;
use zklink_sdk_types::prelude::H256;

pub struct Wallet {
    pub contracts: HashMap<String, Contract>,
    pub signer: EthSigner,
    pub provider: Arc<Provider<Http>>,
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
        self.contracts
            .get(&contract_name.to_string())
            .unwrap()
            .clone()
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

    pub async fn approve_erc20(
        &self,
        zklink_addr: ZkLinkAddress,
        amount: BigUint,
        eth_params: EthTxOption,
    ) -> Result<H256, WalletError> {
        let zklink_addr = Address::from_slice(zklink_addr.as_bytes());
        self.inner_approve_erc20(zklink_addr, amount, eth_params.into())
            .await
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
}
