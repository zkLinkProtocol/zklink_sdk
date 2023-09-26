use crate::basic_types::pack::{pack_fee_amount, pack_token_amount};
use crate::basic_types::params::{
    ORDERS_BYTES, PRICE_BIT_WIDTH, SIGNED_ORDER_BIT_WIDTH, SIGNED_ORDER_MATCHING_BIT_WIDTH,
    TOKEN_MAX_PRECISION,
};
use crate::basic_types::{AccountId, Nonce, SlotId, SubAccountId, TokenId};
use crate::signatures::TxLayer1Signature;
use crate::tx_builder::OrderMatchingBuilder;
use crate::tx_type::validator::*;
use crate::tx_type::{format_units, TxTrait, ZkSignatureTrait};
use num::{BigUint, One, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ffi")]
use std::sync::Arc;
use validator::Validate;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
#[cfg(feature = "ffi")]
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_signers::zklink_signer::utils::rescue_hash_orders;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    #[validate(custom = "slot_id_validator")]
    pub slot_id: SlotId,
    #[validate(custom = "nonce_validator")]
    pub nonce: Nonce,
    #[validate(custom = "token_validator")]
    pub base_token_id: TokenId, // btc
    #[validate(custom = "token_validator")]
    pub quote_token_id: TokenId, // usdt
    /// The amount of base token buy or sell
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "amount_packable")]
    pub amount: BigUint,

    /// How much a quote token, accuracy will be improved
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "price_validator")]
    pub price: BigUint,
    /// Order type, 0: buy, 1: sell
    #[validate(custom = "boolean_validator")]
    pub is_sell: u8,
    /// Fee as maker, 100 means 1%, max is 2.56 %
    pub fee_ratio1: u8,
    /// Fee as taker
    pub fee_ratio2: u8,
    pub signature: ZkLinkSignature,
}

impl Order {
    pub const MSG_TYPE: u8 = 0xff;

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_id: AccountId,
        sub_account_id: SubAccountId,
        slot_id: SlotId,
        nonce: Nonce,
        base_token_id: TokenId,
        quote_token_id: TokenId,
        amount: BigUint,
        price: BigUint,
        is_sell: bool,
        fee_ratio1: u8,
        fee_ratio2: u8,
    ) -> Self {
        Self {
            account_id,
            sub_account_id,
            slot_id,
            nonce,
            base_token_id,
            quote_token_id,
            amount,
            price,
            is_sell: u8::from(is_sell),
            fee_ratio1,
            fee_ratio2,
            signature: ZkLinkSignature::default(),
        }
    }

    pub fn get_eth_sign_msg(&self, quote_token: &str, based_token: &str, decimals: u8) -> String {
        let mut message = if self.amount.is_zero() {
            format!("Limit order for {} -> {}\n", quote_token, based_token)
        } else {
            format!(
                "Order for {} {} -> {}\n",
                format_units(&self.amount, decimals),
                quote_token,
                based_token
            )
        };
        message += format!(
            "price: {price}\n\
            Nonce: {nonce}",
            price = self.price,
            nonce = self.nonce
        )
        .as_str();
        message
    }
}

impl TxTrait for Order {
    fn get_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(SIGNED_ORDER_BIT_WIDTH / 8);
        out.extend_from_slice(&[Self::MSG_TYPE]);
        out.extend_from_slice(&self.account_id.to_be_bytes());
        out.extend_from_slice(&self.sub_account_id.to_be_bytes());
        out.extend_from_slice(&(*self.slot_id as u16).to_be_bytes());
        out.extend_from_slice(&self.nonce.to_be_bytes()[1..]);
        out.extend_from_slice(&(*self.base_token_id as u16).to_be_bytes());
        out.extend_from_slice(&(*self.quote_token_id as u16).to_be_bytes());
        out.extend_from_slice(&pad_front(&self.price.to_bytes_be(), PRICE_BIT_WIDTH / 8));
        out.extend_from_slice(&self.is_sell.to_be_bytes());
        out.extend_from_slice(&self.fee_ratio1.to_be_bytes());
        out.extend_from_slice(&self.fee_ratio2.to_be_bytes());
        out.extend_from_slice(&pack_token_amount(&self.amount));
        out
    }
}

