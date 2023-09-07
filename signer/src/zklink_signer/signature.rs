use super::error::ZkSignerError as Error;
use super::JUBJUB_PARAMS;
use super::RESCUE_PARAMS;
use super::{utils, EddsaSignature, PACKED_POINT_SIZE, SIGNATURE_SIZE};
use crate::zklink_signer::public_key::PackedPublicKey;
use franklin_crypto::alt_babyjubjub::{edwards, fs::FsRepr, FixedGenerators};
use franklin_crypto::bellman::pairing::bn256::Bn256 as Engine;
use franklin_crypto::bellman::pairing::ff::{PrimeField, PrimeFieldRepr};
use franklin_crypto::jubjub::JubjubEngine;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone)]
pub struct PackedSignature(pub(crate) EddsaSignature<Engine>);

impl AsRef<EddsaSignature<Engine>> for PackedSignature {
    fn as_ref(&self) -> &EddsaSignature<Engine> {
        &self.0
    }
}

impl std::fmt::Debug for PackedSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl From<EddsaSignature<Engine>> for PackedSignature {
    fn from(value: EddsaSignature<Engine>) -> Self {
        Self(value)
    }
}

impl Serialize for PackedSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.as_hex();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for PackedSignature {
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

impl PackedSignature {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != PACKED_POINT_SIZE * 2 {
            return Err(Error::InvalidSignature("size mismatch".into()));
        }
        let r_bar = &bytes[0..PACKED_POINT_SIZE];
        let s_bar = &bytes[PACKED_POINT_SIZE..];

        let r = JUBJUB_PARAMS
            .with(|params| edwards::Point::read(r_bar, params))
            .map_err(|_| Error::invalid_signature("Failed to parse signature"))?;

        let mut s_repr = FsRepr::default();
        s_repr
            .read_le(s_bar)
            .map_err(|_| Error::invalid_signature("Failed to parse signature"))?;

        let s = <Engine as JubjubEngine>::Fs::from_repr(s_repr)
            .map_err(|_| Error::invalid_signature("Failed to parse signature"))?;
        let s = EddsaSignature::<Engine> { r, s };
        Ok(s.into())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut packed_signature = [0u8; 64];
        let (r_bar, s_bar) = packed_signature.as_mut().split_at_mut(32);

        (self.0).r.write(r_bar).expect("io error");
        (self.0).s.into_repr().write_le(s_bar).expect("io error");

        packed_signature.to_vec()
    }

    pub fn as_hex(&self) -> String {
        let bytes = self.as_bytes();
        format!("0x{}", hex::encode(bytes))
    }

    pub fn from_hex(s: &str) -> Result<Self, Error> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let raw = hex::decode(s)
            .map_err(|_e| Error::InvalidSignature("can't convert string to bytes".into()))?;
        Self::from_bytes(&raw)
    }
}

/// ZkLink signature
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ZkLinkSignature {
    /// packed public key
    pub public_key: PackedPublicKey,
    /// packed signature
    pub signature: PackedSignature,
}

impl Default for ZkLinkSignature {
    fn default() -> Self {
        Self {
            public_key: PackedPublicKey::from_bytes(&[0; 32]).unwrap(),
            signature: PackedSignature::from_bytes(&[0; 64]).unwrap(),
        }
    }
}
impl ZkLinkSignature {
    /// Create a ZkLinkSignature from u8 slice
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != SIGNATURE_SIZE {
            return Err(Error::InvalidSignature("Signature length is not 96 bytes. Make sure it contains both the public key and the signature itself.".into()));
        }
        Ok(Self {
            public_key: PackedPublicKey::from_bytes(&bytes[0..PACKED_POINT_SIZE])?,
            signature: PackedSignature::from_bytes(&bytes[PACKED_POINT_SIZE..])?,
        })
    }

    /// Create a ZkLinkSignature from hex string which starts with 0x or not
    pub fn from_hex(s: &str) -> Result<Self, Error> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let raw = hex::decode(s)
            .map_err(|_| Error::InvalidSignature("invalid signature string".into()))?;
        Self::from_bytes(&raw)
    }

    /// converts signature to a hex string with the 0x prefix
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(SIGNATURE_SIZE);
        let public_bytes = self.public_key.as_bytes();
        let signature_bytes = self.signature.as_bytes();
        bytes[0..PACKED_POINT_SIZE].copy_from_slice(&public_bytes);
        bytes[PACKED_POINT_SIZE..].copy_from_slice(&signature_bytes);
        bytes
    }

    /// converts signature to a hex string with the 0x prefix
    pub fn as_hex(&self) -> String {
        let bytes = self.as_bytes();
        format!("0x{}", hex::encode(bytes))
    }

    pub fn verify_musig(&self, msg: &[u8]) -> Result<bool, Error> {
        let msg = utils::rescue_hash_tx_msg(msg);
        let value = JUBJUB_PARAMS.with(|jubjub_params| {
            RESCUE_PARAMS.with(|rescue_params| {
                self.public_key.as_ref().verify_musig_rescue(
                    &msg,
                    self.signature.as_ref(),
                    FixedGenerators::SpendingKeyGenerator,
                    rescue_params,
                    jubjub_params,
                )
            })
        });

        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use crate::zklink_signer::pk_signer::ZkLinkSigner;

    #[test]
    fn test_signature() {
        let eth_private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(eth_private_key).unwrap();
        let msg = b"hello world";
        let signature = zk_signer.sign_musig(msg).unwrap();
        let verify = signature.verify_musig(msg).unwrap();
        assert!(verify);
    }
}
