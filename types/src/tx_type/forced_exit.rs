use num::{BigUint, ToPrimitive};
use validator::Validate;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

use crate::basic_types::{
    AccountId, ChainId, GetBytes, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::params::{SIGNED_FORCED_EXIT_BIT_WIDTH, TX_TYPE_BIT_WIDTH};
#[cfg(feature = "ffi")]
use crate::prelude::ForcedExitBuilder;
use crate::tx_type::validator::*;
use crate::tx_type::{TxTrait, ZkSignatureTrait};
use serde::{Deserialize, Serialize};
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;

/// `ForcedExit` transaction is used to withdraw funds from an unowned
/// account to its corresponding L1 address.
///
/// Caller of this function will pay fee for the operation, and has no
/// control over the address on which funds will be withdrawn. Account
/// to which `ForcedExit` is applied must have no public key hash set.
///
/// This operation is expected to be used in cases when account in L1
/// cannot prove its identity in L2 (e.g. it's an existing smart contract),
/// so the funds won't get "locked" in L2.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ForcedExit {
    /// The chain ID of receiver of the transaction.
    #[validate(custom = "chain_id_validator")]
    pub to_chain_id: ChainId,
    /// zkLink network account ID of the transaction initiator.
    #[validate(custom = "account_validator")]
    pub initiator_account_id: AccountId,
    /// sub-account ID of initiator fee token.
    #[validate(custom = "sub_account_validator")]
    pub initiator_sub_account_id: SubAccountId,
    /// Current initiator account nonce.
    #[validate(custom = "nonce_validator")]
    pub initiator_nonce: Nonce,
    /// Layer1 address of the account to withdraw funds from.
    /// Also this field represents the address in L1 to which funds will be withdrawn.
    #[validate(custom = "zklink_address_validator")]
    pub target: ZkLinkAddress,
    /// Source sub-account ID of the transaction withdraw.
    #[validate(custom = "sub_account_validator")]
    pub target_sub_account_id: SubAccountId,
    /// Source token and target token of ForcedExit from l2 to l1.
    /// Also represents the token in which fee will be paid.
    #[validate(custom = "token_validator")]
    pub l2_source_token: TokenId,
    #[validate(custom = "token_validator")]
    pub l1_target_token: TokenId,
    /// Amount of funds to exit, layer1 can not unpack it, do not packaging
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "amount_unpackable")]
    pub exit_amount: BigUint,
    /// whether withdraw to layer1.
    #[validate(custom = "boolean_validator")]
    pub withdraw_to_l1: u8,
    /// Transaction zkLink signature.
    #[serde(default)]
    pub signature: ZkLinkSignature,

    /// Used as request id
    pub ts: TimeStamp,
}

impl ForcedExit {
    #[cfg(feature = "ffi")]
    pub fn new(builder: ForcedExitBuilder) -> Self {
        builder.build()
    }
}

impl GetBytes for ForcedExit {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut out = Vec::with_capacity(bytes_len);
        out.extend_from_slice(&[Self::TX_TYPE]);
        out.extend_from_slice(&self.to_chain_id.to_be_bytes());
        out.extend_from_slice(&self.initiator_account_id.to_be_bytes());
        out.extend_from_slice(&self.initiator_sub_account_id.to_be_bytes());
        out.extend_from_slice(&self.target.to_fixed_bytes());
        out.extend_from_slice(&self.target_sub_account_id.to_be_bytes());
        out.extend_from_slice(&(*self.l2_source_token as u16).to_be_bytes());
        out.extend_from_slice(&(*self.l1_target_token as u16).to_be_bytes());
        out.extend_from_slice(&self.initiator_nonce.to_be_bytes());
        out.extend_from_slice(&self.exit_amount.to_u128().unwrap().to_be_bytes());
        out.push(self.withdraw_to_l1);
        out.extend_from_slice(&self.ts.to_be_bytes());
        assert_eq!(out.len(), bytes_len);
        out
    }

    fn bytes_len(&self) -> usize {
        SIGNED_FORCED_EXIT_BIT_WIDTH / TX_TYPE_BIT_WIDTH
    }
}

impl TxTrait for ForcedExit {}
impl ZkSignatureTrait for ForcedExit {
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
    use crate::prelude::ForcedExitBuilder;
    use std::str::FromStr;

    #[test]
    fn test_get_bytes() {
        let address =
            ZkLinkAddress::from_str("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9").unwrap();
        let ts = 1693472232u32;
        let builder = ForcedExitBuilder {
            to_chain_id: ChainId(1),
            initiator_account_id: AccountId(10),
            initiator_sub_account_id: SubAccountId(1),
            target: address,
            target_sub_account_id: SubAccountId(1),
            l2_source_token: TokenId(18),
            l1_target_token: TokenId(18),
            initiator_nonce: Nonce(1),
            exit_amount: BigUint::from(10000u32),
            withdraw_to_l1: false,
            timestamp: ts.into(),
        };
        let forced_exit = builder.build();
        let bytes = forced_exit.get_bytes();
        let excepted_bytes = [
            7, 1, 0, 0, 0, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 175, 175, 243, 173, 26, 4,
            37, 215, 146, 67, 45, 158, 205, 28, 62, 38, 239, 44, 66, 233, 1, 0, 18, 0, 18, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 16, 0, 100, 240, 85, 232,
        ];

        assert_eq!(bytes, excepted_bytes);
    }
}
