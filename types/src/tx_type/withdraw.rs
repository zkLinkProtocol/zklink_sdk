use num::{BigUint, ToPrimitive};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ffi")]
use std::sync::Arc;
use validator::Validate;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;
#[cfg(feature = "ffi")]
use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
#[cfg(feature = "ffi")]
use zklink_signers::eth_signer::pk_signer::PrivateKeySigner;
use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_signers::zklink_signer::pk_signer::sha256_bytes;
#[cfg(not(feature = "ffi"))]
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_signers::zklink_signer::signature::ZkLinkSignature;

use crate::basic_types::pack::pack_fee_amount;
use crate::basic_types::params::TOKEN_MAX_PRECISION;
use crate::basic_types::{
    AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::tx_type::ethereum_sign_message_part;
use crate::tx_type::validator::*;
use zklink_signers::zklink_signer::pubkey_hash::PubKeyHash;

/// `Withdraw` transaction performs a withdrawal of funds from zklink account to L1 account.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw {
    /// Target chain of withdraw.
    #[validate(custom = "chain_id_validator")]
    pub to_chain_id: ChainId,
    /// zkLink network account ID of the transaction initiator.
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    /// The source sub-account id of withdraw amount.
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    /// Address of L1 account to withdraw funds to.
    #[validate(custom = "zklink_address_validator")]
    pub to: ZkLinkAddress,
    /// Source token and target token of withdrawal from l2 to l1.
    /// Also represents the token in which fee will be paid.
    #[validate(custom = "token_validator")]
    pub l2_source_token: TokenId,
    #[validate(custom = "token_validator")]
    pub l1_target_token: TokenId,
    /// Amount of funds to withdraw, layer1 can not unpack it, do not packaging
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "amount_unpackable")]
    pub amount: BigUint,
    /// Fee for the transaction, need packaging
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "fee_packable")]
    pub fee: BigUint,
    /// Current account nonce.
    #[validate(custom = "nonce_validator")]
    pub nonce: Nonce,
    /// Transaction zkLink signature.
    #[serde(default)]
    pub signature: ZkLinkSignature,

    /// Fast withdraw or normal withdraw
    #[validate(custom = "boolean_validator")]
    pub fast_withdraw: u8,
    /// Amount of funds to withdraw.
    #[validate(custom = "withdraw_fee_ratio_validator")]
    pub withdraw_fee_ratio: u16,
    /// Used as request id
    pub ts: TimeStamp,
}

impl Withdraw {
    /// Creates transaction from all the required fields.
    ///
    /// While `signature` field is mandatory for new transactions, it may be `None`
    /// in some cases (e.g. when restoring the network state from the L1 contract data).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_id: AccountId,
        sub_account_id: SubAccountId,
        to_chain_id: ChainId,
        to: ZkLinkAddress,
        l2_source_token: TokenId,
        l1_target_token: TokenId,
        amount: BigUint,
        fee: BigUint,
        nonce: Nonce,
        fast_withdraw: bool,
        withdraw_fee_ratio: u16,
        ts: TimeStamp,
    ) -> Self {
        let fast_withdraw = u8::from(fast_withdraw);

        Self {
            to_chain_id,
            account_id,
            sub_account_id,
            to,
            l2_source_token,
            l1_target_token,
            amount,
            fee,
            nonce,
            signature: ZkLinkSignature::default(),
            fast_withdraw,
            withdraw_fee_ratio,
            ts,
        }
    }

    /// Encodes the transaction data as the byte sequence according to the zkLink protocol.
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&[Self::TX_TYPE]);
        out.extend_from_slice(&self.to_chain_id.to_be_bytes());
        out.extend_from_slice(&self.account_id.to_be_bytes());
        out.extend_from_slice(&self.sub_account_id.to_be_bytes());
        out.extend_from_slice(&self.to.to_fixed_bytes());
        out.extend_from_slice(&(*self.l2_source_token as u16).to_be_bytes());
        out.extend_from_slice(&(*self.l1_target_token as u16).to_be_bytes());
        out.extend_from_slice(&self.amount.to_u128().unwrap().to_be_bytes());
        out.extend_from_slice(&pack_fee_amount(&self.fee));
        out.extend_from_slice(&self.nonce.to_be_bytes());
        out.extend_from_slice(&self.fast_withdraw.to_be_bytes());
        out.extend_from_slice(&self.withdraw_fee_ratio.to_be_bytes());
        out.extend_from_slice(&self.ts.to_be_bytes());
        out
    }

    pub fn tx_hash(&self) -> Vec<u8> {
        let bytes = self.get_bytes();
        sha256_bytes(&bytes)
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

    pub fn is_signature_valid(&self) -> Result<bool, ZkSignerError> {
        self.signature.verify_musig(&self.get_bytes())
    }

    pub fn is_validate(&self) -> bool {
        self.validate().is_ok()
    }

    /// Restores the `PubKeyHash` from the transaction signature.
    pub fn verify_signature(&self) -> Option<PubKeyHash> {
        match self.signature.verify_musig(&self.get_bytes()) {
            Ok(ret) if ret => Some(self.signature.public_key.public_key_hash()),
            _ => None,
        }
    }

    /// Get the first part of the message we expect to be signed by Ethereum account key.
    /// The only difference is the missing `nonce` since it's added at the end of the transactions
    /// batch message.
    pub fn get_ethereum_sign_message_part(&self, token_symbol: &str) -> String {
        ethereum_sign_message_part(
            "Withdraw",
            token_symbol,
            TOKEN_MAX_PRECISION as u8,
            &self.amount,
            &self.fee,
            &self.to,
        )
    }

    /// Get message that should be signed by Ethereum keys of the account for 2-Factor authentication.
    pub fn get_ethereum_sign_message(&self, token_symbol: &str) -> String {
        let mut message = self.get_ethereum_sign_message_part(token_symbol);
        if !message.is_empty() {
            message.push('\n');
        }
        message.push_str(format!("Nonce: {}", self.nonce).as_str());
        message
    }

    #[cfg(feature = "ffi")]
    pub fn eth_signature(
        &self,
        eth_signer: Arc<PrivateKeySigner>,
        l2_source_token_symbol: &str,
    ) -> Result<PackedEthSignature, ZkSignerError> {
        let message = self.get_ethereum_sign_message(&l2_source_token_symbol);
        let eth_signature = eth_signer.sign_message(message.as_bytes())?;
        Ok(eth_signature)
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
        let withdraw = Withdraw::new(
            AccountId(10),
            SubAccountId(1),
            ChainId(1),
            address,
            TokenId(18),
            TokenId(18),
            BigUint::from(10000u32),
            BigUint::from(3u32),
            Nonce(1),
            false,
            0,
            ts.into(),
        );
        let bytes = withdraw.get_bytes();
        let excepted_bytes = [
            3, 1, 0, 0, 0, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 175, 175, 243, 173, 26, 4,
            37, 215, 146, 67, 45, 158, 205, 28, 62, 38, 239, 44, 66, 233, 0, 18, 0, 18, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 16, 0, 96, 0, 0, 0, 1, 0, 0, 0, 100, 240, 85, 232,
        ];

        assert_eq!(bytes, excepted_bytes);
    }
}
