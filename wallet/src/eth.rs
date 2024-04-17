use crate::error::WalletError;
use ethers::abi::{Contract, Token};
use ethers::core::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Address, Eip1559TransactionRequest, TransactionRequest, U256};
use serde::{Deserialize, Serialize};
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::prelude::ZkLinkAddress;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EthTxOption {
    pub is_support_eip1559: bool,
    pub to: ZkLinkAddress,
    pub nonce: Option<u64>,
    pub value: Option<BigUint>,
    pub gas: Option<u64>,
    pub gas_price: Option<BigUint>,
}

#[derive(Clone, Default)]
pub struct EthTxParam {
    pub(crate) is_support_eip1559: bool,
    pub(crate) to: Address,
    pub(crate) data: Option<Vec<u8>>,
    pub(crate) nonce: Option<U256>,
    pub(crate) value: Option<U256>,
    pub(crate) gas: Option<U256>,
    pub(crate) gas_price: Option<U256>,
}

impl From<EthTxOption> for EthTxParam {
    fn from(eth_tx_option: EthTxOption) -> EthTxParam {
        EthTxParam {
            is_support_eip1559: eth_tx_option.is_support_eip1559,
            to: Address::from_slice(eth_tx_option.to.as_bytes()),
            data: None,
            nonce: eth_tx_option.nonce.map(U256::from),
            value: eth_tx_option
                .value
                .map(|v| U256::from_str_radix(&v.to_string(), 10).unwrap_or_default()),
            gas: eth_tx_option.gas.map(U256::from),
            gas_price: eth_tx_option
                .gas_price
                .map(|g| U256::from_str_radix(&g.to_string(), 10).unwrap_or_default()),
        }
    }
}

pub fn new_call_typed_tx(tx_params: EthTxParam, chain_id: u64) -> TypedTransaction {
    let mut tx = if tx_params.is_support_eip1559 {
        TypedTransaction::Eip1559(Eip1559TransactionRequest::new())
    } else {
        TypedTransaction::Legacy(TransactionRequest::new())
    };
    tx.set_to(tx_params.to);
    if let Some(data) = tx_params.data {
        tx.set_data(data.into());
    }
    tx.set_chain_id(chain_id);
    tx
}

pub fn new_typed_tx(from: Address, tx_params: EthTxParam, chain_id: u64) -> TypedTransaction {
    let mut tx = if tx_params.is_support_eip1559 {
        TypedTransaction::Eip1559(Eip1559TransactionRequest::new())
    } else {
        TypedTransaction::Legacy(TransactionRequest::new())
    };
    tx.set_from(from);
    tx.set_to(tx_params.to);
    tx.set_nonce(tx_params.nonce.unwrap());
    if let Some(data) = tx_params.data {
        tx.set_data(data.into());
    }
    if let Some(gas) = tx_params.gas {
        tx.set_gas(gas);
    }
    if let Some(value) = tx_params.value {
        tx.set_value(value);
    }
    if let Some(gas_price) = tx_params.gas_price {
        tx.set_gas_price(gas_price);
    }
    tx.set_chain_id(chain_id);
    tx
}

pub fn encode_tx(contract: Contract, method: &str, params: Vec<Token>) -> Result<Vec<u8>, WalletError> {
    let function = contract.function(method)?;
    let tx_data = function.encode_input(&params)?;
    Ok(tx_data)
}
