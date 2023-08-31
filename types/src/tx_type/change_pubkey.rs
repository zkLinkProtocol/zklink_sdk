use crate::basic_types::{
    AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress, H256,
};
use crate::tx_type::format_units;
use crate::tx_type::pack::pack_fee_amount;
use crate::tx_type::validator::*;
use num::{BigUint, Zero};
use parity_crypto::Keccak256;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_crypto::eth_signer::eip712::eip712::{eip712_typed_data, EIP712Domain, TypedData};
use zklink_crypto::eth_signer::eip712::{BytesM, Uint};
use zklink_crypto::eth_signer::error::EthSignerError;
use zklink_crypto::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_crypto::eth_signer::EthTypedData;
use zklink_crypto::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_crypto::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

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
    pub creator_address: Address,
    pub salt_arg: H256,
    pub code_hash: H256,
}

impl Create2Data {
    pub fn get_address(&self, pubkey_hash: Vec<u8>) -> Address {
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
    Onchain,
    EthECDSA{
        eth_signature: PackedEthSignature
    },
    EthCREATE2{
        data: Create2Data
    },
    // StarkECDSA{
    //     data = StarkECDSAData
    // },
}

impl Default for ChangePubKeyAuthData {
    fn default() -> Self {
        Self::Onchain
    }
}

impl ChangePubKeyAuthData {
    pub fn is_eth_ecdsa(&self) -> bool {
        matches!(self, ChangePubKeyAuthData::EthECDSA{..})
    }

    pub fn is_onchain(&self) -> bool {
        matches!(self, ChangePubKeyAuthData::Onchain)
    }

    pub fn is_create2(&self) -> bool {
        matches!(self, ChangePubKeyAuthData::EthCREATE2{..})
    }

    // pub fn is_stark_ecdsa(&self) -> bool {
    //     matches!(self, ChangePubKeyAuthData::StarkECDSA(..))
    // }

    pub fn get_eth_witness(&self) -> Option<Vec<u8>> {
        match self {
            ChangePubKeyAuthData::Onchain => None,
            ChangePubKeyAuthData::EthECDSA{ eth_signature } => {
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
            ChangePubKeyAuthData::EthCREATE2{data} => {
                let mut bytes = Vec::new();
                bytes.push(0x01);
                bytes.extend_from_slice(data.creator_address.as_bytes());
                bytes.extend_from_slice(data.salt_arg.as_bytes());
                bytes.extend_from_slice(data.code_hash.as_bytes());
                Some(bytes)
            } // ChangePubKeyAuthData::StarkECDSA(StarkECDSAData{signature, public_key}) =>{
              //     let mut bytes = Vec::new();
              //     bytes.push(0x02);
              //     bytes.extend_from_slice(&signature.0);
              //     bytes.extend_from_slice(public_key);
              //     bytes
              // }
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
        signature: Option<ZkLinkSignature>,
        eth_signature: Option<PackedEthSignature>,
        ts: TimeStamp,
    ) -> Self {
        let eth_auth_data = eth_signature
            .map(|eth_signature| ChangePubKeyAuthData::EthECDSA { eth_signature })
            .unwrap_or(ChangePubKeyAuthData::Onchain);

        Self {
            chain_id,
            account_id,
            sub_account_id,
            new_pk_hash,
            fee_token,
            fee,
            nonce,
            signature: signature.unwrap_or_default(),
            eth_auth_data,
            ts,
        }
    }

    // /// Creates a signed transaction using private key and
    // /// checks for the transaction correcteness.
    // #[allow(clippy::too_many_arguments)]
    // pub fn new_signed(
    //     chain_id: ChainId,
    //     account_id: AccountId,
    //     sub_account_id: SubAccountId,
    //     new_pk_hash: PubKeyHash,
    //     fee_token: TokenId,
    //     fee: BigUint,
    //     nonce: Nonce,
    //     eth_signature: Option<PackedEthSignature>,
    //     private_key: &PrivateKey,
    //     ts: TimeStamp,
    // ) -> Result<Self, anyhow::Error> {
    //     let mut tx = Self::new(
    //         chain_id,
    //         account_id,
    //         sub_account_id,
    //         new_pk_hash,
    //         fee_token,
    //         fee,
    //         nonce,
    //         None,
    //         eth_signature,
    //         ts,
    //     );
    //     tx.signature = TxSignature::sign_musig(private_key, &tx.get_bytes());
    //     if !tx.is_validate() {
    //         anyhow::bail!(crate::tx::TRANSACTION_SIGNATURE_ERROR);
    //     }
    //     Ok(tx)
    // }

    // /// Restores the `PubKeyHash` from the transaction signature.
    // pub fn verify_signature(&self) -> Option<PubKeyHash> {
    //     self.signature
    //         .verify_musig(&self.get_bytes())
    //         .map(|pub_key| PubKeyHash::from_pubkey(&pub_key))
    // }

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
        let domain = EIP712Domain::from_chain(layer_one_chain_id, verifying_contract.to_string())?;
        let typed_data = eip712_typed_data::<EIP712ChangePubKey>(domain, self.into())?;
        let raw_data = serde_json::to_string(&typed_data)
            .map_err(|e| EthSignerError::CustomError(format!("serialization error: {e:?}")))?;
        let data_hash = typed_data.sign_hash()?;
        let data_hash = zklink_crypto::eth_signer::H256::from_slice(&data_hash.0);
        Ok(EthTypedData {
            raw_data,
            data_hash,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "ChangePubKey")]
#[serde(rename_all = "camelCase")]
struct EIP712ChangePubKey {
    pub_key_hash: BytesM<20>,
    nonce: Uint<32>,
    account_id: Uint<32>,
}

impl From<&ChangePubKey> for EIP712ChangePubKey {
    fn from(value: &ChangePubKey) -> Self {
        todo!()
    }
}
