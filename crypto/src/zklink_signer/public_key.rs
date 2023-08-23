use franklin_crypto::eddsa::PublicKey;
use web3::signing::SigningError;
use super::{EddsaPubkey, Engine};
use crate::zklink_signer::utils::{
    append_le_fixed_width, pack_bits_into_bytes, rescue_hash_elements,
};
use franklin_crypto::alt_babyjubjub::{edwards, fs::FsRepr, FixedGenerators, AltJubjubBn256};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::zklink_signer::{JUBJUB_PARAMS, NEW_PUBKEY_HASH_WIDTH, PACKED_POINT_SIZE};
use crate::zklink_signer::error::ZkSignerError;
use crate::zklink_signer::private_key::PackedPrivateKey;

#[derive(Clone)]
pub struct PackedPublicKey(EddsaPubkey<Engine>);
impl AsRef<EddsaPubkey<Engine>> for PackedPublicKey {
    fn as_ref(&self) -> &EddsaPubkey<Engine> {
        &self.0
    }
}
impl From<EddsaPubkey<Engine>> for PackedPublicKey {
    fn from(value: EddsaPubkey<Engine>) -> Self {
        Self(value)
    }
}

impl PackedPublicKey {

    /// Converts private key into a corresponding public key.
    pub fn from_private_key(pk: &PackedPrivateKey) -> PackedPublicKey {
        PublicKey::from_private(
            pk.as_ref(),
            FixedGenerators::SpendingKeyGenerator,
            &JUBJUB_PARAMS.with(|params| params),
        ).into()
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

    pub fn public_key_hash(&self) -> Vec<u8> {
        let (pub_x, pub_y) = self.as_ref().0.into_xy();
        let pub_key_hash = rescue_hash_elements(&[pub_x, pub_y]);
        let mut pub_key_hash_bits = Vec::with_capacity(NEW_PUBKEY_HASH_WIDTH);
        append_le_fixed_width(&mut pub_key_hash_bits, &pub_key_hash, NEW_PUBKEY_HASH_WIDTH);
        let mut bytes = pack_bits_into_bytes(&pub_key_hash_bits);
        bytes.reverse();
        bytes
    }

    pub fn serialize_packed(&self) -> std::io::Result<Vec<u8>> {
        let mut packed_point = [0u8; 32];
        (self.0).0.write(packed_point.as_mut())?;
        Ok(packed_point.to_vec())
    }

    pub fn deserialize_packed(bytes: &[u8]) -> Result<Self, ZkSignerError> {
        if bytes.len() != 32 {
            return Err(ZkSignerError::custom_error("PublicKey size mismatch"));
        }

        Ok(PackedPublicKey(PublicKey::<Engine>(
            edwards::Point::read(bytes, JUBJUB_PARAMS.with(|params| params))
                .map_err(|e| ZkSignerError::custom_error(format!("Failed to restore point: {}", e.to_string())))?
        )))
    }
}

impl Serialize for PackedPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let packed_point = self.serialize_packed().map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&hex::encode(packed_point))
    }
}

impl<'de> Deserialize<'de> for PackedPublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        let bytes = hex::decode(&string).map_err(Error::custom)?;
        Self::deserialize_packed(&bytes).map_err(Error::custom)
    }
}

