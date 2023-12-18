#![allow(dead_code)]
use super::error::StarkSignerError;
use crate::starknet_signer::typed_data::TypedData;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use starknet_core::crypto::Signature;
use starknet_core::types::FieldElement;
use starknet_signers::VerifyingKey;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use zklink_sdk_utils::serde::ZeroPrefixHexSerde;

#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Debug)]
pub struct StarkEcdsaSignature {
    pub s: FieldElement,
    pub r: FieldElement,
}

impl StarkEcdsaSignature {
    pub fn to_bytes_be(&self) -> [u8; 64] {
        let mut bytes = [0; 64];
        let s = self.s.to_bytes_be();
        let r = self.r.to_bytes_be();
        bytes[0..32].clone_from_slice(&s);
        bytes[32..].clone_from_slice(&r);
        bytes
    }

    pub fn as_hex(&self) -> String {
        let bytes = self.to_bytes_be();
        hex::encode(bytes)
    }

    pub fn from_hex(s: &str) -> Result<Self, StarkSignerError> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(s).map_err(StarkSignerError::invalid_signature)?;
        Self::from_bytes_be(&bytes)
    }

    pub fn from_rs_str(r: &str, s: &str) -> Result<Self, StarkSignerError> {
        let r = FieldElement::from_str(r)
            .map_err(|e| StarkSignerError::InvalidSignature(e.to_string()))?;
        let s = FieldElement::from_str(s)
            .map_err(|e| StarkSignerError::InvalidSignature(e.to_string()))?;
        Ok(Self { s, r })
    }

    pub fn from_bytes_be(bytes: &[u8]) -> Result<Self, StarkSignerError> {
        let mut s = [0_u8; 32];
        let mut r = [0_u8; 32];
        if bytes.len() != 64 {
            return Err(StarkSignerError::invalid_signature(
                "bytes should be 64 length",
            ));
        }
        s.clone_from_slice(&bytes[0..32]);
        r.clone_from_slice(&bytes[32..]);
        let s = FieldElement::from_bytes_be(&s)
            .map_err(|e| StarkSignerError::invalid_signature(e.to_string()))?;
        let r = FieldElement::from_bytes_be(&r)
            .map_err(|e| StarkSignerError::invalid_signature(e.to_string()))?;
        Ok(Self { s, r })
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct StarkEip712Signature {
    /// starknet public key
    pub pub_key: FieldElement,
    /// starknet signature
    pub signature: StarkEcdsaSignature,
}

impl StarkEip712Signature {
    pub fn to_bytes_be(&self) -> Vec<u8> {
        let mut bytes = [0_u8; 96];
        let pub_key = self.pub_key.to_bytes_be();
        let signature = self.signature.to_bytes_be();
        bytes[0..32].copy_from_slice(&pub_key);
        bytes[32..].copy_from_slice(&signature);
        bytes.to_vec()
    }

    pub fn from_bytes_be(bytes: &[u8]) -> Result<Self, StarkSignerError> {
        if bytes.len() != 96 {
            return Err(StarkSignerError::invalid_signature(
                "bytes length should be equal to 96",
            ));
        }
        let mut pub_key = [0_u8; 32];
        pub_key.clone_from_slice(&bytes[0..32]);
        let pub_key = FieldElement::from_bytes_be(&pub_key)
            .map_err(|_| StarkSignerError::invalid_signature("invalid public key"))?;
        let signature = StarkEcdsaSignature::from_bytes_be(&bytes[32..])?;
        Ok(Self { pub_key, signature })
    }

    pub fn as_hex(&self) -> String {
        let bytes = self.to_bytes_be();
        format!("0x{}", hex::encode(bytes))
    }

    pub fn from_hex(s: &str) -> Result<Self, StarkSignerError> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(s).map_err(StarkSignerError::invalid_signature)?;
        Self::from_bytes_be(&bytes)
    }
}

impl StarkEip712Signature {
    pub fn verify(&self, msg: &TypedData, addr: &str) -> Result<bool, StarkSignerError> {
        let addr = FieldElement::from_hex_be(addr)
            .map_err(|e| StarkSignerError::SignError(e.to_string()))?;
        let hash = msg.get_message_hash(addr)?;
        let verifying_key = VerifyingKey::from_scalar(self.pub_key);
        let is_ok = verifying_key
            .verify(
                &hash,
                &Signature {
                    r: self.signature.r,
                    s: self.signature.s,
                },
            )
            .map_err(StarkSignerError::invalid_signature)?;
        Ok(is_ok)
    }
}

impl fmt::Display for StarkEip712Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StarkEip712Signature {}", self.as_hex())
    }
}

impl fmt::Debug for StarkEip712Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl<'de> Deserialize<'de> for StarkEip712Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = ZeroPrefixHexSerde::deserialize(deserializer)?;
        let signature: Self = Self::from_bytes_be(&bytes).map_err(serde::de::Error::custom)?;
        Ok(signature)
    }
}

impl Serialize for StarkEip712Signature {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let bytes = self.to_bytes_be();
        ZeroPrefixHexSerde::serialize(bytes, serializer)
    }
}
