use crate::basic_types::pack::pack_fee_amount;
use crate::basic_types::params::{SIGNED_CHANGE_PUBKEY_BIT_WIDTH, TX_TYPE_BIT_WIDTH};
use crate::basic_types::{
    AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::tx_builder::ChangePubKeyBuilder;
use crate::tx_type::validator::*;
use crate::tx_type::{format_units, TxTrait, ZkSignatureTrait};
use ethers::utils::keccak256;
use num::{BigUint, Zero};
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_signers::eth_signer::eip712::eip712::{EIP712Domain, TypedData};
use zklink_sdk_signers::eth_signer::eip712::{BytesM, Uint};
use zklink_sdk_signers::eth_signer::error::EthSignerError;
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::eth_signer::EthTypedData;
use zklink_sdk_signers::eth_signer::H256;
use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Create2Data {
    pub creator_address: ZkLinkAddress,
    pub salt_arg: H256,
    pub code_hash: H256,
}

impl Create2Data {
    pub fn salt(&self, pubkey_hash: &[u8]) -> [u8; 32] {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.salt_arg.as_bytes());
        bytes.extend_from_slice(pubkey_hash);
        keccak256(bytes)
    }
    pub fn get_address(&self, pubkey_hash: &[u8]) -> ZkLinkAddress {
        let salt = self.salt(pubkey_hash);
        let mut bytes = Vec::new();
        bytes.push(0xff);
        bytes.extend_from_slice(self.creator_address.as_bytes());
        bytes.extend_from_slice(&salt);
        bytes.extend_from_slice(self.code_hash.as_bytes());
        ZkLinkAddress::from_slice(&keccak256(bytes)[12..]).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChangePubKeyAuthData {
    Onchain,
    EthECDSA {
        #[serde(rename = "ethSignature")]
        eth_signature: PackedEthSignature,
    },
    EthCreate2 {
        data: Create2Data,
    },
}

impl Default for ChangePubKeyAuthData {
    fn default() -> Self {
        Self::Onchain
    }
}

impl ChangePubKeyAuthData {
    pub fn is_eth_ecdsa(&self) -> bool {
        matches!(self, ChangePubKeyAuthData::EthECDSA { .. })
    }

    pub fn is_onchain(&self) -> bool {
        matches!(self, ChangePubKeyAuthData::Onchain)
    }

    pub fn is_create2(&self) -> bool {
        matches!(self, ChangePubKeyAuthData::EthCreate2 { .. })
    }

    pub fn get_eth_witness(&self) -> Option<Vec<u8>> {
        match self {
            ChangePubKeyAuthData::Onchain => None,
            ChangePubKeyAuthData::EthECDSA { eth_signature } => {
                let mut bytes = Vec::new();
                bytes.push(0x00);
                bytes.extend_from_slice(&eth_signature.0.to_vec()[..64]);
                // add 27 to v
                let mut v = eth_signature.0.to_vec()[64];
                if v == 0 || v == 1 {
                    v += 27;
                }
                bytes.push(v);
                Some(bytes)
            }
            ChangePubKeyAuthData::EthCreate2 { data } => {
                let mut bytes = Vec::new();
                bytes.push(0x01);
                bytes.extend_from_slice(data.creator_address.as_bytes());
                bytes.extend_from_slice(data.salt_arg.as_bytes());
                bytes.extend_from_slice(data.code_hash.as_bytes());
                Some(bytes)
            }
        }
    }
}

/// `ChangePubKey` transaction is used to set the owner"s public key hash
/// associated with the account.
///
/// Without public key hash set, account is unable to execute any L2 transactions.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangePubKey {
    /// According to layer2 chain id , select eip712domain layer1 chain id.
    #[validate(custom = "chain_id_validator")]
    pub chain_id: ChainId,
    /// zklink network account ID to apply operation to.
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    /// zklink network sub account ID to apply operation to.
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    /// Public key hash to set.
    pub new_pk_hash: PubKeyHash,
    /// Token to be used for fee.
    #[validate(custom = "token_validator")]
    pub fee_token: TokenId,
    /// Fee for the transaction, need packaging
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "fee_packable")]
    pub fee: BigUint,
    /// Current account nonce of account_id
    #[validate(custom = "nonce_validator")]
    pub nonce: Nonce,
    /// Transaction zkLink signature. Must be signed with the key corresponding to the
    /// `new_pk_hash` value. This signature is required to ensure that `fee_token` and `fee`
    /// fields can"t be changed by an attacker.
    #[serde(default)]
    pub signature: ZkLinkSignature,
    /// Data needed to check if Ethereum address authorized ChangePubKey operation
    pub eth_auth_data: ChangePubKeyAuthData,
    /// Used as request id
    pub ts: TimeStamp,
}

impl TxTrait for ChangePubKey {
    fn get_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&[Self::TX_TYPE]);
        out.extend_from_slice(&self.chain_id.to_be_bytes());
        out.extend_from_slice(&self.account_id.to_be_bytes());
        out.extend_from_slice(&self.sub_account_id.to_be_bytes());
        out.extend_from_slice(&self.new_pk_hash.data);
        out.extend_from_slice(&(*self.fee_token as u16).to_be_bytes());
        out.extend_from_slice(&pack_fee_amount(&self.fee));
        out.extend_from_slice(&self.nonce.to_be_bytes());
        out.extend_from_slice(&self.ts.to_be_bytes());
        assert_eq!(
            out.len() * TX_TYPE_BIT_WIDTH,
            SIGNED_CHANGE_PUBKEY_BIT_WIDTH
        );
        out
    }
}

