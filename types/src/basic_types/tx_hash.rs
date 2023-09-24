use crate::error::TypeError as Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{convert::TryInto, str::FromStr};
use zklink_sdk_utils::serde::{Prefix, ZeroxPrefix};
use zklink_signers::eth_signer::H256;

const TX_HASH_LEN: usize = 32;

/// Transaction hash.
/// Essentially, a SHA-256 hash of transaction bytes encoded according to the zkLink protocol.
#[derive(Debug, Copy, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
pub struct TxHash {
    pub(crate) data: [u8; TX_HASH_LEN],
}

impl TxHash {
    /// Reads a transaction hash from its byte sequence representation.
    ///
    /// Returns none if the slice length does not match with hash length.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        let mut out = TxHash {
            data: [0_u8; TX_HASH_LEN],
        };

        if slice.len() != TX_HASH_LEN {
            Err(Error::SizeMismatch)
        } else {
            out.data.copy_from_slice(slice);
            Ok(out)
        }
    }

    pub fn from_hex(s: &str) -> Result<Self, Error> {
        let hash = TxHash::from_str(s).map_err(|_| Error::InvalidTxHash)?;
        Ok(hash)
    }

    pub fn as_hex(&self) -> String {
        self.to_string()
    }

    pub fn as_h256(&self) -> H256 {
        H256::from_slice(&self.data)
    }
}

impl AsRef<[u8]> for TxHash {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl ToString for TxHash {
    fn to_string(&self) -> String {
        format!("{}{}", ZeroxPrefix::prefix(), hex::encode(self.data))
    }
}

impl FromStr for TxHash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let zerox_prefix = ZeroxPrefix::prefix();
        if !s.starts_with(zerox_prefix) {
            return Err(Error::NotStartWithZerox);
        }
        let remove_prefix_start = zerox_prefix.len();
        let bytes = hex::decode(&s[remove_prefix_start..])
            .map_err(|e| Error::DecodeFromHexErr(e.to_string()))?;
        if bytes.len() != 32 {
            return Err(Error::SizeMismatch);
        }
        Ok(TxHash {
            data: bytes.as_slice().try_into().unwrap(),
        })
    }
}

impl Serialize for TxHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for TxHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Self::from_str(&string).map_err(serde::de::Error::custom)
    }
}
