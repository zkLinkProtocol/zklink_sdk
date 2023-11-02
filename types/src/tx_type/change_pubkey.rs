use crate::basic_types::pack::pack_fee_amount;
use crate::basic_types::{
    AccountId, ChainId, GetBytes, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::params::{SIGNED_CHANGE_PUBKEY_BIT_WIDTH, TX_TYPE_BIT_WIDTH};
#[cfg(feature = "ffi")]
use crate::prelude::ChangePubKeyBuilder;
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
        #[serde(flatten)]
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

    pub fn get_eth_witness(&self) -> Vec<u8> {
        match self {
            ChangePubKeyAuthData::Onchain => vec![],
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
                bytes
            }
            ChangePubKeyAuthData::EthCreate2 { data } => {
                let mut bytes = Vec::new();
                bytes.push(0x01);
                bytes.extend_from_slice(data.creator_address.as_bytes());
                bytes.extend_from_slice(data.salt_arg.as_bytes());
                bytes.extend_from_slice(data.code_hash.as_bytes());
                bytes
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

impl GetBytes for ChangePubKey {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut out = Vec::with_capacity(bytes_len);
        out.extend_from_slice(&[Self::TX_TYPE]);
        out.extend_from_slice(&self.chain_id.to_be_bytes());
        out.extend_from_slice(&self.account_id.to_be_bytes());
        out.extend_from_slice(&self.sub_account_id.to_be_bytes());
        out.extend_from_slice(&self.new_pk_hash.data);
        out.extend_from_slice(&(*self.fee_token as u16).to_be_bytes());
        out.extend_from_slice(&pack_fee_amount(&self.fee));
        out.extend_from_slice(&self.nonce.to_be_bytes());
        out.extend_from_slice(&self.ts.to_be_bytes());
        assert_eq!(out.len(), bytes_len);
        out
    }

    fn bytes_len(&self) -> usize {
        SIGNED_CHANGE_PUBKEY_BIT_WIDTH / TX_TYPE_BIT_WIDTH
    }
}

impl TxTrait for ChangePubKey {}

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
    #[cfg(feature = "ffi")]
    pub fn new(builder: ChangePubKeyBuilder) -> Self {
        builder.build()
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
    /// "ChangePubKey\nPubKeyHash: {PubKeyHash}\nNonce: {Nonce}\nAccountId: {AccountId}"
    ///
    /// for example:
    /// ChangePubKey
    /// PubKeyHash: 0x823b747710c5bc9b8a47243f2c3d1805f1aa00c5
    /// Nonce: 3
    /// AccountId: 2
    ///
    #[inline]
    pub fn get_eth_sign_msg(
        pubkey_hash: &PubKeyHash,
        nonce: Nonce,
        account_id: AccountId,
    ) -> String {
        format!(
            "ChangePubKey\nPubKeyHash: {}\nNonce: {}\nAccountId: {}",
            pubkey_hash.as_hex(),
            nonce,
            account_id
        )
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
    use crate::prelude::ChangePubKeyBuilder;
    use zklink_sdk_signers::eth_signer::EthSigner;
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
        let change_pubkey = builder.build();
        let bytes = change_pubkey.get_bytes();
        let expected_bytes = [
            6, 1, 0, 0, 0, 1, 1, 216, 213, 251, 106, 108, 174, 240, 106, 163, 220, 42, 189, 205,
            194, 64, 152, 126, 83, 48, 254, 0, 18, 12, 128, 0, 0, 0, 1, 100, 240, 85, 232,
        ];
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn test_change_pubkey_eth_sign_msg() {
        let pubkey_hash =
            PubKeyHash::from_hex("0xdbd9c8235e4fc9d5b9b7bb201f1133e8a28c0edd").unwrap();
        let nonce = Nonce(0);
        let account_id = 2.into();
        let key: H256 = [5; 32].into();
        let signer = EthSigner::from(key);
        let eth_sign_msg = ChangePubKey::get_eth_sign_msg(&pubkey_hash, nonce, account_id);
        let eth_signature = signer.sign_message(eth_sign_msg.as_bytes()).unwrap();
        assert_eq!(eth_signature.as_hex(), "0xefd0d9c6beb00310535bb51ee58745adb547e7d875d5823892365a6450caf6c559a6a4bfd83bf336ac59cf83e97948dbf607bf2aecd24f6829c3deac20ecdb601b");
    }

    #[test]
    fn test_change_pubkey_create2() {
        let s = r#"
        {
   "ethAuthType":"EthCreate2",
   "chainId":2,
   "account":"0x4504d5BE8634e3896d42784A5aB89fc41C3d4511",
   "accountId":22,
   "subAccountId":0,
   "fee":"0",
   "nonce":0,
   "ts":1698128102,
   "type":"ChangePubKey",
   "newPkHash":"0x179d3888ad53fb3ce5e01f548c2e7c50dea076a6",
   "feeToken":17,
   "ethAuthData":{
      "type":"EthCreate2",
      "creatorAddress":"0x6E253C951A40fAf4032faFbEc19262Cd1531A5F5",
      "saltArg":"0x0000000000000000000000000000000000000000000000000000000000000000",
      "codeHash":"0x4f063cd4b2e3a885f61fefb0988cc12487182c4f09ff5de374103f5812f33fe7"
   },
   "signature":{
      "pubKey":"0b3e7d5328193b9cda3d5372cece28be209b4c7c136e734c6261c4fda965e710",
      "signature":"2f28abf960060dab8d829af5e243b35e2d41545c3354eef4f897a44bca73c629b60d997e343c4a0dd95e68f879c1e09c17fccb93906d458b1a75b7910d89a303"
   }
}"#;
        let tx: Result<ChangePubKey, _> = serde_json::from_str(s);
        assert!(tx.is_ok())
    }
}
