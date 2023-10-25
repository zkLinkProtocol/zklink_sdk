use crate::basic_types::{
    AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::tx_type::order_matching::Order;

use crate::prelude::{
    ChangePubKey, ChangePubKeyAuthData, Deposit, ForcedExit, FullExit, OrderMatching, Transfer,
    Withdraw,
};
use crate::tx_type::exit_info::ExitInfo;
use num::BigUint;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::eth_signer::H256;
use zklink_sdk_signers::zklink_signer::pubkey_hash::PubKeyHash;

pub struct ChangePubKeyBuilder {
    pub chain_id: ChainId,
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub new_pubkey_hash: PubKeyHash,
    pub fee_token: TokenId,
    pub fee: BigUint,
    pub nonce: Nonce,
    pub eth_signature: Option<PackedEthSignature>,
    pub timestamp: TimeStamp,
}

impl ChangePubKeyBuilder {
    /// Creates ChangePubKey transaction
    pub fn build(self) -> ChangePubKey {
        let eth_auth_data = self
            .eth_signature
            .map(|eth_signature| ChangePubKeyAuthData::EthECDSA { eth_signature })
            .unwrap_or(ChangePubKeyAuthData::Onchain);

        ChangePubKey {
            chain_id: self.chain_id,
            account_id: self.account_id,
            sub_account_id: self.sub_account_id,
            new_pk_hash: self.new_pubkey_hash,
            fee_token: self.fee_token,
            fee: self.fee,
            nonce: self.nonce,
            signature: Default::default(),
            eth_auth_data,
            ts: self.timestamp,
        }
    }
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

impl TransferBuilder {
    /// Creates Transfer transaction
    pub fn build(self) -> Transfer {
        Transfer {
            account_id: self.account_id,
            from_sub_account_id: self.from_sub_account_id,
            to_sub_account_id: self.to_sub_account_id,
            to: self.to_address,
            token: self.token,
            amount: self.amount,
            fee: self.fee,
            nonce: self.nonce,
            signature: Default::default(),
            ts: self.timestamp,
        }
    }
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

impl DepositBuilder {
    /// Creates Deposit transaction
    pub fn build(self) -> Deposit {
        Deposit {
            from: self.from_address,
            to: self.to_address,
            from_chain_id: self.from_chain_id,
            sub_account_id: self.sub_account_id,
            l2_target_token: self.l2_target_token,
            l1_source_token: self.l1_source_token,
            amount: self.amount,
            serial_id: self.serial_id,
            eth_hash: self.eth_hash,
        }
    }
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
    pub withdraw_to_l1: bool,
    pub withdraw_fee_ratio: u16,
    pub timestamp: TimeStamp,
}

impl WithdrawBuilder {
    /// Creates Withdraw transaction
    pub fn build(self) -> Withdraw {
        let fast_withdraw = u8::from(self.fast_withdraw);
        let withdraw_to_l1 = u8::from(self.withdraw_to_l1);

        Withdraw {
            to_chain_id: self.to_chain_id,
            account_id: self.account_id,
            sub_account_id: self.sub_account_id,
            to: self.to_address,
            l2_source_token: self.l2_source_token,
            l1_target_token: self.l1_target_token,
            amount: self.amount,
            fee: self.fee,
            nonce: self.nonce,
            signature: Default::default(),
            fast_withdraw,
            withdraw_to_l1,
            withdraw_fee_ratio: self.withdraw_fee_ratio,
            ts: self.timestamp,
        }
    }
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
    pub withdraw_to_l1: bool,
    pub timestamp: TimeStamp,
}

impl ForcedExitBuilder {
    /// Creates ForcedExit transaction
    pub fn build(self) -> ForcedExit {
        ForcedExit {
            to_chain_id: self.to_chain_id,
            initiator_account_id: self.initiator_account_id,
            initiator_sub_account_id: self.initiator_sub_account_id,
            target_sub_account_id: self.target_sub_account_id,
            target: self.target,
            l2_source_token: self.l2_source_token,
            l1_target_token: self.l1_target_token,
            initiator_nonce: self.initiator_nonce,
            signature: Default::default(),
            ts: self.timestamp,
            exit_amount: self.exit_amount,
            withdraw_to_l1: u8::from(self.withdraw_to_l1),
        }
    }
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

impl FullExitBuilder {
    /// Creates FullExit transaction
    pub fn build(self) -> FullExit {
        FullExit {
            to_chain_id: self.to_chain_id,
            account_id: self.account_id,
            sub_account_id: self.sub_account_id,
            exit_address: self.exit_address,
            l2_source_token: self.l2_source_token,
            l1_target_token: self.l1_target_token,
            serial_id: self.serial_id,
            eth_hash: self.eth_hash,
        }
    }
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

impl OrderMatchingBuilder {
    #[cfg(feature = "ffi")]
    pub fn build(self) -> OrderMatching {
        let taker = (*self.taker).clone();
        let maker = (*self.maker).clone();
        OrderMatching {
            account_id: self.account_id,
            taker,
            maker,
            fee: self.fee,
            fee_token: self.fee_token,
            sub_account_id: self.sub_account_id,
            expect_base_amount: self.expect_base_amount,
            expect_quote_amount: self.expect_quote_amount,
            signature: Default::default(),
        }
    }

    #[cfg(not(feature = "ffi"))]
    pub fn build(self) -> OrderMatching {
        OrderMatching {
            account_id: self.account_id,
            taker: self.taker,
            maker: self.maker,
            fee: self.fee,
            fee_token: self.fee_token,
            sub_account_id: self.sub_account_id,
            expect_base_amount: self.expect_base_amount,
            expect_quote_amount: self.expect_quote_amount,
            signature: Default::default(),
        }
    }
}

pub struct ExitInfoBuilder {
    pub withdrawal_account_id: AccountId,
    pub received_address: ZkLinkAddress,
    pub sub_account_id: SubAccountId,
    pub l2_source_token: TokenId,
    pub l1_target_token: TokenId,
    pub chain_id: ChainId,
}

impl ExitInfoBuilder {
    pub fn build(self) -> ExitInfo {
        ExitInfo {
            withdrawal_account_id: self.withdrawal_account_id,
            received_address: self.received_address,
            sub_account_id: self.sub_account_id,
            l2_source_token: self.l2_source_token,
            l1_target_token: self.l1_target_token,
            chain_id: self.chain_id,
            signature: Default::default(),
        }
    }
}
