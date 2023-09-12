use crate::basic_types::pack::{pack_fee_amount, pack_token_amount};
use crate::basic_types::params::TOKEN_MAX_PRECISION;
use crate::basic_types::{AccountId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress};
use crate::tx_type::ethereum_sign_message_part;
use crate::tx_type::validator::*;
use num::BigUint;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;
use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_signers::eth_signer::pk_signer::PrivateKeySigner;
use zklink_signers::zklink_signer::error::ZkSignerError;
#[cfg(not(feature = "ffi"))]
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_signers::zklink_signer::signature::ZkLinkSignature;

/// `Transfer` transaction performs a move of funds from one zklink account to another.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    /// zklink network account ID of the transaction initiator.
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    /// zklink network sub-account ID of the transaction initiator.
    #[validate(custom = "sub_account_validator")]
    pub from_sub_account_id: SubAccountId,
    /// zklink network sub-account ID of the transaction initiator.
    #[validate(custom = "sub_account_validator")]
    pub to_sub_account_id: SubAccountId,
    /// Layer1 address of account to transfer funds to.
    #[validate(custom = "zklink_address_validator")]
    pub to: ZkLinkAddress,
    /// Type of token for transfer. Also represents the token in which fee will be paid.
    #[validate(custom = "token_validator")]
    pub token: TokenId,
    /// Amount of funds to transfer, layer1 need unpack it, do packaging
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "amount_packable")]
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

    /// Used as request id
    pub ts: TimeStamp,
}

impl Transfer {
    /// Creates transaction from all the required fields.
    ///
    /// While `signature` field is mandatory for new transactions, it may be `None`
    /// in some cases (e.g. when restoring the network state from the L1 contract data).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_id: AccountId,
        to: ZkLinkAddress,
        from_sub_account_id: SubAccountId,
        to_sub_account_id: SubAccountId,
        token: TokenId,
        amount: BigUint,
        fee: BigUint,
        nonce: Nonce,
        ts: TimeStamp,
    ) -> Self {
        Self {
            account_id,
            from_sub_account_id,
            to_sub_account_id,
            to,
            token,
            amount,
            fee,
            nonce,
            signature: ZkLinkSignature::default(),
            ts,
        }
    }

    /// Encodes the transaction data as the byte sequence according to the zkLink protocol.
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&[Self::TX_TYPE]);
        out.extend_from_slice(&self.account_id.to_be_bytes());
        out.extend_from_slice(&self.from_sub_account_id.to_be_bytes());
        out.extend_from_slice(&self.to.to_fixed_bytes());
        out.extend_from_slice(&self.to_sub_account_id.to_be_bytes());
        out.extend_from_slice(&(*self.token as u16).to_be_bytes());
        out.extend_from_slice(&pack_token_amount(&self.amount));
        out.extend_from_slice(&pack_fee_amount(&self.fee));
        out.extend_from_slice(&self.nonce.to_be_bytes());
        out.extend_from_slice(&self.ts.to_be_bytes());
        out
    }

    #[cfg(not(feature = "ffi"))]
    pub fn sign(&mut self, signer: &ZkLinkSigner) -> Result<(), ZkSignerError> {
        let bytes = self.get_bytes();
        self.signature = signer.sign_musig(&bytes)?;
        Ok(())
    }

    #[cfg(feature = "ffi")]
    pub fn json_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
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
            "Transfer",
            token_symbol,
            TOKEN_MAX_PRECISION as u8,
            &self.amount,
            &self.fee,
            &self.to,
        )
    }

    /// Gets message that should be signed by Ethereum keys of the account for 2-Factor authentication.
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
        token_symbol: &str,
    ) -> Result<PackedEthSignature, ZkSignerError> {
        let message = self
            .get_ethereum_sign_message(token_symbol)
            .as_bytes()
            .to_vec();
        let eth_signature = eth_signer.sign_message(&message)?;
        Ok(eth_signature)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;
    use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
    use zklink_signers::eth_signer::pk_signer::PrivateKeySigner;
    use zklink_signers::zklink_signer::public_key::PackedPublicKey;

    #[test]
    fn test_get_bytes() {
        let address =
            ZkLinkAddress::from_str("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9").unwrap();
        let ts = 1693472232u32;
        let transfer = Transfer::new(
            AccountId(10),
            address,
            SubAccountId(1),
            SubAccountId(1),
            TokenId(18),
            BigUint::from(10000u32),
            BigUint::from(3u32),
            Nonce(1),
            ts.into(),
        );
        let bytes = transfer.get_bytes();
        let excepted_bytes = [
            4, 0, 0, 0, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 175, 175, 243, 173, 26, 4, 37,
            215, 146, 67, 45, 158, 205, 28, 62, 38, 239, 44, 66, 233, 1, 0, 18, 0, 0, 4, 226, 0, 0,
            96, 0, 0, 0, 1, 100, 240, 85, 232,
        ];
        assert_eq!(bytes, excepted_bytes);
    }

    #[test]
    fn test_verify_signature() {
        let private_key_str = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let signature = "0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f8125107ecbe23c307d18007ee43090940a4a43bd02bdcda206ad695f745c2f0a64f4ac4c4c8beb9ed9cbdd0e523e75ffc7dedd0281da4946bb37fa26a04283bd480a04";
        let public_key_str = "0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510";
        let ts = 1693472232;
        let eth_signature = "0x1f11707e54773e059bc38aa73526fe2b51af9b89a77df731af7bcc429750d0317727a857efda5d79232eb5f9a66ed60a79aad2195d4de1375f5021c0db041b221b";
        let address =
            ZkLinkAddress::from_str("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9").unwrap();
        let mut tx = Transfer::new(
            AccountId(1),
            address,
            SubAccountId(1),
            SubAccountId(1),
            TokenId(18),
            BigUint::from_str("100000").unwrap(),
            BigUint::from_str("100").unwrap(),
            Nonce(1),
            ts.into(),
        );
        //check l2 signature
        tx.signature = ZkLinkSignature::from_hex(signature).unwrap();
        let recover_pubkey_hash = tx.verify_signature().unwrap();
        let pubkey = PackedPublicKey::from_hex(public_key_str).unwrap();
        let pubkey_hash = pubkey.public_key_hash();

        //check l1 signature
        let l1_signature = PackedEthSignature::from_hex(eth_signature).unwrap();
        let token_symbol = "USDC";
        let message = tx
            .get_ethereum_sign_message(token_symbol)
            .as_bytes()
            .to_vec();
        let recover_address = l1_signature.signature_recover_signer(&message).unwrap();
        let private_key = PrivateKeySigner::try_from(private_key_str).unwrap();
        let address = private_key.get_address().unwrap();
        assert_eq!(pubkey_hash, recover_pubkey_hash);
        assert_eq!(address, recover_address);
    }
}
