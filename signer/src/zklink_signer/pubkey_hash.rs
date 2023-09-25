use super::error::ZkSignerError as Error;
use crate::zklink_signer::NEW_PUBKEY_HASH_BYTES_LEN;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryInto;

/// Hash of the account's owner public key.
///
/// This is an essential type used within zklink network to authorize transaction author
/// to perform an operation.
///
/// `PubKeyHash` is calculated as the Rescue hash of the public key byte sequence.
#[derive(Copy, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
pub struct PubKeyHash {
    pub data: [u8; NEW_PUBKEY_HASH_BYTES_LEN],
}

impl std::fmt::Debug for PubKeyHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl PubKeyHash {
    /// Creates an uninitialized `PubkeyHash` object.
    /// This value is used for new accounts to signalize that `PubKeyHash` was not yet
    /// set for the corresponding account.
    /// Accounts with unset `PubKeyHash` are unable to execute L2 transactions.
    pub fn zero() -> Self {
        PubKeyHash {
            data: [0; NEW_PUBKEY_HASH_BYTES_LEN],
        }
    }

    /// Converts the `PubKeyHash` object into its hexadecimal representation.
    /// `PubKeyHash` hexadecimal form is prepended with the `0x` prefix.
    ///
    /// # Example
    ///
    /// ```
    /// use zklink_signers::zklink_signer::pubkey_hash::PubKeyHash;
    ///
    /// let pubkey_hash = PubKeyHash::zero();
    /// assert_eq!(pubkey_hash.as_hex(), "0x0000000000000000000000000000000000000000");
    /// ```
    pub fn as_hex(&self) -> String {
        format!("0x{}", hex::encode(self.data))
    }

    /// Decodes `PubKeyHash` from its hexadecimal form.
    /// Input string must have a `0x` prefix.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// use zklink_signers::zklink_signer::pubkey_hash::PubKeyHash;
    ///
    /// let pubkey_hash = PubKeyHash::from_hex("0x0000000000000000000000000000000000000000").unwrap();
    /// assert_eq!(pubkey_hash, PubKeyHash::zero());
    /// ```
    pub fn from_hex(s: &str) -> Result<Self, Error> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(s).map_err(|_| Error::InvalidPubkeyHash("invalid hex".into()))?;
        Self::from_bytes(&bytes)
    }

    /// Decodes `PubKeyHash` from the byte sequence.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let data: [u8; NEW_PUBKEY_HASH_BYTES_LEN] = bytes
            .try_into()
            .map_err(|_| Error::InvalidPubkeyHash("size mismatch".into()))?;
        Ok(PubKeyHash { data })
    }
}

impl Serialize for PubKeyHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_hex())
    }
}

impl<'de> Deserialize<'de> for PubKeyHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        PubKeyHash::from_hex(&string).map_err(serde::de::Error::custom)
    }
}
