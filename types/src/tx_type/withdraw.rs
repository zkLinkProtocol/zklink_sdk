use num::{BigUint, ToPrimitive};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ffi")]
use std::sync::Arc;
use validator::Validate;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;
use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_signers::eth_signer::pk_signer::EthSigner;
use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_signers::zklink_signer::signature::ZkLinkSignature;

use crate::basic_types::pack::pack_fee_amount;
use crate::basic_types::params::TOKEN_MAX_PRECISION;
use crate::basic_types::{
    AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::tx_builder::WithdrawBuilder;
use crate::tx_type::validator::*;
use crate::tx_type::{ethereum_sign_message_part, TxTrait, ZkSignatureTrait};
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
    pub fn new(builder: WithdrawBuilder) -> Self {
        let fast_withdraw = u8::from(builder.fast_withdraw);

        Self {
            to_chain_id: builder.to_chain_id,
            account_id: builder.account_id,
            sub_account_id: builder.sub_account_id,
            to: builder.to_address,
            l2_source_token: builder.l2_source_token,
            l1_target_token: builder.l1_target_token,
            amount: builder.amount,
            fee: builder.fee,
            nonce: builder.nonce,
            signature: ZkLinkSignature::default(),
            fast_withdraw,
            withdraw_fee_ratio: builder.withdraw_fee_ratio,
            ts: builder.timestamp,
        }
    }

    /// Restores the `PubKeyHash` from the transaction signature.
    pub fn verify_signature(&self) -> Option<PubKeyHash> {
        match self.signature.verify_musig(&self.get_bytes()) {
            Ok(ret) if ret => Some(self.signature.pub_key.public_key_hash()),
            _ => None,
        }
    }

    /// Get the first part of the message we expect to be signed by Ethereum account key.
    /// The only difference is the missing `nonce` since it's added at the end of the transactions
    /// batch message.
    pub fn get_eth_sign_message_part(&self, token_symbol: &str) -> String {
        ethereum_sign_message_part(
            "Withdraw",
            token_symbol,
            TOKEN_MAX_PRECISION,
            &self.amount,
            &self.fee,
            &self.to,
        )
    }

    /// Get message that should be signed by Ethereum keys of the account for 2-Factor authentication.
    pub fn get_eth_sign_msg(&self, token_symbol: &str) -> String {
        let mut message = self.get_eth_sign_message_part(token_symbol);
        if !message.is_empty() {
            message.push('\n');
        }
        message.push_str(format!("Nonce: {}", self.nonce).as_str());
        message
    }

    #[cfg(feature = "ffi")]
    pub fn eth_signature(
        &self,
        eth_signer: Arc<EthSigner>,
        l2_source_token_symbol: &str,
    ) -> Result<PackedEthSignature, ZkSignerError> {
        let message = self.get_eth_sign_msg(l2_source_token_symbol);
        let eth_signature = eth_signer.sign_message(message.as_bytes())?;
        Ok(eth_signature)
    }

    #[cfg(not(feature = "ffi"))]
    pub fn eth_signature(
        &self,
        eth_signer: &EthSigner,
        l2_source_token_symbol: &str,
    ) -> Result<PackedEthSignature, ZkSignerError> {
        let message = self.get_eth_sign_msg(l2_source_token_symbol);
        let eth_signature = eth_signer.sign_message(message.as_bytes())?;
        Ok(eth_signature)
    }
}

impl TxTrait for Withdraw {
    fn get_bytes(&self) -> Vec<u8> {
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
}

impl ZkSignatureTrait for Withdraw {
    fn set_signature(&mut self, signature: ZkLinkSignature) {
        self.signature = signature;
    }

    #[cfg(feature = "ffi")]
    fn signature(&self) -> ZkLinkSignature {
        self.signature.clone()
    }

    fn is_signature_valid(&self) -> Result<bool, ZkSignerError> {
        let bytes = self.get_bytes();
        self.signature.verify_musig(&bytes)
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
        let builder = WithdrawBuilder {
            account_id: AccountId(10),
            sub_account_id: SubAccountId(1),
            to_chain_id: ChainId(1),
            to_address: address,
            l2_source_token: TokenId(18),
            l1_target_token: TokenId(18),
            amount: BigUint::from(10000u32),
            fee: BigUint::from(3u32),
            nonce: Nonce(1),
            fast_withdraw: false,
            withdraw_fee_ratio: 0,
            timestamp: ts.into(),
        };
        let withdraw = Withdraw::new(builder);
        let bytes = withdraw.get_bytes();
        let excepted_bytes = [
            3, 1, 0, 0, 0, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 175, 175, 243, 173, 26, 4,
            37, 215, 146, 67, 45, 158, 205, 28, 62, 38, 239, 44, 66, 233, 0, 18, 0, 18, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 16, 0, 96, 0, 0, 0, 1, 0, 0, 0, 100, 240, 85, 232,
        ];

        assert_eq!(bytes, excepted_bytes);
    }
}
