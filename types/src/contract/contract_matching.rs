use crate::basic_types::pack::{pack_fee_amount, pack_token_amount};
use crate::basic_types::pad::pad_front;
use crate::basic_types::{AccountId, GetBytes, Nonce, PairId, SlotId, SubAccountId, TokenId};
use crate::params::{
    CONTRACT_BYTES, ORDERS_BYTES, PRICE_BIT_WIDTH, SIGNED_CONTRACT_MATCHING_BIT_WIDTH,
};
use crate::prelude::validator::*;
use crate::tx_type::{format_units, TxTrait, ZkSignatureTrait};
use num::{BigUint, One, Zero};
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_signers::zklink_signer::utils::rescue_hash_orders;
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

/// `ContractMatching` transaction was used to match two contract orders.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractMatching {
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,

    #[validate]
    pub maker: Vec<Contract>,
    #[validate]
    pub taker: Contract,

    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "fee_packable")]
    pub fee: BigUint,
    #[validate(custom = "token_validator")]
    pub fee_token: TokenId,

    pub signature: ZkLinkSignature,
}

impl ContractMatching {
    /// Creates transaction from all the required fields.
    ///
    /// While `signature` field is mandatory for new transactions, it may be `None`
    /// in some cases (e.g. when restoring the network state from the L1 contract data).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_id: AccountId,
        sub_account_id: SubAccountId,
        taker: Contract,
        maker: Vec<Contract>,
        fee: BigUint,
        fee_token: TokenId,
        signature: Option<ZkLinkSignature>,
    ) -> Self {
        Self {
            account_id,
            taker,
            maker,
            sub_account_id,
            fee,
            fee_token,
            signature: signature.unwrap_or_default(),
        }
    }
}

impl GetBytes for ContractMatching {
    fn get_bytes(&self) -> Vec<u8> {
        let mut orders_bytes = Vec::with_capacity(CONTRACT_BYTES * self.maker.len() + 1);
        self.maker
            .iter()
            .for_each(|maker| orders_bytes.extend(maker.get_bytes()));
        orders_bytes.extend(self.taker.get_bytes());
        orders_bytes.resize(ORDERS_BYTES, 0);

        let mut out = Vec::with_capacity(SIGNED_CONTRACT_MATCHING_BIT_WIDTH / 8);
        out.push(Self::TX_TYPE);
        out.extend_from_slice(&self.account_id.to_be_bytes());
        out.extend_from_slice(&self.sub_account_id.to_be_bytes());
        out.extend(rescue_hash_orders(&orders_bytes));
        out.extend_from_slice(&(*self.fee_token as u16).to_be_bytes());
        out.extend_from_slice(&pack_fee_amount(&self.fee));
        out
    }

    fn bytes_len(&self) -> usize {
        SIGNED_CONTRACT_MATCHING_BIT_WIDTH / 8
    }
}

impl TxTrait for ContractMatching {
    fn is_valid(&self) -> bool {
        match self.validate() {
            Ok(_) => self.maker.iter().all(|t| t.is_valid()) && self.taker.is_valid(),
            Err(_) => false,
        }
    }
}

impl ZkSignatureTrait for ContractMatching {
    fn set_signature(&mut self, signature: ZkLinkSignature) {
        self.signature = signature;
    }

    #[cfg(feature = "ffi")]
    fn signature(&self) -> ZkLinkSignature {
        self.signature.clone()
    }

    fn is_signature_valid(&self) -> bool {
        let bytes = self.get_bytes();
        self.signature.verify_musig(&bytes)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    #[validate(custom = "slot_id_validator")]
    pub slot_id: SlotId,
    #[validate(custom = "order_nonce_validator")]
    pub nonce: Nonce,
    /// contract token pair(e.g. btc/usdc)
    #[validate(custom = "pair_validator")]
    pub pair_id: PairId,
    /// Position size for open positions
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "amount_packable")]
    pub size: BigUint,
    /// Price of open positions
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "price_validator")]
    pub price: BigUint,
    /// 0 -> short, 1 -> long
    #[validate(custom = "direction_validator")]
    pub direction: u8,
    /// index 0 => maker_fee, index 1 => taker_fee, 100 means 1%, max is 2.56%
    pub fee_rates: [u8; 2],
    /// Subsidy only works for maker and fee_rates[0]
    #[validate(custom = "boolean_validator")]
    pub has_subsidy: u8,
    pub signature: ZkLinkSignature,
}

impl GetBytes for Contract {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut out = Vec::with_capacity(bytes_len);
        out.push(Self::MSG_TYPE);
        out.extend(self.account_id.to_be_bytes());
        out.push(*self.sub_account_id);
        out.extend((*self.slot_id as u16).to_be_bytes());
        out.extend_from_slice(&self.nonce.to_be_bytes()[1..]);
        out.push(*self.pair_id as u8);
        out.push(self.direction);
        out.extend(pack_token_amount(&self.size));
        out.extend(pad_front(&self.price.to_bytes_be(), PRICE_BIT_WIDTH / 8));
        out.extend(self.fee_rates);
        out.push(self.has_subsidy);
        assert_eq!(out.len(), bytes_len);
        out
    }

    fn bytes_len(&self) -> usize {
        CONTRACT_BYTES
    }
}

impl TxTrait for Contract {}

impl Contract {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_id: AccountId,
        sub_account_id: SubAccountId,
        slot_id: SlotId,
        nonce: Nonce,
        pair_id: PairId,
        size: BigUint,
        price: BigUint,
        direction: bool,
        fee_rates: [u8; 2],
        has_subsidy: bool,
        signature: Option<ZkLinkSignature>,
    ) -> Self {
        Self {
            account_id,
            sub_account_id,
            slot_id,
            nonce,
            pair_id,
            size,
            price,
            direction: direction as u8,
            fee_rates,
            has_subsidy: has_subsidy as u8,
            signature: signature.unwrap_or_default(),
        }
    }

    pub fn is_long(&self) -> bool {
        self.direction == 1
    }

    pub fn is_short(&self) -> bool {
        self.direction == 0
    }

    pub fn get_ethereum_sign_message(&self, symbol: &str, decimals: u8) -> String {
        let mut message = if self.size.is_zero() {
            format!("Contract order for {}\n", symbol)
        } else {
            format!(
                "Contract order for {} {}\n",
                format_units(&self.size, decimals),
                symbol,
            )
        };
        message += format!(
            "Direction: {Direction}\n\
            Price: {price}\n
            Nonce: {nonce}\n\
            Fee rate: {maker_fee_rate},{taker_fee_rate}\n",
            Direction = if self.direction.is_one() {
                "Long"
            } else {
                "Short"
            },
            price = self.price,
            nonce = self.nonce,
            maker_fee_rate = self.fee_rates[0],
            taker_fee_rate = self.fee_rates[1],
        )
        .as_str();
        message
    }
}

impl ZkSignatureTrait for Contract {
    fn set_signature(&mut self, signature: ZkLinkSignature) {
        self.signature = signature;
    }

    #[cfg(feature = "ffi")]
    fn signature(&self) -> ZkLinkSignature {
        self.signature.clone()
    }
    fn is_signature_valid(&self) -> bool {
        let bytes = self.get_bytes();
        self.signature.verify_musig(&bytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contract_matching_get_bytes() {
        let tx = ContractMatching::default();
        let bytes = tx.get_bytes();
        assert_eq!(bytes.len(), tx.bytes_len());
    }
}