impl ZkSignatureTrait for Order {
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

    #[cfg(feature = "ffi")]
    fn submitter_signature(
        &self,
        _signer: Arc<ZkLinkSigner>,
    ) -> Result<ZkLinkSignature, ZkSignerError> {
        unreachable!("no need submitter signature for Order")
    }
}

/// `OrderMatching` transaction was used to match two orders.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OrderMatching {
    /// zklink network account ID of the transaction initiator.
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    /// all content of Taker and Maker orders
    #[validate]
    pub taker: Order,
    #[validate]
    pub maker: Order,

    /// Fee for the transaction, need packaging
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "fee_packable")]
    pub fee: BigUint,
    #[validate(custom = "token_validator")]
    pub fee_token: TokenId,

    /// The maximum base(quote) token amount that tx submitter expects to trade
    /// These two value will be smaller than the maximum amount can be traded between maker and taker
    /// The zero value will not affect the actual amount of the order
    /// example: BTC/USD orderbook of dex:
    /// sell (price, amount)
    /// 10000, 4
    /// 8000, 2
    /// buy (price, amount)
    /// 7000, 3
    /// when a user buy 3 BTC for price 10000, dex will submit two OrderMathcing
    /// maker: 8000, 2 <-> taker: 10000, 3
    /// maker: 10000, 4 <-> taker: 10000, 3
    /// if all is well, all OrderMathcing will be executed in sequence
    /// but when the balance of maker (8000, 2) is not enough, the first OrderMathcing will be failed
    /// and the second OrderMathcing will be still success, the second maker (10000, 4) will be trade for 3 BTC
    /// but the result may be not dex want to see
    /// dex can set `expect_base_amount` and `expect_quote_amount` to limit the maximum trade amount
    /// maker: 8000, 2, <-> taker: 10000, 3 <-> expect_base_amount 2 => the maximum of BTC traded will be 2
    /// maker: 10000, 4<-> taker: 10000, 3 <-> expect_base_amount 1 => the maximum of BTC traded will be 1
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    /// why not pack expect_base_amount and expect_quote_amount?
    /// for example:
    /// maker: 8000, m1 <-> taker: 10000, t1 <-> expect_base_amount t1, (m1 <= t1)
    /// maker: 10000, m2 <-> taker: taker: 10000, t2 <-> expect_base_amount t2 - t1, (t2 <= m2)
    /// t1 and t2 both packable, but (t2 - t1) may not be packable
    #[validate(custom = "amount_unpackable")]
    pub expect_base_amount: BigUint,
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "amount_unpackable")]
    pub expect_quote_amount: BigUint,

    /// Time range when the transaction is valid(layer2).
    #[serde(default)]
    pub signature: ZkLinkSignature,
}
impl OrderMatching {
    /// Creates transaction from all the required fields.
    #[cfg(feature = "ffi")]
    pub fn new(builder: OrderMatchingBuilder) -> Self {
        Self {
            account_id: builder.account_id,
            taker: (*builder.taker).clone(),
            maker: (*builder.maker).clone(),
            fee: builder.fee,
            fee_token: builder.fee_token,
            sub_account_id: builder.sub_account_id,
            expect_base_amount: builder.expect_base_amount,
            expect_quote_amount: builder.expect_quote_amount,
            signature: ZkLinkSignature::default(),
        }
    }

    #[cfg(not(feature = "ffi"))]
    pub fn new(builder: OrderMatchingBuilder) -> Self {
        Self {
            account_id: builder.account_id,
            taker: builder.taker,
            maker: builder.maker,
            fee: builder.fee,
            fee_token: builder.fee_token,
            sub_account_id: builder.sub_account_id,
            expect_base_amount: builder.expect_base_amount,
            expect_quote_amount: builder.expect_quote_amount,
            signature: ZkLinkSignature::default(),
        }
    }

