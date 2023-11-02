use crate::basic_types::pack::pack_fee_amount;
use crate::basic_types::{AccountId, GetBytes, Nonce, SubAccountId, TokenId};
use crate::contract::prices::OraclePrices;
use crate::params::SIGNED_LIQUIDATION_BIT_WIDTH;
use crate::prelude::validator::*;
#[cfg(feature = "ffi")]
use crate::tx_builder::LiquidationBuilder;
use crate::tx_type::{TxTrait, ZkSignatureTrait};
use num::BigUint;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

/// `Liquidation` transaction was used to liquidation some burst users.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Liquidation {
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    #[validate(custom = "nonce_validator")]
    pub sub_account_nonce: Nonce,

    /// Contains required mark prices for all margin tokens and contracts from Oracle(e.g. ChainLink, Api3)
    #[validate]
    pub oracle_prices: OraclePrices,

    /// The account that are required to liquidation their positions
    #[validate(custom = "account_validator")]
    pub liquidation_account_id: AccountId,

    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "fee_packable")]
    pub fee: BigUint,
    #[validate(custom = "token_validator")]
    pub fee_token: TokenId,

    pub signature: ZkLinkSignature,
}

impl Liquidation {
    /// Creates transaction from all the required fields.
    #[cfg(feature = "ffi")]
    pub fn new(builder: LiquidationBuilder) -> Self {
        builder.build()
    }
}

impl TxTrait for Liquidation {}

impl GetBytes for Liquidation {
    /// Encodes the transaction data as the byte sequence.
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut out = Vec::with_capacity(bytes_len);
        out.push(Self::TX_TYPE);
        out.extend(self.account_id.to_be_bytes());
        out.extend(self.sub_account_id.to_be_bytes());
        out.extend(self.sub_account_nonce.to_be_bytes());
        out.extend(self.oracle_prices.rescue_hash());
        out.extend(self.liquidation_account_id.to_be_bytes());
        out.extend((*self.fee_token as u16).to_be_bytes());
        out.extend(pack_fee_amount(&self.fee));
        assert_eq!(out.len(), bytes_len);
        out
    }

    fn bytes_len(&self) -> usize {
        SIGNED_LIQUIDATION_BIT_WIDTH / 8
    }
}

impl ZkSignatureTrait for Liquidation {
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