impl ZkSignatureTrait for ChangePubKey {
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

impl ChangePubKey {
    /// Creates transaction from all the required fields.
    ///
    /// While `signature` field is mandatory for new transactions, it may be `None`
    /// in some cases (e.g. when restoring the network state from the L1 contract data).
    pub fn new(builder: ChangePubKeyBuilder) -> Self {
        let eth_auth_data = builder
            .eth_signature
            .map(|eth_signature| ChangePubKeyAuthData::EthECDSA { eth_signature })
            .unwrap_or(ChangePubKeyAuthData::Onchain);

        Self {
            chain_id: builder.chain_id,
            account_id: builder.account_id,
            sub_account_id: builder.sub_account_id,
            new_pk_hash: builder.new_pubkey_hash,
            fee_token: builder.fee_token,
            fee: builder.fee,
            nonce: builder.nonce,
            signature: ZkLinkSignature::default(),
            eth_auth_data,
            ts: builder.timestamp,
        }
    }

    pub fn sign(&mut self, signer: &ZkLinkSigner) -> Result<(), ZkSignerError> {
        let bytes = self.get_bytes();
        self.signature = signer.sign_musig(&bytes)?;
        Ok(())
    }

    pub fn is_onchain(&self) -> bool {
        self.eth_auth_data.is_onchain()
    }

    /// Get part of the message that should be signed with Ethereum account key for the batch of transactions.
    /// The message for single `ChangePubKey` transaction is defined differently. The pattern is:
    ///
    /// Set signing key: {pubKeyHash}
    /// [Fee: {fee} {token}]
    ///
    /// Note that the second line is optional.
    pub fn get_eth_sign_msg_part(&self, token_symbol: &str, decimals: u8) -> String {
        let mut message = format!(
            "Set signing key: {}",
            hex::encode(self.new_pk_hash.data).to_ascii_lowercase()
        );
        if !self.fee.is_zero() {
            message.push_str(
                format!(
                    "\nFee: {fee} {token}",
                    fee = format_units(&self.fee, decimals),
                    token = token_symbol,
                )
                .as_str(),
            );
        }
        message
    }

    pub fn to_eip712_request_payload(
        &self,
        layer_one_chain_id: u32,
        verifying_contract: &ZkLinkAddress,
    ) -> Result<EthTypedData, EthSignerError> {
        let domain =
            EIP712Domain::new_zklink_domain(layer_one_chain_id, verifying_contract.to_string())?;
        let typed_data = TypedData::<EIP712ChangePubKey>::new(domain, self.into())?;
        let raw_data = serde_json::to_string(&typed_data)
            .map_err(|e| EthSignerError::CustomError(format!("serialization error: {e:?}")))?;
        let data_hash = typed_data.sign_hash()?;
        let data_hash = H256::from_slice(&data_hash.0);
        Ok(EthTypedData {
            raw_data,
            data_hash,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "ChangePubKey", rename_all = "camelCase")]
pub(crate) struct EIP712ChangePubKey {
    pub_key_hash: BytesM<20>,
    nonce: Uint<32>,
    account_id: Uint<32>,
}

impl From<&ChangePubKey> for EIP712ChangePubKey {
    fn from(change_pub_key: &ChangePubKey) -> Self {
        let pub_key_hash: BytesM<20> = BytesM::from(change_pub_key.new_pk_hash.data);
        let nonce: Uint<32> = Uint::from(change_pub_key.nonce.0);
        let account_id: Uint<32> = Uint::from(change_pub_key.account_id.0);

        EIP712ChangePubKey {
            pub_key_hash,
            nonce,
            account_id,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;

    #[test]
    fn test_get_bytes_onchain() {
        let eth_private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(eth_private_key).unwrap();
        let pub_key = zk_signer.public_key();
        let pub_key_hash = pub_key.public_key_hash();
        println!("pubkey hash: {:?}", pub_key_hash);
        let ts = 1693472232u32;
        let builder = ChangePubKeyBuilder {
            chain_id: ChainId(1),
            account_id: AccountId(1),
            sub_account_id: SubAccountId(1),
            new_pubkey_hash: pub_key_hash,
            fee_token: TokenId(18),
            fee: BigUint::from(100u32),
            nonce: Nonce(1),
            eth_signature: None,
            timestamp: ts.into(),
        };
        let change_pubkey = ChangePubKey::new(builder);
        let bytes = change_pubkey.get_bytes();
        let expected_bytes = [
            6, 1, 0, 0, 0, 1, 1, 216, 213, 251, 106, 108, 174, 240, 106, 163, 220, 42, 189, 205,
            194, 64, 152, 126, 83, 48, 254, 0, 18, 12, 128, 0, 0, 0, 1, 100, 240, 85, 232,
        ];

        assert_eq!(bytes, expected_bytes);
    }
}
