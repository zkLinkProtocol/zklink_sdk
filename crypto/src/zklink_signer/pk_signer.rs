use super::error::ZkSignerError as Error;
use super::private_key::PackedPrivateKey;
use super::{JUBJUB_PARAMS, RESCUE_PARAMS};
use crate::zklink_signer::public_key::PackedPublicKey;
use crate::zklink_signer::signature::{PackedSignature, ZkLinkSignature};
use crate::zklink_signer::utils;
use franklin_crypto::alt_babyjubjub::FixedGenerators;
use franklin_crypto::eddsa::Seed;
use std::fmt;

pub struct ZkLinkSigner {
    private_key: PackedPrivateKey,
    pub public_key: PackedPublicKey,
}

impl fmt::Display for ZkLinkSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZkLinkSigner")
    }
}

impl ZkLinkSigner {
    pub fn new() -> Result<Self, Error> {
        let private_key = PackedPrivateKey::new()?;
        let public_key = PackedPublicKey::from_private_key(&private_key);

        Ok(ZkLinkSigner {
            private_key,
            public_key,
        })
    }

    pub fn new_from_seed(seed: &[u8]) -> Result<Self, Error> {
        let private_key = PackedPrivateKey::new_from_seed(seed)?;
        let public_key = PackedPublicKey::from_private_key(&private_key);

        Ok(ZkLinkSigner {
            private_key,
            public_key,
        })
    }

    pub fn new_from_hex_eth_signer(eth_hex_private_key: &str) -> Result<Self, Error> {
        let private_key = PackedPrivateKey::new_from_hex_eth_signer(eth_hex_private_key)?;
        let public_key = PackedPublicKey::from_private_key(&private_key);

        Ok(ZkLinkSigner {
            private_key,
            public_key,
        })
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let private_key = PackedPrivateKey::new_from_bytes(bytes)?;
        let public_key = PackedPublicKey::from_private_key(&private_key);

        Ok(ZkLinkSigner {
            private_key,
            public_key,
        })
    }

    /// We use musig Schnorr signature scheme.
    /// It is impossible to restore signer for signature, that is why we provide public key of the signer
    /// along with signature.
    pub fn sign_musig(&self, msg: &[u8]) -> Result<ZkLinkSignature, Error> {
        let p_g = FixedGenerators::SpendingKeyGenerator;
        let public_key = self.public_key.clone();
        let signature = JUBJUB_PARAMS.with(|jubjub_params| {
            RESCUE_PARAMS.with(|rescue_params| {
                let hashed_msg = utils::rescue_hash_tx_msg(msg);
                let seed = Seed::deterministic_seed(self.private_key.as_ref(), &hashed_msg);
                self.private_key.as_ref().musig_rescue_sign(
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

    pub fn verify_musig(signature: &ZkLinkSignature, msg: &[u8]) -> Result<bool, Error> {
        signature.verify_musig(msg)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign() {
        let eth_private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(eth_private_key).unwrap();
        let pub_key = zk_signer.public_key.as_hex();
        assert_eq!(
            pub_key,
            "0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510"
        );
        let pub_key_hash = zk_signer.public_key.public_key_hash();
        assert_eq!(
            pub_key_hash.as_hex(),
            "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe"
        );
    }
}
