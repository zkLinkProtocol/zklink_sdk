use crate::crypto::{utils, Signature, PACKED_POINT_SIZE, SIGNATURE_SIZE};

use crate::crypto::error::Error;
use crate::crypto::zklink_signer::ZkLinkSigner;
use crate::crypto::JUBJUB_PARAMS;
use crate::crypto::RESCUE_PARAMS;
use franklin_crypto::alt_babyjubjub::{edwards, fs::FsRepr, FixedGenerators};
use franklin_crypto::bellman::pairing::bn256::Bn256 as Engine;
use franklin_crypto::bellman::pairing::ff::{PrimeField, PrimeFieldRepr};
use franklin_crypto::eddsa::{PublicKey, Seed};
use franklin_crypto::jubjub::JubjubEngine;

/// ZkLink signature
/// [0..32] - packed public key of signer.
/// [32..64] - packed r point of the signature.
/// [64..96] - s poing of the signature.
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

    /// We use musig Schnorr signature scheme.
    /// It is impossible to restore signer for signature, that is why we provide public key of the signer
    /// along with signature.
    pub fn sign_musig(zk_signer: &ZkLinkSigner, msg: &[u8]) -> Result<ZkLinkSignature, Error> {
        let mut packed_full_signature = Vec::with_capacity(SIGNATURE_SIZE);
        let p_g = FixedGenerators::SpendingKeyGenerator;
        let private_key = zk_signer.private_key()?;
        let public_key = zk_signer.public_key()?;
        public_key
            .write(&mut packed_full_signature)
            .expect("failed to write pubkey to packed_point");

        let signature = JUBJUB_PARAMS.with(|jubjub_params| {
            RESCUE_PARAMS.with(|rescue_params| {
                let hashed_msg = utils::rescue_hash_tx_msg(msg);
                let seed = Seed::deterministic_seed(&private_key, &hashed_msg);
                private_key.musig_rescue_sign(&hashed_msg, &seed, p_g, rescue_params, jubjub_params)
            })
        });

        signature
            .r
            .write(&mut packed_full_signature)
            .expect("failed to write signature");
        signature
            .s
            .into_repr()
            .write_le(&mut packed_full_signature)
            .expect("failed to write signature repr");

        assert_eq!(
            packed_full_signature.len(),
            SIGNATURE_SIZE,
            "incorrect signature size when signing"
        );
        let mut inner = [0; SIGNATURE_SIZE];
        inner.copy_from_slice(&packed_full_signature);
        Ok(Self(inner))
    }

    pub fn verify_musig(&self, msg: &[u8]) -> Result<bool, Error> {
        let pubkey = &self.0[..PACKED_POINT_SIZE];
        let pubkey = JUBJUB_PARAMS
            .with(|params| edwards::Point::read(pubkey, params).map(PublicKey))
            .map_err(|_| Error::invalid_signature("couldn't read public key"))?;

        let signature = self.signature()?;

        let msg = utils::rescue_hash_tx_msg(msg);
        let value = JUBJUB_PARAMS.with(|jubjub_params| {
            RESCUE_PARAMS.with(|rescue_params| {
                pubkey.verify_musig_rescue(
                    &msg,
                    &signature,
                    FixedGenerators::SpendingKeyGenerator,
                    rescue_params,
                    jubjub_params,
                )
            })
        });

        Ok(value)
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

        Ok(Signature { r, s })
    }
}

#[cfg(test)]
mod test {
    use eth_signer::H256;
    use super::*;

    #[test]
    fn test_signature() {
        let eth_private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let eth_pk = H256::from_slice(&hex::decode(eth_private_key).unwrap());
        let zk_signer = ZkLinkSigner::new_from_eth_signer(&eth_pk).unwrap();
        let msg = b"hello world";
        let signature = ZkLinkSignature::sign_musig(&zk_signer, msg).unwrap();
        let verify = signature.verify_musig(msg).unwrap();
        assert!(verify);
    }
}
