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
use sha2::{Digest, Sha256};
use std::fmt;

#[cfg(feature = "web")]
use crate::eth_signer::json_rpc_signer::JsonRpcSigner;
#[cfg(feature = "web")]
use crate::starknet_signer::starknet_json_rpc_signer::StarknetJsonRpcSigner;
use crate::eth_signer::pk_signer::EthSigner;
use crate::starknet_signer::StarkSigner;
#[cfg(feature = "web")]
use crate::starknet_signer::typed_data::message::{TypedDataMessage, Message};

pub struct ZkLinkSigner(EddsaPrivKey<Engine>);

impl Clone for ZkLinkSigner {
    fn clone(&self) -> Self {
        let pk = EddsaPrivKey(self.0 .0);
        Self(pk)
    }
}

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

impl fmt::Debug for ZkLinkSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "**ZkLinkSigner**")
    }
}

pub fn sha256_bytes(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

impl ZkLinkSigner {
    const SIGN_MESSAGE: &'static str =
        "Sign this message to create a key to interact with zkLink's layer2 services.\nNOTE: This application is powered by zkLink protocol.\n\nOnly sign this message for a trusted client!";
    const STARKNET_SIGN_MESSAGE: &'static str =
        "Create zkLink's layer2 key.\n";
    pub fn new() -> Result<Self, Error> {
        let eth_pk = H256::random();
        let eth_signer = EthSigner::from(eth_pk);
        let signature = eth_signer.sign_message(Self::SIGN_MESSAGE.as_bytes())?;
        let seed = signature.serialize_packed();
        Self::new_from_seed(&seed)
    }

    pub fn new_from_seed(seed: &[u8]) -> Result<Self, Error> {
        if seed.len() < 32 {
            return Err(Error::InvalidSeed("seed is too short".into()));
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
        let eth_signer = EthSigner::try_from(eth_hex_private_key)?;
        let signature = eth_signer.sign_message(Self::SIGN_MESSAGE.as_bytes())?;
        let seed = signature.serialize_packed();
        Self::new_from_seed(&seed)
    }

    pub fn new_from_hex_stark_signer(hex_private_key: &str) -> Result<Self, Error> {
        let stark_signer = StarkSigner::new_from_hex_str(hex_private_key)?;
        Self::new_from_starknet_signer(&stark_signer)
    }

    pub fn new_from_eth_signer(eth_signer: &EthSigner) -> Result<Self, Error> {
        let signature = eth_signer.sign_message(Self::SIGN_MESSAGE.as_bytes())?;
        let seed = signature.serialize_packed();
        Self::new_from_seed(&seed)
    }

    /// create zkLink signer from starknet signer
    pub fn new_from_starknet_signer(starknet_signer: &StarkSigner) -> Result<Self, Error> {
        let signature = starknet_signer.sign_message(Self::SIGN_MESSAGE.as_bytes())?;
        let seed = signature.to_bytes_be();
        Self::new_from_seed(&seed)
    }

    #[cfg(feature = "web")]
    pub async fn new_from_eth_rpc_signer(eth_signer: &JsonRpcSigner) -> Result<Self, Error> {
        let signature = eth_signer
            .sign_message(Self::SIGN_MESSAGE.as_bytes())
            .await?;
        let seed = signature.serialize_packed();
        Self::new_from_seed(&seed)
    }

    #[cfg(feature = "web")]
    pub async fn new_from_starknet_rpc_signer(starknet_signer: &StarknetJsonRpcSigner) -> Result<Self, Error> {
        let message = TypedDataMessage::CreateL2Key(Message {
            data: Self::STARKNET_SIGN_MESSAGE.to_string()
        });
        let signature = starknet_signer
            .sign_message(message)
            .await?;
        let seed = signature.to_bytes_be();
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
            pub_key: public_key,
            signature: PackedSignature(signature),
        };
        Ok(signature)
    }

    fn from_fs(fs: Fs) -> Self {
        PrivateKey(fs).into()
    }

    pub fn public_key(&self) -> PackedPublicKey {
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
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zklink_signer() {
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
        let zk_signer2 = zk_signer.clone();
        let pub_key_hash2 = zk_signer2.public_key().public_key_hash();
        assert_eq!(pub_key_hash.as_hex(), pub_key_hash2.as_hex());

        let a = "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
        let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(a).unwrap();
        let pub_key2 = zklink_signer.public_key().as_hex();
        assert_eq!(
            "0x8e3eb3abb0cbf96605956a5313ab239ff685a64562332ac52ef51b9eb8d0d72c",
            pub_key2
        );

        let message = b"hello world";
        let signature = zklink_signer.sign_musig(message).unwrap();
        let passed = signature.verify_musig(message);

        assert!(passed);

        let expect_signature = serde_json::json!(
            {
                "pubKey": "0x8e3eb3abb0cbf96605956a5313ab239ff685a64562332ac52ef51b9eb8d0d72c",
                "signature": "e396adddbd484e896d0eea6b248a339a0497f65d482112981d947fd71010c4022a40cc5a72b334e89a1601f71518dcaa05c56737e1647828fa822e94b1ff7501"
            }
        );
        assert_eq!(serde_json::to_value(signature).unwrap(), expect_signature);
    }
}
