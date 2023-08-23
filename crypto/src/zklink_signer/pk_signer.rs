use super::error::ZkSignerError as Error;
use super::{EddsaPubkey, Engine, JUBJUB_PARAMS, RESCUE_PARAMS};
use super::private_key::PackedPrivateKey;
use crate::eth_signer::packed_eth_signature::PackedEthSignature;
use crate::eth_signer::H256;
use crate::zklink_signer::public_key::PackedPublicKey;
use crate::zklink_signer::signature::{PackedSignature, ZkLinkSignature};
use crate::zklink_signer::utils;
use franklin_crypto::alt_babyjubjub::fs::FsRepr;
use franklin_crypto::alt_babyjubjub::FixedGenerators;
use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::PrimeFieldRepr;
use franklin_crypto::eddsa::PrivateKey as FLPrivateKey;
use franklin_crypto::eddsa::Seed;
use franklin_crypto::jubjub::JubjubEngine;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::fmt;
use zklink_sdk_utils::serde::ZeroPrefixHexSerde;

type Fs = <Engine as JubjubEngine>::Fs;

#[derive(Debug, Clone, PartialEq)]
pub struct ZkLinkSigner(Vec<u8>);

impl fmt::Display for ZkLinkSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZkLinkSigner")
    }
}

impl<'de> Deserialize<'de> for ZkLinkSigner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = ZeroPrefixHexSerde::deserialize(deserializer)?;
        Ok(Self(bytes))
    }
}

impl Serialize for ZkLinkSigner {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ZeroPrefixHexSerde::serialize(&self.0, serializer)
    }
}

impl ZkLinkSigner {
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
            if Fs::from_repr(fs_repr).is_ok() {
                return Ok(Self(raw_priv_key));
            } else {
                effective_seed = raw_priv_key;
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

    pub fn new_from_slice(slice: &[u8]) -> Result<Self, Error> {
        let s = Self(slice.to_vec());
        let _private_key = s.private_key()?;
        Ok(s)
    }

    pub fn private_key(&self) -> Result<PackedPrivateKey, Error> {
        let mut fs_repr = FsRepr::default();
        fs_repr
            .read_be(&*self.0)
            .map_err(|_| Error::custom_error("couldn't read private key repr"))?;
        let private_key = FLPrivateKey::<Engine>(
            Fs::from_repr(fs_repr).expect("couldn't read private key from repr"),
        );
        Ok(private_key.into())
    }

    pub fn get_public_key(&self) -> Result<PackedPublicKey, Error> {
        let p_g = FixedGenerators::SpendingKeyGenerator;
        let private_key = self.private_key()?;
        let public_key = JUBJUB_PARAMS
            .with(|params| EddsaPubkey::<Engine>::from_private(private_key.as_ref(), p_g, params));
        Ok(public_key.into())
    }

    /// We use musig Schnorr signature scheme.
    /// It is impossible to restore signer for signature, that is why we provide public key of the signer
    /// along with signature.
    pub fn sign_musig(&self, msg: &[u8]) -> Result<ZkLinkSignature, Error> {
        let p_g = FixedGenerators::SpendingKeyGenerator;
        let private_key = self.private_key()?;
        let public_key = self.get_public_key()?;
        let signature = JUBJUB_PARAMS.with(|jubjub_params| {
            RESCUE_PARAMS.with(|rescue_params| {
                let hashed_msg = utils::rescue_hash_tx_msg(msg);
                let seed = Seed::deterministic_seed(private_key.as_ref(), &hashed_msg);
                private_key.as_ref().musig_rescue_sign(
                    &hashed_msg,
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign() {
        let eth_private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(eth_private_key).unwrap();
        let pub_key = zk_signer.get_public_key().unwrap().as_bytes();
        assert_eq!(
            hex::encode(&pub_key),
            "7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510"
        );
        let pub_key_hash = zk_signer.get_public_key().unwrap().public_key_hash();
        assert_eq!(
            pub_key_hash.as_hex(),
            "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe"
        );
    }
}
