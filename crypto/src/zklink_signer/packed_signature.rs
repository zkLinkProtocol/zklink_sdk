use franklin_crypto::alt_babyjubjub::{AltJubjubBn256, edwards, JubjubEngine};
use franklin_crypto::alt_babyjubjub::fs::FsRepr;
use franklin_crypto::bellman::{PrimeField, PrimeFieldRepr};
use franklin_crypto::eddsa::Signature;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::zklink_signer::{EddsaSignature, Engine, JUBJUB_PARAMS};
use crate::zklink_signer::error::ZkSignerError;

#[derive(Clone)]
pub struct PackedSignature(EddsaSignature<Engine>);
impl AsRef<EddsaSignature<Engine>> for PackedSignature {
    fn as_ref(&self) -> &EddsaSignature<Engine> {
        &self.0
    }
}

impl From<EddsaSignature<Engine>> for PackedSignature {
    fn from(value: EddsaSignature<Engine>) -> Self {
        Self(value)
    }
}

impl PackedSignature {

    pub fn serialize_packed(&self) -> std::io::Result<Vec<u8>> {
        let mut packed_signature = [0u8; 64];
        let (r_bar, s_bar) = packed_signature.as_mut().split_at_mut(32);

        (self.0).r.write(r_bar)?;
        (self.0).s.into_repr().write_le(s_bar)?;

        Ok(packed_signature.to_vec())
    }

    pub fn deserialize_packed(bytes: &[u8]) -> Result<Self, ZkSignerError> {
        if bytes.len() != 64 {
            return Err(ZkSignerError::CustomError("Signature size mismatch".into()));
        }

        let (r_bar, s_bar) = bytes.split_at(32);

        let r = edwards::Point::read(r_bar, JUBJUB_PARAMS.with(|t| t))
            .map_err(|e| ZkSignerError::custom_error(format!("Failed to restore R point from R_bar: {}", e.to_string())))?;

        let mut s_repr = FsRepr::default();
        s_repr
            .read_le(s_bar)
            .map_err(|e| ZkSignerError::custom_error(format!("Failed to restore R point from R_bar: {}", e.to_string())))?;

        let s = <Engine as JubjubEngine>::Fs::from_repr(s_repr)
            .map_err(|e| ZkSignerError::custom_error(format!("Failed to restore R point from R_bar: {}", e.to_string())))?;

        Ok(Self(Signature { r, s }.into()))
    }

}


impl Serialize for PackedSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let packed_signature = self.serialize_packed().map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&hex::encode(&packed_signature))
    }
}

impl<'de> Deserialize<'de> for PackedSignature {
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
