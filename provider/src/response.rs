use bigdecimal::BigDecimal;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use chrono::serde::{ts_microseconds, ts_microseconds_option};
use zklink_sdk_signers::eth_signer::H256;
use zklink_sdk_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_sdk_types::prelude::{
    AccountId, BigIntSerdeWrapper, BigUintSerdeWrapper, BlockNumber, ChainId, MarginId, Nonce, PairId, SlotId,
    SubAccountId, TokenId, TxHash, ZkLinkAddress, U256,
};
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;

pub type SubAccountNonces = HashMap<SubAccountId, Nonce>;
pub type SubAccountBalances = HashMap<SubAccountId, HashMap<TokenId, BigIntSerdeWrapper>>;
pub type SubAccountPositions = HashMap<SubAccountId, HashMap<PairId, ResponsePosition>>;
pub type SubAccountOrders = HashMap<SubAccountId, HashMap<SlotId, ResponseTidyOrder>>;
pub type SubAccountGlobalVars = HashMap<SubAccountId, GlobalVarsResp>;
pub type MarginParams = HashMap<MarginId, ResponseMarginParams>;
pub type ContractParams = HashMap<PairId, ResponseContractParams>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainResp {
    pub chain_id: ChainId,
    pub chain_type: u8,
    pub layer_one_chain_id: U256,
    pub main_contract: ZkLinkAddress,
    pub gas_token_id: TokenId,
    pub deposit_confirmation: u64,
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
    #[serde(with = "ts_microseconds")]
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
pub struct GlobalVarsResp {
    pub sub_account_id: SubAccountId,
    pub fee_account: Option<AccountId>,
    pub insurance_fund_account: Option<AccountId>,
    pub margin_params: MarginParams,
    pub contract_params: ContractParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMarginParams {
    pub token_id: TokenId,
    pub symbol: String,
    pub index_price: BigUintSerdeWrapper,
    pub ratio: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContractParams {
    pub symbol: String,
    pub mark_price: BigUintSerdeWrapper,
    pub initial_margin_rate: u16,
    pub maintenance_margin_rate: u16,
    pub acc_funding_price: BigIntSerdeWrapper,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTidyOrder {
    pub nonce: Nonce,
    pub residue: BigUintSerdeWrapper,
}

impl ResponseTidyOrder {
    pub fn new(nonce: Nonce, residue: BigUintSerdeWrapper) -> Self {
        Self { nonce, residue }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePosition {
    pub direction: bool,
    pub price: BigUintSerdeWrapper,
    pub value: BigUintSerdeWrapper,
    pub size: BigUintSerdeWrapper,
    pub acc_funding_price: BigIntSerdeWrapper,
}

impl ResponsePosition {
    pub fn new(
        direction: bool,
        price: BigUintSerdeWrapper,
        value: BigUintSerdeWrapper,
        size: BigUintSerdeWrapper,
        acc_funding_price: BigIntSerdeWrapper,
    ) -> Self {
        Self {
            direction,
            price,
            value,
            size,
            acc_funding_price,
        }
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
    pub positions: SubAccountPositions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "stateUpdateType")]
pub enum StateUpdateResp {
    AccountUpdate(AccountUpdateResp),
    GlobalVarsUpdate(GlobalVarsUpdateResp),
}

impl From<AccountUpdateResp> for StateUpdateResp {
    fn from(value: AccountUpdateResp) -> Self {
        Self::AccountUpdate(value)
    }
}

impl From<GlobalVarsUpdateResp> for StateUpdateResp {
    fn from(value: GlobalVarsUpdateResp) -> Self {
        Self::GlobalVarsUpdate(value)
    }
}

impl StateUpdateResp {
    pub fn update_id(&self) -> i32 {
        match self {
            StateUpdateResp::AccountUpdate(u) => u.update_id(),
            StateUpdateResp::GlobalVarsUpdate(u) => u.update_id(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "accountUpdateType")]
pub enum AccountUpdateResp {
    AccountCreate(AccountCreateResp),
    AccountChangePubkeyUpdate(AccountChangePubkeyUpdateResp),
    BalanceUpdate(BalanceUpdateResp),
    OrderUpdate(OrderUpdateResp),
    PositionUpdate(PositionUpdateResp),
}

impl AccountUpdateResp {
    pub fn update_id(&self) -> i32 {
        match self {
            AccountUpdateResp::AccountCreate(u) => u.update_id,
            AccountUpdateResp::AccountChangePubkeyUpdate(u) => u.update_id,
            AccountUpdateResp::BalanceUpdate(u) => u.update_id,
            AccountUpdateResp::OrderUpdate(u) => u.update_id,
            AccountUpdateResp::PositionUpdate(u) => u.update_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "globalVarUpdate")]
pub enum GlobalVarsUpdateResp {
    FeeAccountUpdate(FeeAccountUpdateResp),
    InsuranceFundAccountUpdate(InsuranceFundAccountUpdateResp),
    MarginParamsUpdate(MarginParamsUpdateResp),
    ContractParamsUpdate(ContractParamsUpdateResp),
}

impl GlobalVarsUpdateResp {
    pub fn update_id(&self) -> i32 {
        match self {
            GlobalVarsUpdateResp::FeeAccountUpdate(u) => u.update_id,
            GlobalVarsUpdateResp::InsuranceFundAccountUpdate(u) => u.update_id,
            GlobalVarsUpdateResp::MarginParamsUpdate(u) => u.update_id,
            GlobalVarsUpdateResp::ContractParamsUpdate(u) => u.update_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeAccountUpdateResp {
    pub update_id: i32,
    pub sub_account_id: SubAccountId,
    pub old_fee_account_id: AccountId,
    pub new_fee_account_id: AccountId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceFundAccountUpdateResp {
    pub update_id: i32,
    pub sub_account_id: SubAccountId,
    pub old_insurance_fund_account_id: AccountId,
    pub new_insurance_fund_account_id: AccountId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginParamsUpdateResp {
    pub update_id: i32,
    pub sub_account_id: SubAccountId,
    pub margin_id: MarginId,
    pub old_symbol: String,
    pub new_symbol: String,
    pub old_index_price: BigUintSerdeWrapper,
    pub new_index_price: BigUintSerdeWrapper,
    pub old_token_id: TokenId,
    pub new_token_id: TokenId,
    pub old_ratio: u8,
    pub new_ratio: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractParamsUpdateResp {
    pub update_id: i32,
    pub sub_account_id: SubAccountId,
    pub pair_id: PairId,
    pub old_symbol: String,
    pub new_symbol: String,
    pub old_maintenance_margin_rate: u16,
    pub new_maintenance_margin_rate: u16,
    pub old_initial_margin_rate: u16,
    pub new_initial_margin_rate: u16,
    pub old_acc_funding_price: BigIntSerdeWrapper,
    pub new_acc_funding_price: BigIntSerdeWrapper,
    pub old_mark_price: BigUintSerdeWrapper,
    pub new_mark_price: BigUintSerdeWrapper,
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
    pub old_balance: BigIntSerdeWrapper,
    pub new_balance: BigIntSerdeWrapper,
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
pub struct PositionUpdateResp {
    pub update_id: i32,
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub pair_id: PairId,
    pub old_position: ResponsePosition,
    pub new_position: ResponsePosition,
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
pub struct WithdrawTxResp {
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
    #[serde(with = "ts_microseconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EthPropertyResp {
    pub chain_id: ChainId,
    pub layer_one_chain_id: U256,
    pub gateways: Vec<GateWayInfo>,
    pub gas_token_id: TokenId,
    pub deposit_confirmation: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GateWayInfo {
    pub chain_id: ChainId,
    pub l1_gateway_contract: ZkLinkAddress,
    pub l2_gateway_contract: ZkLinkAddress,
    pub tokens: Vec<TokenInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub token_id: TokenId,
    pub token_address: ZkLinkAddress,
    pub decimal: u8,
    pub fast_withdraw: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rpc_response() {
        let s = r#" {
    "chainId": 4,
    "layerOneChainId": "0x1",
	"gateways": [
		{
			"chainId": 5,
			"l1GatewayContract": "0xffffffffffffffffffffffffffffffffffffffff",
			"l2GatewayContract": "0xffffffffffffffffffffffffffffffffffffffff",
			"tokens": [
				{
					"tokenId": 1,
					"tokenAddress":"0xffffffffffffffffffffffffffffffffffffffff",
					"decimal": 6,
					"fastWithdraw": true
				},
				{
					"tokenId": 3,
					"tokenAddress":"0xffffffffffffffffffffffffffffffffffffffff",
					"decimal": 7,
					"fastWithdraw": false
				}
			]
		},
		{
			"chainId": 7,
			"l1GatewayContract": "0xffffffffffffffffffffffffffffffffffffffff",
			"l2GatewayContract": "0xffffffffffffffffffffffffffffffffffffffff",
			"tokens": [
				{
					"tokenId": 1,
					"tokenAddress":"0xffffffffffffffffffffffffffffffffffffffff",
					"decimal": 6,
					"fastWithdraw": true
				},
				{
					"tokenId": 3,
					"tokenAddress":"0xffffffffffffffffffffffffffffffffffffffff",
					"decimal": 6,
					"fastWithdraw": false
				}
			]
		}
	],
    "gasTokenId": 1,
    "depositConfirmation": 200
}
        "#;
        let resp: Result<EthPropertyResp, _> = serde_json::from_str(s);
        println!("{:?}", resp);
        assert!(resp.is_ok());
    }
}
