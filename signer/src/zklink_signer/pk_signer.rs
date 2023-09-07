use super::error::ZkSignerError as Error;
use super::{JUBJUB_PARAMS, RESCUE_PARAMS};

use crate::eth_signer::H256;
use crate::zklink_signer::public_key::PackedPublicKey;
use crate::zklink_signer::signature::{PackedSignature, ZkLinkSignature};
use crate::zklink_signer::utils;
use crate::zklink_signer::{EddsaPrivKey, Engine};
use franklin_crypto::alt_babyjubjub::fs::{Fs, FsRepr};
use franklin_crypto::alt_babyjubjub::FixedGenerators;
use franklin_crypto::bellman::{PrimeField, PrimeFieldRepr};
use franklin_crypto::eddsa::{PrivateKey as FLPrivateKey, PrivateKey, PublicKey, Seed};
use once_cell::sync::OnceCell;
use sha2::{Digest, Sha256};
use std::fmt;

use crate::eth_signer::pk_signer::PrivateKeySigner;

pub struct ZkLinkSigner(EddsaPrivKey<Engine>);

impl AsRef<EddsaPrivKey<Engine>> for ZkLinkSigner {
    fn as_ref(&self) -> &EddsaPrivKey<Engine> {
        &self.0
    }
}

impl From<EddsaPrivKey<Engine>> for ZkLinkSigner {
    fn from(value: EddsaPrivKey<Engine>) -> Self {
        Self(value)
    }
}

impl fmt::Display for ZkLinkSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZkLinkSigner")
    }
}

impl ZkLinkSigner {
    const SIGN_MESSAGE: &'static str =
        "Sign this message to create a key to interact with zkLink's layer2 services.\nNOTE: This application is powered by zkLink protocol.\n\nOnly sign this message for a trusted client!";
    pub fn new() -> Result<Self, Error> {
        let eth_pk = H256::random();
        let eth_signer = PrivateKeySigner::from(&eth_pk);
        let signature = eth_signer.sign_message(Self::SIGN_MESSAGE.as_bytes())?;
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
                    return Ok(Self::from_fs(fs));
                }
                Err(_) => {
                    effective_seed = raw_priv_key;
                }
            }
        }
    }

    pub fn new_from_hex_eth_signer(eth_hex_private_key: &str) -> Result<Self, Error> {
        let eth_signer = PrivateKeySigner::try_from(eth_hex_private_key)?;
        let signature = eth_signer.sign_message(Self::SIGN_MESSAGE.as_bytes())?;
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

    /// We use musig Schnorr signature scheme.
    /// It is impossible to restore signer for signature, that is why we provide public key of the signer
    /// along with signature.
    ///
    pub fn sign_musig(&self, msg: &[u8]) -> Result<ZkLinkSignature, Error> {
        let p_g = FixedGenerators::SpendingKeyGenerator;
        let public_key = self.public_key();
        let signature = JUBJUB_PARAMS.with(|jubjub_params| {
            RESCUE_PARAMS.with(|rescue_params| {
                let hashed_msg = utils::rescue_hash_tx_msg(msg);
                let seed = Seed::deterministic_seed(self.as_ref(), &hashed_msg);
                self.as_ref().musig_rescue_sign(
                    hashed_msg.as_slice(),
                    &seed,
                    p_g,
                    rescue_params,
                    jubjub_params,
                )
            })
        });
        let signature = ZkLinkSignature {
            public_key,
            signature: PackedSignature(signature),
        };
        Ok(signature)
    }

    fn from_fs(fs: Fs) -> Self {
        PrivateKey(fs).into()
    }

    pub fn public_key(&self) -> PackedPublicKey {
        static INSTANCE: OnceCell<PackedPublicKey> = OnceCell::new();
        let pubkey = INSTANCE.get_or_init(|| {
            let pubkey: PackedPublicKey = JUBJUB_PARAMS
                .with(|params| {
                    PublicKey::from_private(
                        self.as_ref(),
                        FixedGenerators::SpendingKeyGenerator,
                        params,
                    )
                })
                .into();
            pubkey
        });
        pubkey.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign() {
        let eth_private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(eth_private_key).unwrap();
        let pub_key = zk_signer.public_key().as_hex();
        assert_eq!(
            pub_key,
            "0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510"
        );
        let pub_key_hash = zk_signer.public_key().public_key_hash();
        assert_eq!(
            pub_key_hash.as_hex(),
            "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe"
        );
    }
}
