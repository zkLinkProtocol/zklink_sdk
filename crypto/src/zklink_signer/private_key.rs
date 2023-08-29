use std::sync::Arc;
use super::error::ZkSignerError as Error;
use crate::eth_signer::packed_eth_signature::PackedEthSignature;
use crate::zklink_signer::{JUBJUB_PARAMS, EddsaPrivKey, Engine};
use franklin_crypto::alt_babyjubjub::fs::{Fs, FsRepr};
use franklin_crypto::bellman::{PrimeField, PrimeFieldRepr};
use franklin_crypto::eddsa::{PrivateKey as FLPrivateKey, PrivateKey};
use franklin_crypto::jubjub::FixedGenerators;
use franklin_crypto::eddsa::PublicKey;
use sha2::{Digest, Sha256};
use web3::types::H256;
use crate::zklink_signer::public_key::PackedPublicKey;

pub struct PackedPrivateKey(EddsaPrivKey<Engine>);

impl AsRef<EddsaPrivKey<Engine>> for PackedPrivateKey {
    fn as_ref(&self) -> &EddsaPrivKey<Engine> {
        &self.0
    }
}

impl From<EddsaPrivKey<Engine>> for PackedPrivateKey {
    fn from(value: EddsaPrivKey<Engine>) -> Self {
        Self(value)
    }
}

impl PackedPrivateKey {
    const SIGN_MESSAGE: &'static str =
        "Sign this message to create a key to interact with zkLink's layer2 services.\nNOTE: This application is powered by zkLink protocol.\n\nOnly sign this message for a trusted client!";

    pub fn new() -> Result<Self, Error> {
        let eth_pk = H256::random();
        let signature = PackedEthSignature::sign(&eth_pk, Self::SIGN_MESSAGE.as_bytes())?;
        let seed = signature.serialize_packed();
        Self::new_from_seed(&seed)
    }

    pub fn new_from_seed(seed: &[u8]) -> Result<Self, Error> {
        if seed.len() < 32 {
            return Err(Error::InvalidSeed("seed is too short".into()));
        };

        let sha256_bytes = |input: &[u8]| -> Vec<u8> {
            let mut hasher = Sha256::new();
            hasher.update(input);
            hasher.finalize().to_vec()
        };

        let mut effective_seed = sha256_bytes(seed);

        loop {
            let raw_priv_key = sha256_bytes(effective_seed.as_slice());
            let mut fs_repr = FsRepr::default();
            fs_repr
                .read_be(&raw_priv_key[..])
                .expect("failed to read raw_priv_key");
            match Fs::from_repr(fs_repr) {
                Ok(fs) => {
                    return Ok(PackedPrivateKey::from_fs(fs));
                }
                Err(_) => {
                    effective_seed = raw_priv_key;
                }
            }
        }
    }

    pub fn new_from_hex_eth_signer(eth_hex_private_key: &str) -> Result<Self, Error> {
        let eth_private_key = eth_hex_private_key
            .strip_prefix("0x")
            .unwrap_or(eth_hex_private_key);
        let hex_privkey = hex::decode(eth_private_key)
            .map_err(|_| Error::invalid_privkey("invalid eth private key"))?;
        let eth_pk = H256::from_slice(&hex_privkey);
        let signature = PackedEthSignature::sign(&eth_pk, Self::SIGN_MESSAGE.as_bytes())?;
        let seed = signature.serialize_packed();
        Self::new_from_seed(&seed)
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut fs_repr = FsRepr::default();
        fs_repr
            .read_be(bytes)
            .map_err(|_| Error::custom_error("couldn't read private key repr"))?;
        let private_key = FLPrivateKey::<Engine>(
            Fs::from_repr(fs_repr).expect("couldn't read private key from repr"),
        );
        Ok(private_key.into())
    }

    /// Decodes a private key from a field element.
    fn from_fs(fs: Fs) -> PackedPrivateKey {
        PrivateKey(fs).into()
    }
}
