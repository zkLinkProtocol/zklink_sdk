use crate::basic_types::pack::pack_fee_amount;
use crate::basic_types::{
    AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::tx_type::format_units;
use crate::tx_type::validator::*;
use num::{BigUint, Zero};
use parity_crypto::Keccak256;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;
use zklink_signers::eth_signer::eip712::eip712::{EIP712Domain, TypedData};
use zklink_signers::eth_signer::eip712::{BytesM, Uint};
use zklink_signers::eth_signer::error::EthSignerError;
use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_signers::eth_signer::EthTypedData;
use zklink_signers::eth_signer::H256;
use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_signers::zklink_signer::pk_signer::sha256_bytes;
#[cfg(not(feature = "ffi"))]
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_signers::zklink_signer::signature::ZkLinkSignature;

//todo: starknet
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct StarkECDSAData {
//     pub signature: StarkECDSASignature,
//     pub public_key: Vec<u8>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Create2Data {
    pub creator_address: ZkLinkAddress,
    pub salt_arg: H256,
    pub code_hash: H256,
}

impl Create2Data {
    pub fn get_address(&self, pubkey_hash: Vec<u8>) -> ZkLinkAddress {
        let salt = {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(self.salt_arg.as_bytes());
            bytes.extend_from_slice(&pubkey_hash);
            bytes.keccak256()
        };

        let mut bytes = Vec::new();
        bytes.push(0xff);
        bytes.extend_from_slice(self.creator_address.as_bytes());
        bytes.extend_from_slice(&salt);
        bytes.extend_from_slice(self.code_hash.as_bytes());
        ZkLinkAddress::from_slice(&bytes.keccak256()[12..]).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChangePubKeyAuthData {
    OnChain,
    EthECDSA { eth_signature: PackedEthSignature },
    EthCreate2 { data: Create2Data },
}

impl Default for ChangePubKeyAuthData {
    fn default() -> Self {
        Self::OnChain
    }
}

impl ChangePubKeyAuthData {
    pub fn is_eth_ecdsa(&self) -> bool {
        matches!(self, ChangePubKeyAuthData::EthECDSA { .. })
    }

    pub fn is_onchain(&self) -> bool {
        matches!(self, ChangePubKeyAuthData::OnChain)
    }

    pub fn is_create2(&self) -> bool {
        matches!(self, ChangePubKeyAuthData::EthCreate2 { .. })
    }

    pub fn get_eth_witness(&self) -> Option<Vec<u8>> {
        match self {
            ChangePubKeyAuthData::OnChain => None,
            ChangePubKeyAuthData::EthECDSA { eth_signature } => {
                let mut bytes = Vec::new();
                bytes.push(0x00);
                bytes.extend_from_slice(&eth_signature.0[..64]);
                // add 27 to v
                let mut v = eth_signature.0[64];
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

impl ChangePubKey {
    /// Creates transaction from all the required fields.
    ///
    /// While `signature` field is mandatory for new transactions, it may be `None`
    /// in some cases (e.g. when restoring the network state from the L1 contract data).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        chain_id: ChainId,
        account_id: AccountId,
        sub_account_id: SubAccountId,
        new_pk_hash: PubKeyHash,
        fee_token: TokenId,
        fee: BigUint,
        nonce: Nonce,
        eth_signature: Option<PackedEthSignature>,
        ts: TimeStamp,
    ) -> Self {
        let eth_auth_data = eth_signature
            .map(|eth_signature| ChangePubKeyAuthData::EthECDSA { eth_signature })
            .unwrap_or(ChangePubKeyAuthData::OnChain);

        Self {
            chain_id,
            account_id,
            sub_account_id,
            new_pk_hash,
            fee_token,
            fee,
            nonce,
            signature: ZkLinkSignature::default(),
            eth_auth_data,
            ts,
        }
    }

    /// Encodes the transaction data as the byte sequence according to the zkLink protocol.
    pub fn get_bytes(&self) -> Vec<u8> {
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
    pub fn get_ethereum_sign_message_part(&self, token_symbol: &str, decimals: u8) -> String {
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
        let data_hash = zklink_signers::eth_signer::H256::from_slice(&data_hash.0);
        Ok(EthTypedData {
            raw_data,
            data_hash,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "ChangePubKey", rename_all = "camelCase")]
pub struct EIP712ChangePubKey {
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
    use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;

    #[test]
    fn test_get_bytes_onchain() {
        let eth_private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(eth_private_key).unwrap();
        let pub_key = zk_signer.public_key();
        let pub_key_hash = pub_key.public_key_hash();
        let ts = 1693472232u32;
        let change_pubkey = ChangePubKey::new(
            ChainId(1),
            AccountId(1),
            SubAccountId(1),
            pub_key_hash,
            TokenId(18),
            BigUint::from(100u32),
            Nonce(1),
            Default::default(),
            ts.into(),
        );
        let bytes = change_pubkey.get_bytes();
        let excepted_bytes = [
            0, 0, 0, 0, 0, 0, 125, 1, 227, 95, 58, 57, 213, 66, 246, 210, 118, 194, 242, 3, 232,
            253, 100, 252, 184, 191, 93, 176, 98, 183, 28, 202, 207, 69, 213, 236, 217, 212, 86,
            243,
        ];
        assert_eq!(bytes, excepted_bytes);
    }
}