    pub fn get_eth_sign_msg(&self) -> String {
        format!(
            "OrderMatching fee: {} {}\n",
            format_units(&self.fee, TOKEN_MAX_PRECISION),
            self.fee_token
        )
    }

    #[cfg(feature = "ffi")]
    pub fn eth_signature(
        &self,
        eth_signer: Arc<EthSigner>,
    ) -> Result<TxLayer1Signature, ZkSignerError> {
        let msg = self.get_eth_sign_msg();
        let eth_signature = eth_signer.sign_message(msg.as_bytes())?;
        let tx_eth_signature = TxLayer1Signature::EthereumSignature(eth_signature);
        Ok(tx_eth_signature)
    }

    #[cfg(not(feature = "ffi"))]
    pub fn eth_signature(
        &self,
        eth_signer: &EthSigner,
    ) -> Result<TxLayer1Signature, ZkSignerError> {
        let msg = self.get_eth_sign_msg();
        let eth_signature = eth_signer.sign_message(msg.as_bytes())?;
        let tx_eth_signature = TxLayer1Signature::EthereumSignature(eth_signature);
        Ok(tx_eth_signature)
    }

    /// Returns the expectant exchange amount of maker.
    pub fn maker_expect_amount(&self) -> &BigUint {
        if self.maker.is_sell.is_one() {
            &self.expect_base_amount
        } else {
            &self.expect_quote_amount
        }
    }

    /// Returns the expectant exchange amount of taker.
    pub fn taker_expect_amount(&self) -> &BigUint {
        if self.taker.is_sell.is_one() {
            &self.expect_base_amount
        } else {
            &self.expect_quote_amount
        }
    }

    /// Returns the expectant exchange amounts of maker and taker.
    pub fn expectant_amounts(&self) -> (&BigUint, &BigUint) {
        (self.maker_expect_amount(), self.taker_expect_amount())
    }

    /// The expect_mode is to trade according to expect_base_amount and expect_quote_amount
    /// Not expect_mode, will trade according to the maximum value of the two orders that can be filled.
    pub fn is_expect_mode(&self) -> bool {
        !self.expect_base_amount.is_zero() && !self.expect_quote_amount.is_zero()
    }
}

impl TxTrait for OrderMatching {
    fn get_bytes(&self) -> Vec<u8> {
        let maker_order_bytes = self.maker.get_bytes();
        let mut orders_bytes = Vec::with_capacity(maker_order_bytes.len() * 2);
        orders_bytes.extend(maker_order_bytes);
        orders_bytes.extend(self.taker.get_bytes());
        // todo do not resize, sdk should be update
        orders_bytes.resize(ORDERS_BYTES, 0);

        let mut out = Vec::with_capacity(SIGNED_ORDER_MATCHING_BIT_WIDTH / 8);
        out.push(Self::TX_TYPE);
        out.extend_from_slice(&self.account_id.to_be_bytes());
        out.extend_from_slice(&self.sub_account_id.to_be_bytes());
        out.extend(rescue_hash_orders(&orders_bytes));
        out.extend_from_slice(&(*self.fee_token as u16).to_be_bytes());
        out.extend_from_slice(&pack_fee_amount(&self.fee));
        out.extend_from_slice(&self.expect_base_amount.to_u128().unwrap().to_be_bytes());
        out.extend_from_slice(&self.expect_quote_amount.to_u128().unwrap().to_be_bytes());
        out
    }

    fn is_valid(&self) -> bool {
        let order_valid = match self.validate() {
            Ok(_) => self.maker.is_valid() && self.taker.is_valid(),
            Err(_) => false,
        };
        order_valid && self.validate().is_ok()
    }
}

impl ZkSignatureTrait for OrderMatching {
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

fn pad_front(bytes: &[u8], size: usize) -> Vec<u8> {
    assert!(size >= bytes.len());
    let mut result = vec![0u8; size];
    result[size - bytes.len()..].copy_from_slice(bytes);
    result
}
