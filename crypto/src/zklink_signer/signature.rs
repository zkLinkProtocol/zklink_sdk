use super::error::ZkSignerError as Error;
use super::JUBJUB_PARAMS;
use super::RESCUE_PARAMS;
use super::{utils, EddsaSignature, PACKED_POINT_SIZE, SIGNATURE_SIZE};
use franklin_crypto::alt_babyjubjub::{edwards, fs::FsRepr, FixedGenerators};
use franklin_crypto::bellman::pairing::bn256::Bn256 as Engine;
use franklin_crypto::bellman::pairing::ff::{PrimeField, PrimeFieldRepr};
use franklin_crypto::eddsa::PublicKey;
use franklin_crypto::jubjub::JubjubEngine;

pub struct Signature(EddsaSignature<Engine>);
impl AsRef<EddsaSignature<Engine>> for Signature {
    fn as_ref(&self) -> &EddsaSignature<Engine> {
        &self.0
    }
}

impl From<EddsaSignature<Engine>> for Signature {
    fn from(value: EddsaSignature<Engine>) -> Self {
        Self(value)
    }
}

/// ZkLink signature
/// [0..32] - packed public key of signer.
/// [32..64] - packed r point of the signature.
/// [64..96] - s point of the signature.
pub struct ZkLinkSignature(pub [u8; SIGNATURE_SIZE]);

impl ZkLinkSignature {
    /// Create a ZkLinkSignature from u8 slice
    pub fn new_from_slice(slice: &[u8]) -> Result<Self, Error> {
        if slice.len() != SIGNATURE_SIZE {
            return Err(Error::InvalidSignature("Signature length is not 96 bytes. Make sure it contains both the public key and the signature itself.".into()));
        }
        let mut raw = [0; SIGNATURE_SIZE];
        raw.copy_from_slice(slice);
        Ok(Self(raw))
    }

    /// Create a ZkLinkSignature from hex string which starts with 0x or not
    pub fn from_hex(s: &str) -> Result<Self, Error> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let raw = hex::decode(s).map_err(|_|Error::InvalidSignature("invalid signature string".into()))?;
        Self::new_from_slice(&raw)
    }

    /// converts signature to a hex string with the 0x prefix
    pub fn as_hex(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }

    pub fn public_key(&self) -> Result<PublicKey<Engine>, Error> {
        let pubkey = &self.0[..PACKED_POINT_SIZE];
        let pubkey = JUBJUB_PARAMS
            .with(|params| edwards::Point::read(pubkey, params).map(PublicKey))
            .map_err(|_| Error::invalid_signature("couldn't read public key"))?;
        Ok(pubkey)
    }

    fn signature(&self) -> Result<Signature, Error> {
        let r_bar = &self.0[PACKED_POINT_SIZE..PACKED_POINT_SIZE * 2];
        let s_bar = &self.0[PACKED_POINT_SIZE * 2..];

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

    pub fn verify_musig(&self, msg: &[u8]) -> Result<bool, Error> {
        let pubkey = self.public_key()?;
        let signature = self.signature()?;

        let msg = utils::rescue_hash_tx_msg(msg);
        let value = JUBJUB_PARAMS.with(|jubjub_params| {
            RESCUE_PARAMS.with(|rescue_params| {
                pubkey.verify_musig_rescue(
                    &msg,
                    signature.as_ref(),
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
    use crate::zklink_signer::ZkLinkSigner;

    #[test]
    fn test_signature() {
        let eth_private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(&eth_private_key).unwrap();
        let msg = b"hello world";
        let signature = zk_signer.sign_musig(msg).unwrap();
        let verify = signature.verify_musig(msg).unwrap();
        assert!(verify);
    }
}
