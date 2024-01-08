use super::prices::OraclePrices;
use crate::basic_types::pack::{pack_fee_amount, pack_token_amount};
use crate::basic_types::pad::pad_front;
use crate::basic_types::{AccountId, GetBytes, Nonce, PairId, SubAccountId, TokenId};
use crate::params::{PRICE_BIT_WIDTH, SIGNED_AUTO_DELEVERAGING_BIT_WIDTH};
use crate::prelude::validator::*;
#[cfg(feature = "ffi")]
use crate::tx_builder::AutoDeleveragingBuilder;
use crate::tx_type::{TxTrait, ZkSignatureTrait};
use num::BigUint;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

/// `AutoDeleveraging` transaction was used to auto deleveraging some account.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AutoDeleveraging {
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    #[validate(custom = "nonce_validator")]
    pub sub_account_nonce: Nonce,
    /// Contains required mark prices for all margin tokens and contracts from Oracle(e.g. ChainLink, Band, Api3)
    #[validate]
    pub oracle_prices: OraclePrices,
    /// The account that are required to adl their position
    #[validate(custom = "account_validator")]
    pub adl_account_id: AccountId,
    /// contract token pair of the specified adl position(e.g. btc/usdc)
    #[validate(custom = "pair_validator")]
    pub pair_id: PairId,
    /// size of adl position
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "adl_size_unpackable")]
    pub adl_size: BigUint,
    /// Price of adl position
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "price_validator")]
    pub adl_price: BigUint,
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "fee_packable")]
    pub fee: BigUint,
    #[validate(custom = "token_validator")]
    pub fee_token: TokenId,
    pub signature: ZkLinkSignature,
}

impl AutoDeleveraging {
    #[cfg(feature = "ffi")]
    pub fn new(builder: AutoDeleveragingBuilder) -> Self {
        builder.build()
    }
}

impl TxTrait for AutoDeleveraging {}

impl GetBytes for AutoDeleveraging {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut out = Vec::with_capacity(bytes_len);
        out.push(Self::TX_TYPE);
        out.extend(self.account_id.to_be_bytes());
        out.push(*self.sub_account_id);
        out.extend(self.sub_account_nonce.to_be_bytes());
        out.extend(self.oracle_prices.rescue_hash());
        out.extend(self.adl_account_id.to_be_bytes());
        out.push(*self.pair_id as u8);
        out.extend(pack_token_amount(&self.adl_size));
        out.extend(pad_front(
            &self.adl_price.to_bytes_be(),
            PRICE_BIT_WIDTH / 8,
        ));
        out.extend((*self.fee_token as u16).to_be_bytes());
        out.extend(pack_fee_amount(&self.fee));
        assert_eq!(out.len(), bytes_len);
        out
    }

    fn bytes_len(&self) -> usize {
        SIGNED_AUTO_DELEVERAGING_BIT_WIDTH / 8
    }
}

impl ZkSignatureTrait for AutoDeleveraging {
    fn set_signature(&mut self, signature: ZkLinkSignature) {
        self.signature = signature;
    }

    fn signature(&self) -> &ZkLinkSignature {
        &self.signature
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_auto_deleveraging_get_bytes() {
        // TODO: do not use default, because oracle_prices should has some values
        let tx = AutoDeleveraging::default();
        let bytes = tx.get_bytes();
        assert_eq!(bytes.len(), tx.bytes_len());
    }
}
