use crate::basic_types::params::TOKEN_MAX_PRECISION;
use crate::basic_types::{AccountId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress};
use crate::tx_type::ethereum_sign_message_part;
use crate::tx_type::pack::{pack_fee_amount, pack_token_amount};
use crate::tx_type::validator::*;
use num::BigUint;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_crypto::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

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
        signature: Option<ZkLinkSignature>,
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
            signature: signature.unwrap_or_default(),
            ts,
        }
    }

    // /// Creates a signed transaction using private key and
    // /// checks for the transaction correcteness.
    // #[allow(clippy::too_many_arguments)]
    // pub fn new_signed(
    //     account_id: AccountId,
    //     to: ZkLinkAddress,
    //     from_sub_account_id: SubAccountId,
    //     to_sub_account_id: SubAccountId,
    //     token: TokenId,
    //     amount: BigUint,
    //     fee: BigUint,
    //     nonce: Nonce,
    //     private_key: &PrivateKey<Engine>,
    //     ts: TimeStamp,
    // ) -> Result<Self, anyhow::Error> {
    //     let mut tx = Self::new(
    //         account_id,
    //         to,
    //         from_sub_account_id,
    //         to_sub_account_id,
    //         token,
    //         amount,
    //         fee,
    //         nonce,
    //         None,
    //         ts,
    //     );
    //     tx.signature = TxSignature::sign_musig(private_key, &tx.get_bytes());
    //     if !tx.is_validate() {
    //         anyhow::bail!(crate::tx::TRANSACTION_SIGNATURE_ERROR);
    //     }
    //     Ok(tx)
    // }

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

    pub fn is_validate(&self) -> bool {
        self.validate().is_ok()
    }

    // /// Restores the `PubKeyHash` from the transaction signature.
    // pub fn verify_signature(&self) -> Option<PubKeyHash> {
    //     self.signature
    //         .verify_musig(&self.get_bytes())
    //         .map(|pub_key| PubKeyHash::from_pubkey(&pub_key))
    // }

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
}
