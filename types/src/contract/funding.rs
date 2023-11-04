use crate::basic_types::pack::pack_fee_amount;
use crate::basic_types::{AccountId, GetBytes, Nonce, PairId, SubAccountId, TokenId};
use crate::params::{
    FUNDING_RATE_BYTES, PAIR_BIT_WIDTH, SIGNED_BATCH_FUNDING_BIT_WIDTH, SIGNED_FUNDING_BIT_WIDTH,
};
use crate::prelude::validator::*;
#[cfg(feature = "ffi")]
use crate::tx_builder::FundingBuilder;
use crate::tx_type::{TxTrait, ZkSignatureTrait};
use num::BigUint;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    #[validate(custom = "pair_validator")]
    pub pair_id: PairId,
    // TODO: can it be lower than 0?
    pub funding_rate: i16,
}

impl GetBytes for FundingRate {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut funding_rate_encode = Vec::with_capacity(bytes_len);
        funding_rate_encode.push(*self.pair_id as u8);
        // For the convenience of the circuit, we use the true code instead of the original complement.
        let mut rate_bytes = self.funding_rate.unsigned_abs().to_be_bytes();
        if self.funding_rate.is_negative() {
            rate_bytes[0] |= 0b1000_0000
        };
        funding_rate_encode.extend(rate_bytes);
        assert_eq!(funding_rate_encode.len(), bytes_len);
        funding_rate_encode
    }

    fn bytes_len(&self) -> usize {
        PAIR_BIT_WIDTH / 8 + FUNDING_RATE_BYTES
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Funding {
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    #[validate(custom = "nonce_validator")]
    pub sub_account_nonce: Nonce,
    pub funding_account_ids: Vec<AccountId>,
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "fee_packable")]
    pub fee: BigUint,
    #[validate(custom = "token_validator")]
    pub fee_token: TokenId,
    pub signature: ZkLinkSignature,
}

impl Funding {
    #[cfg(feature = "ffi")]
    pub fn new(builder: FundingBuilder) -> Self {
        builder.build()
    }

    pub fn is_batch_funding(&self) -> bool {
        !self.is_single_funding()
    }

    pub fn is_single_funding(&self) -> bool {
        self.funding_account_ids.len() == 1
    }

    pub fn get_updated_account_ids(&self) -> Vec<AccountId> {
        let mut account_ids = self.funding_account_ids.clone();
        account_ids.push(self.account_id);
        account_ids
    }
}

impl TxTrait for Funding {}

impl GetBytes for Funding {
    fn get_bytes(&self) -> Vec<u8> {
        let funding_account_ids_bytes = if self.is_batch_funding() {
            self.funding_account_ids.rescue_hash()
        } else {
            self.funding_account_ids.get_bytes()
        };
        let bytes_len = self.bytes_len();
        let mut out = Vec::with_capacity(bytes_len);
        out.push(Self::TX_TYPE);
        out.extend(self.account_id.to_be_bytes());
        out.push(*self.sub_account_id);
        out.extend(self.sub_account_nonce.to_be_bytes());
        out.extend(funding_account_ids_bytes);
        out.extend((*self.fee_token as u16).to_be_bytes());
        out.extend(pack_fee_amount(&self.fee));
        assert_eq!(out.len(), bytes_len);
        out
    }

    fn bytes_len(&self) -> usize {
        if self.is_batch_funding() {
            SIGNED_BATCH_FUNDING_BIT_WIDTH / 8
        } else {
            SIGNED_FUNDING_BIT_WIDTH / 8
        }
    }
}

impl ZkSignatureTrait for Funding {
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
