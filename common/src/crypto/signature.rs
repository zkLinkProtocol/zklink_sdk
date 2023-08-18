use crate::crypto::zklink_private_key::privkey_from_slice;
use crate::crypto::{utils, PACKED_POINT_SIZE, PACKED_SIGNATURE_SIZE};

use crate::crypto::error::Error;
use crate::crypto::JUBJUB_PARAMS;
use crate::crypto::RESCUE_PARAMS;
use franklin_crypto::alt_babyjubjub::{edwards, fs::FsRepr, FixedGenerators};
pub use franklin_crypto::bellman::pairing::bn256::{Bn256 as Engine, Fr};
use franklin_crypto::bellman::pairing::ff::{PrimeField, PrimeFieldRepr};
use franklin_crypto::eddsa::{PublicKey, Seed, Signature as EddsaSignature};
use franklin_crypto::jubjub::JubjubEngine;

pub type Signature = EddsaSignature<Engine>;

/// We use musig Schnorr signature scheme.
/// It is impossible to restore signer for signature, that is why we provide public key of the signer
/// along with signature.
/// [0..32] - packed public key of signer.
/// [32..64] - packed r point of the signature.
/// [64..96] - s poing of the signature.
pub fn sign_musig(private_key: &[u8], msg: &[u8]) -> Result<Vec<u8>, Error> {
    let mut packed_full_signature = Vec::with_capacity(PACKED_POINT_SIZE + PACKED_SIGNATURE_SIZE);
    let p_g = FixedGenerators::SpendingKeyGenerator;
    let private_key = privkey_from_slice(private_key)?;

    {
        let public_key =
            JUBJUB_PARAMS.with(|params| PublicKey::from_private(&private_key, p_g, params));
        public_key
            .write(&mut packed_full_signature)
            .expect("failed to write pubkey to packed_point");
    };

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
        PACKED_POINT_SIZE + PACKED_SIGNATURE_SIZE,
        "incorrect signature size when signing"
    );

    Ok(packed_full_signature)
}

pub fn verify_musig(msg: &[u8], signature: &[u8]) -> Result<bool, Error> {
    if signature.len() != PACKED_POINT_SIZE + PACKED_SIGNATURE_SIZE {
        return Err(Error::InvalidSignature("Signature length is not 96 bytes. Make sure it contains both the public key and the signature itself.".into()));
    }

    let pubkey = &signature[..PACKED_POINT_SIZE];
    let pubkey = JUBJUB_PARAMS
        .with(|params| edwards::Point::read(pubkey, params).map(PublicKey))
        .map_err(|_| Error::common("couldn't read public key"))?;

    let signature = deserialize_signature(&signature[PACKED_POINT_SIZE..])?;

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

fn deserialize_signature(bytes: &[u8]) -> Result<Signature, Error> {
    let (r_bar, s_bar) = bytes.split_at(PACKED_POINT_SIZE);

    let r = JUBJUB_PARAMS
        .with(|params| edwards::Point::read(r_bar, params))
        .map_err(|_| Error::InvalidSignature("Failed to parse signature".into()))?;

    let mut s_repr = FsRepr::default();
    s_repr
        .read_le(s_bar)
        .map_err(|_| Error::InvalidSignature("Failed to parse signature".into()))?;

    let s = <Engine as JubjubEngine>::Fs::from_repr(s_repr)
        .map_err(|_| Error::InvalidSignature("Failed to parse signature".into()))?;

    Ok(Signature { r, s })
}
