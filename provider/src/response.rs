use bigdecimal::BigDecimal;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use chrono::serde::{ts_microseconds, ts_microseconds_option};
use ethers::types::{H256, U256};
use zklink_sdk_utils::serde::BigUintSerdeWrapper;
use zklink_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_types::basic_types::tx_hash::TxHash;
use zklink_types::basic_types::{
    AccountId, BlockNumber, ChainId, Nonce, SlotId, SubAccountId, TokenId, ZkLinkAddress,
};
use zklink_types::tx_type::zklink_tx::ZkLinkTx;

pub type SubAccountNonces = HashMap<SubAccountId, Nonce>;
pub type SubAccountBalances = HashMap<SubAccountId, HashMap<TokenId, BigUintSerdeWrapper>>;
pub type SubAccountOrders = HashMap<SubAccountId, HashMap<SlotId, ResponseTidyOrder>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainResp {
    pub chain_id: ChainId,
    pub chain_type: u8,
    pub layer_one_chain_id: u32,
    pub main_contract: ZkLinkAddress,
    pub layer_zero_contract: ZkLinkAddress,
    pub web3_url: String,
    pub fee_cap: U256,
    pub gas_token_id: TokenId,
    pub validator: ZkLinkAddress,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenResp {
    /// id is used for tx signature and serialization
    pub id: TokenId,
    /// Token symbol (e.g. "ETH" or "USDC")
    pub symbol: String,
    /// Token price
    pub usd_price: BigDecimal,
    /// Token info of each layer one chain
    pub chains: HashMap<ChainId, ChainTokenResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainTokenResp {
    /// chains is used to mark which chain(s) the token can be used
    pub chain_id: ChainId,
    /// Contract address of ERC20 token or Address::zero() for "ETH"
    pub address: ZkLinkAddress,
    /// Token precision (e.g. 18 for "ETH" so "1.0" ETH = 10e18 as U256 number)
    pub decimals: u8,
    /// Is token can fast withdraw
    pub fast_withdraw: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockNumberResp {
    pub last_block_number: u32,
    pub timestamp: u64,
    pub committed: u32,
    pub verified: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockResp {
    pub number: BlockNumber,
    pub commitment: H256,
    pub root_hash: H256,
    pub fee_account_id: AccountId,
    pub block_size: u64,
    pub ops_composition_number: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<TxHashOrDetailResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TxHashOrDetailResp {
    Hash(TxHash),
    TxDetail(BlockTxResp),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockOnChainResp {
    pub committed: Vec<OnChainResp>,
    pub proved: Vec<OnChainResp>,
    pub verified: Vec<OnChainResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnChainResp {
    pub chain_id: ChainId,
    pub tx_hash: H256,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AccountQuery {
    Id(AccountId),
    Address(ZkLinkAddress),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoResp {
    pub id: AccountId,
    pub address: ZkLinkAddress,
    pub nonce: Nonce,
    pub pub_key_hash: PubKeyHash,
    pub sub_account_nonces: SubAccountNonces,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTidyOrder {
    pub nonce: Nonce,
    pub residue: BigDecimal,
}

impl ResponseTidyOrder {
    pub fn new(nonce: Nonce, residue: BigDecimal) -> Self {
        Self { nonce, residue }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountSnapshotResp {
    pub id: AccountId,
    pub address: ZkLinkAddress,
    pub nonce: Nonce,
    pub pub_key_hash: PubKeyHash,
    pub sub_account_nonces: SubAccountNonces,
    pub balances: SubAccountBalances,
    pub order_slots: SubAccountOrders,
    pub block_number: BlockNumber,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum StateUpdateResp {
    AccountCreate(AccountCreateResp),
    AccountChangePubkeyUpdate(AccountChangePubkeyUpdateResp),
    BalanceUpdate(BalanceUpdateResp),
    OrderUpdate(OrderUpdateResp),
}

impl StateUpdateResp {
    pub fn update_id(&self) -> i32 {
        match self {
            StateUpdateResp::AccountCreate(u) => u.update_id,
            StateUpdateResp::AccountChangePubkeyUpdate(u) => u.update_id,
            StateUpdateResp::BalanceUpdate(u) => u.update_id,
            StateUpdateResp::OrderUpdate(u) => u.update_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountCreateResp {
    pub update_id: i32,
    pub account_id: AccountId,
    pub address: ZkLinkAddress,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountChangePubkeyUpdateResp {
    pub update_id: i32,
    pub account_id: AccountId,
    pub old_pubkey_hash: PubKeyHash,
    pub new_pubkey_hash: PubKeyHash,
    pub old_nonce: Nonce,
    pub new_nonce: Nonce,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceUpdateResp {
    pub update_id: i32,
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub coin_id: TokenId,
    pub old_balance: BigUintSerdeWrapper,
    pub new_balance: BigUintSerdeWrapper,
    pub old_nonce: Nonce,
    pub new_nonce: Nonce,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdateResp {
    pub update_id: i32,
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub slot_id: SlotId,
    pub old_tidy_order: ResponseTidyOrder,
    pub new_tidy_order: ResponseTidyOrder,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TxResp {
    pub tx_hash: TxHash,
    pub tx: ZkLinkTx,
    pub receipt: TxReceiptResp,
    pub updates: Vec<StateUpdateResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TxReceiptResp {
    pub executed: bool,
    #[serde(with = "ts_microseconds_option")]
    pub executed_timestamp: Option<DateTime<Utc>>,
    pub success: bool,
    pub fail_reason: Option<String>,
    pub block: Option<BlockNumber>,
    pub index: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockTxResp {
    pub tx_hash: TxHash,
    pub tx: ZkLinkTx,
    #[serde(with = "ts_microseconds")]
    pub executed_timestamp: DateTime<Utc>,
    pub updates: Vec<StateUpdateResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FastWithdrawTxResp {
    pub tx_hash: TxHash,
    pub tx: ZkLinkTx,
    #[serde(with = "ts_microseconds")]
    pub executed_timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForwardTxResp {
    pub tx_id: i64,
    pub op_type: i16,
    pub tx_hash: TxHash,
    pub tx: ZkLinkTx,
    pub executable: bool,
    pub receipt: TxReceiptResp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub total_page_num: u64,
    pub page_index: u64,
    pub page_size: u32,
    pub page_data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ZkLinkTxHistory {
    pub chain_id: ChainId,
    pub from_account: ZkLinkAddress,
    pub to_account: ZkLinkAddress,
    pub amount: BigUintSerdeWrapper,
    pub nonce: Nonce,
    pub tx: ZkLinkTx,
    pub tx_hash: TxHash,
    pub tx_receipt: TxReceiptResp,
    pub created_at: DateTime<Utc>,
}
