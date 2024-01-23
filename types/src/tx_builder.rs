use crate::basic_types::{
    AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::prelude::{
    AutoDeleveraging, ChangePubKey, ChangePubKeyAuthData, Contract, ContractMatching, Deposit,
    ForcedExit, FullExit, Funding, Liquidation, OraclePrices, Order, OrderMatching, PairId,
    Parameter, SlotId, Transfer, UpdateGlobalVar, Withdraw,
};
use crate::tx_type::contract::prices::{ContractPrice, SpotPriceInfo};
use crate::tx_type::exit_info::ExitInfo;
use cfg_if::cfg_if;
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
    pub l2_hash: H256,
    pub eth_hash: Option<H256>,
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
            l2_hash: self.l2_hash,
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
    pub call_data: Option<Vec<u8>>,
    pub fee: BigUint,
    pub nonce: Nonce,
    pub withdraw_to_l1: bool,
    pub withdraw_fee_ratio: u16,
    pub timestamp: TimeStamp,
}

impl WithdrawBuilder {
    /// Creates Withdraw transaction
    pub fn build(self) -> Withdraw {
        let withdraw_to_l1 = u8::from(self.withdraw_to_l1);

        Withdraw {
            to_chain_id: self.to_chain_id,
            account_id: self.account_id,
            sub_account_id: self.sub_account_id,
            to: self.to_address,
            l2_source_token: self.l2_source_token,
            l1_target_token: self.l1_target_token,
            amount: self.amount,
            call_data: self.call_data,
            fee: self.fee,
            nonce: self.nonce,
            signature: Default::default(),
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
    pub contract_prices: Vec<ContractPrice>,
    pub margin_prices: Vec<SpotPriceInfo>,
    pub serial_id: u64,
    pub l2_hash: H256,
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
            oracle_prices: OraclePrices {
                contract_prices: self.contract_prices,
                margin_prices: self.margin_prices,
            },
            serial_id: self.serial_id,
            l2_hash: self.l2_hash,
        }
    }
}

pub struct OrderMatchingBuilder {
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub contract_prices: Vec<ContractPrice>,
    pub margin_prices: Vec<SpotPriceInfo>,
    #[cfg(feature = "ffi")]
    pub taker: Arc<Order>,
    #[cfg(feature = "ffi")]
    pub maker: Arc<Order>,
    #[cfg(not(feature = "ffi"))]
    pub taker: Order,
    #[cfg(not(feature = "ffi"))]
    pub maker: Order,
    pub fee: BigUint,
    pub fee_token: TokenId,
    pub expect_base_amount: BigUint,
    pub expect_quote_amount: BigUint,
}

