use num::{BigUint, ToPrimitive};
use validator::Validate;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

use crate::basic_types::{
    AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::tx_type::validator::*;
use serde::{Deserialize, Serialize};
use zklink_signers::zklink_signer::error::ZkSignerError;
#[cfg(not(feature = "ffi"))]
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_signers::zklink_signer::signature::ZkLinkSignature;
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
    /// Transaction zkLink signature.
    #[serde(default)]
    pub signature: ZkLinkSignature,

    /// Used as request id
    pub ts: TimeStamp,
}

impl ForcedExit {
    /// Creates transaction from all the required fields.
    ///
    /// While `signature` field is mandatory for new transactions, it may be `None`
    /// in some cases (e.g. when restoring the network state from the L1 contract data).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        to_chain_id: ChainId,
        initiator_account_id: AccountId,
        initiator_sub_account_id: SubAccountId,
        target: ZkLinkAddress,
        target_sub_account_id: SubAccountId,
        l2_source_token: TokenId,
        l1_target_token: TokenId,
        nonce: Nonce,
        exit_amount: BigUint,
        ts: TimeStamp,
    ) -> Self {
        Self {
            to_chain_id,
            initiator_account_id,
            initiator_sub_account_id,
            target_sub_account_id,
            target,
            l2_source_token,
            l1_target_token,
            initiator_nonce: nonce,
            signature: ZkLinkSignature::default(),
            ts,
            exit_amount,
        }
    }

    /// Encodes the transaction data as the byte sequence according to the zkLink protocol.
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
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
        out.extend_from_slice(&self.ts.to_be_bytes());
        out
    }

    #[cfg(feature = "ffi")]
    pub fn json_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    #[cfg(not(feature = "ffi"))]
    pub fn sign(&mut self, signer: &ZkLinkSigner) -> Result<(), ZkSignerError> {
        let bytes = self.get_bytes();
        self.signature = signer.sign_musig(&bytes)?;
        Ok(())
    }

    #[cfg(feature = "ffi")]
    pub fn signature(&self) -> ZkLinkSignature {
        self.signature.clone()
    }

    pub fn is_validate(&self) -> bool {
        self.validate().is_ok()
    }

    pub fn is_signature_valid(&self) -> Result<bool, ZkSignerError> {
        self.signature.verify_musig(&self.get_bytes())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_get_bytes() {
        let address =
            ZkLinkAddress::from_str("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9").unwrap();
        let ts = 1693472232u32;
        let forced_exit = ForcedExit::new(
            ChainId(1),
            AccountId(10),
            SubAccountId(1),
            address,
            SubAccountId(1),
            TokenId(18),
            TokenId(18),
            Nonce(1),
            BigUint::from(10000u32),
            ts.into(),
        );
        let bytes = forced_exit.get_bytes();
        println!("{:?}", bytes);
        let excepted_bytes = [
            7, 1, 0, 0, 0, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 175, 175, 243, 173, 26, 4,
            37, 215, 146, 67, 45, 158, 205, 28, 62, 38, 239, 44, 66, 233, 1, 0, 18, 0, 18, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 16, 100, 240, 85, 232,
        ];

        assert_eq!(bytes, excepted_bytes);
    }
}
