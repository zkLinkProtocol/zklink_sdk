use crate::basic_types::{
    AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::tx_type::order_matching::Order;

use num::BigUint;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_signers::eth_signer::H256;
use zklink_signers::zklink_signer::pubkey_hash::PubKeyHash;

pub struct ChangePubKeyBuilder {
    pub chain_id: ChainId,
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub new_pubkey_hash: PubKeyHash,
    pub fee_token: TokenId,
    pub fee: BigUint,
    pub nonce: Nonce,
    pub eth_signature: Option<PackedEthSignature>,
    pub ts: TimeStamp,
}

pub struct TransferBuilder {
    pub account_id: AccountId,
    pub to_address: ZkLinkAddress,
    pub from_sub_account_id: SubAccountId,
    pub to_sub_account_id: SubAccountId,
    pub token: TokenId,
    pub amount: BigUint,
    pub fee: BigUint,
    pub nonce: Nonce,
    pub timestamp: TimeStamp,
}

pub struct DepositBuilder {
    pub from_address: ZkLinkAddress,
    pub to_address: ZkLinkAddress,
    pub from_chain_id: ChainId,
    pub sub_account_id: SubAccountId,
    pub l2_target_token: TokenId,
    pub l1_source_token: TokenId,
    pub amount: BigUint,
    pub serial_id: u64,
    pub eth_hash: H256,
}

pub struct WithdrawBuilder {
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub to_chain_id: ChainId,
    pub to_address: ZkLinkAddress,
    pub l2_source_token: TokenId,
    pub l1_target_token: TokenId,
    pub amount: BigUint,
    pub fee: BigUint,
    pub nonce: Nonce,
    pub fast_withdraw: bool,
    pub withdraw_fee_ratio: u16,
    pub timestamp: TimeStamp,
}

pub struct ForcedExitBuilder {
    pub to_chain_id: ChainId,
    pub initiator_account_id: AccountId,
    pub initiator_sub_account_id: SubAccountId,
    pub target: ZkLinkAddress,
    pub target_sub_account_id: SubAccountId,
    pub l2_source_token: TokenId,
    pub l1_target_token: TokenId,
    pub initiator_nonce: Nonce,
    pub exit_amount: BigUint,
    pub ts: TimeStamp,
}

pub struct FullExitBuilder {
    pub to_chain_id: ChainId,
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub exit_address: ZkLinkAddress,
    pub l2_source_token: TokenId,
    pub l1_target_token: TokenId,
    pub serial_id: u64,
    pub eth_hash: H256,
}

#[cfg(feature = "ffi")]
pub struct OrderMatchingBuilder {
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub taker: Arc<Order>,
    pub maker: Arc<Order>,
    pub fee: BigUint,
    pub fee_token: TokenId,
    pub expect_base_amount: BigUint,
    pub expect_quote_amount: BigUint,
}

#[cfg(not(feature = "ffi"))]
pub struct OrderMatchingBuilder {
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub taker: Order,
    pub maker: Order,
    pub fee: BigUint,
    pub fee_token: TokenId,
    pub expect_base_amount: BigUint,
    pub expect_quote_amount: BigUint,
}