impl OrderMatchingBuilder {
    pub fn build(self) -> OrderMatching {
        cfg_if! {
            if #[cfg(feature = "ffi")] {
                let taker = (*self.taker).clone();
                let maker = (*self.maker).clone();
            } else {
                let taker = self.taker;
                let maker = self.maker;
            }
        }
        OrderMatching {
            account_id: self.account_id,
            taker,
            maker,
            oracle_prices: OraclePrices {
                contract_prices: self.contract_prices,
                margin_prices: self.margin_prices,
            },
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

pub struct AutoDeleveragingBuilder {
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub sub_account_nonce: Nonce,
    pub contract_prices: Vec<ContractPrice>,
    pub margin_prices: Vec<SpotPriceInfo>,
    pub adl_account_id: AccountId,
    pub pair_id: PairId,
    pub adl_size: BigUint,
    pub adl_price: BigUint,
    pub fee: BigUint,
    pub fee_token: TokenId,
}

impl AutoDeleveragingBuilder {
    pub fn build(self) -> AutoDeleveraging {
        AutoDeleveraging {
            account_id: self.account_id,
            sub_account_id: self.sub_account_id,
            sub_account_nonce: self.sub_account_nonce,
            oracle_prices: OraclePrices {
                contract_prices: self.contract_prices,
                margin_prices: self.margin_prices,
            },
            adl_account_id: self.adl_account_id,
            pair_id: self.pair_id,
            adl_size: self.adl_size,
            adl_price: self.adl_price,
            fee: self.fee,
            fee_token: self.fee_token,
            signature: Default::default(),
        }
    }
}

pub struct ContractMatchingBuilder {
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    #[cfg(feature = "ffi")]
    pub taker: Arc<Contract>,
    #[cfg(feature = "ffi")]
    pub maker: Vec<Arc<Contract>>,
    #[cfg(not(feature = "ffi"))]
    pub taker: Contract,
    #[cfg(not(feature = "ffi"))]
    pub maker: Vec<Contract>,
    pub fee: BigUint,
    pub fee_token: TokenId,
    pub contract_prices: Vec<ContractPrice>,
    pub margin_prices: Vec<SpotPriceInfo>,
}

impl ContractMatchingBuilder {
    pub fn build(self) -> ContractMatching {
        cfg_if! {
            if #[cfg(feature = "ffi")] {
                let maker = self.maker.into_iter()
                    .map(|p| (*p).clone())
                    .collect();
                let taker = (*self.taker).clone();
            } else {
                let maker = self.maker;
                let taker = self.taker;
            }
        }
        ContractMatching {
            account_id: self.account_id,
            taker,
            maker,
            sub_account_id: self.sub_account_id,
            fee: self.fee,
            fee_token: self.fee_token,
            oracle_prices: OraclePrices {
                contract_prices: self.contract_prices,
                margin_prices: self.margin_prices,
            },
            signature: Default::default(),
        }
    }
}

pub struct ContractBuilder {
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub slot_id: SlotId,
    pub nonce: Nonce,
    pub pair_id: PairId,
    pub size: BigUint,
    pub price: BigUint,
    pub direction: bool,
    /// 100 means 1%, max is 2.56%
    pub maker_fee_rate: u8,
    /// 100 means 1%, max is 2.56%
    pub taker_fee_rate: u8,
    pub has_subsidy: bool,
}

impl ContractBuilder {
    pub fn build(self) -> Contract {
        Contract {
            account_id: self.account_id,
            sub_account_id: self.sub_account_id,
            slot_id: self.slot_id,
            nonce: self.nonce,
            pair_id: self.pair_id,
            size: self.size,
            price: self.price,
            direction: self.direction as u8,
            fee_rates: [self.maker_fee_rate, self.taker_fee_rate],
            has_subsidy: self.has_subsidy as u8,
            signature: Default::default(),
        }
    }
}

pub struct FundingBuilder {
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub sub_account_nonce: Nonce,
    pub funding_account_ids: Vec<AccountId>,
    pub fee: BigUint,
    pub fee_token: TokenId,
}

impl FundingBuilder {
    pub fn build(self) -> Funding {
        Funding {
            account_id: self.account_id,
            sub_account_id: self.sub_account_id,
            sub_account_nonce: self.sub_account_nonce,
            funding_account_ids: self.funding_account_ids,
            fee: self.fee,
            fee_token: self.fee_token,
            signature: Default::default(),
        }
    }
}

pub struct LiquidationBuilder {
    pub account_id: AccountId,
    pub sub_account_id: SubAccountId,
    pub sub_account_nonce: Nonce,
    pub contract_prices: Vec<ContractPrice>,
    pub margin_prices: Vec<SpotPriceInfo>,
    pub liquidation_account_id: AccountId,
    pub fee: BigUint,
    pub fee_token: TokenId,
}

impl LiquidationBuilder {
    pub fn build(self) -> Liquidation {
        Liquidation {
            account_id: self.account_id,
            sub_account_id: self.sub_account_id,
            sub_account_nonce: self.sub_account_nonce,
            oracle_prices: OraclePrices {
                contract_prices: self.contract_prices,
                margin_prices: self.margin_prices,
            },
            liquidation_account_id: self.liquidation_account_id,
            fee: self.fee,
            fee_token: self.fee_token,
            signature: Default::default(),
        }
    }
}

pub struct UpdateGlobalVarBuilder {
    pub from_chain_id: ChainId,
    pub sub_account_id: SubAccountId,
    #[cfg(feature = "ffi")]
    pub parameter: Parameter,
    #[cfg(not(feature = "ffi"))]
    pub parameter: Parameter,
    pub serial_id: u64,
}

impl UpdateGlobalVarBuilder {
    pub fn build(self) -> UpdateGlobalVar {
        let parameter = self.parameter;
        UpdateGlobalVar {
            from_chain_id: self.from_chain_id,
            sub_account_id: self.sub_account_id,
            parameter,
            serial_id: self.serial_id,
        }
    }
}
