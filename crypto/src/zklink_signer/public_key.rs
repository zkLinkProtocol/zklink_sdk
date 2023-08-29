use std::sync::Arc;
use super::error::ZkSignerError as Error;
use super::{EddsaPubkey, Engine, JUBJUB_PARAMS};
use crate::zklink_signer::private_key::PackedPrivateKey;
use crate::zklink_signer::pubkey_hash::PubKeyHash;
use crate::zklink_signer::utils::{
    append_le_fixed_width, pack_bits_into_bytes, rescue_hash_elements,
};
use crate::zklink_signer::{NEW_PUBKEY_HASH_WIDTH, PACKED_POINT_SIZE};
use franklin_crypto::alt_babyjubjub::FixedGenerators;
use franklin_crypto::eddsa::PublicKey;
use franklin_crypto::jubjub::edwards;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone)]
pub struct PackedPublicKey(pub(crate) EddsaPubkey<Engine>);

impl AsRef<EddsaPubkey<Engine>> for PackedPublicKey {
    fn as_ref(&self) -> &EddsaPubkey<Engine> {
        &self.0
    }
}

impl std::fmt::Debug for PackedPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl From<EddsaPubkey<Engine>> for PackedPublicKey {
    fn from(value: EddsaPubkey<Engine>) -> Self {
        Self(value)
    }
}

impl PackedPublicKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 32 {
            return Err(Error::InvalidPubkey("Public key size mismatch".into()));
        }
        let pubkey = JUBJUB_PARAMS
            .with(|params| edwards::Point::read(bytes, params).map(EddsaPubkey))
            .map_err(|_| Error::invalid_signature("couldn't read public key"))?;
        Ok(Self(pubkey))
    }

    /// Converts private key into a corresponding public key.
    pub fn from_private_key(pk: &PackedPrivateKey) -> PackedPublicKey {
        let pubkey: PackedPublicKey = JUBJUB_PARAMS
            .with(|params| {
                PublicKey::from_private(pk.as_ref(), FixedGenerators::SpendingKeyGenerator, params)
            }).into();
        pubkey
    }

    /// converts public key to byte array
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut pubkey_buf = Vec::with_capacity(PACKED_POINT_SIZE);
        self.as_ref()
            .write(&mut pubkey_buf)
            .expect("failed to write pubkey to buffer");
        let mut pubkey = [0; PACKED_POINT_SIZE];
        pubkey.copy_from_slice(&pubkey_buf);
        pubkey_buf
    }

    /// converts public key to a hex string with the 0x prefix
    pub fn as_hex(&self) -> String {
        let bytes = self.as_bytes();
        format!("0x{}", hex::encode(bytes))
    }

    pub fn public_key_hash(&self) -> PubKeyHash {
        let (pub_x, pub_y) = self.as_ref().0.into_xy();
        let pub_key_hash = rescue_hash_elements(&[pub_x, pub_y]);
        let mut pub_key_hash_bits = Vec::with_capacity(NEW_PUBKEY_HASH_WIDTH);
        append_le_fixed_width(&mut pub_key_hash_bits, &pub_key_hash, NEW_PUBKEY_HASH_WIDTH);
        let mut bytes = pack_bits_into_bytes(&pub_key_hash_bits);
        bytes.reverse();
        PubKeyHash::from_bytes(&bytes).unwrap()
    }
}

impl Serialize for PackedPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.as_hex();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for PackedPublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        let s = s.strip_prefix("0x").unwrap_or(&s);
        let bytes = hex::decode(s).map_err(Error::custom)?;
        Self::from_bytes(&bytes).map_err(Error::custom)
    }
}
