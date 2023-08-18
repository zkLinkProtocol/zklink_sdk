use crate::crypto::error::Error;
use crate::crypto::{utils, Engine, Fs, JUBJUB_PARAMS, PACKED_POINT_SIZE};
use ethers::core::k256::ecdsa::{Signature as EcdsaSignature, SigningKey};
use ethers::prelude::k256::ecdsa::RecoveryId;
use franklin_crypto::alt_babyjubjub::fs::FsRepr;
use franklin_crypto::alt_babyjubjub::FixedGenerators;
use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::PrimeFieldRepr;
use franklin_crypto::eddsa::PrivateKey as FLPrivateKey;
use franklin_crypto::eddsa::PublicKey as FlPublickKey;
use sha2::{Digest, Sha256};
use sha3::Keccak256;

pub type PrivateKey = FLPrivateKey<Engine>;
pub type PublicKey = FlPublickKey<Engine>;

const SIGN_MESSAGE: &str =
    "Sign this message to create a key to interact with zkLink's layer2 services.\nNOTE: This application is powered by zkLink protocol.\n\nOnly sign this message for a trusted client!";

fn sign_hash(data: &str) -> Vec<u8> {
    let hash = format!("\x19Ethereum Signed Message:\n{}{data}", data.len());
    hash.as_bytes().to_vec()
}

fn person_sign(sign_key: &SigningKey, msg: &str) -> Result<(Vec<u8>, u8), Error> {
    let hash = sign_hash(msg);
    let hash_digest = Keccak256::new_with_prefix(hash);
    let (signature, recover_id): (EcdsaSignature, RecoveryId) = sign_key
        .sign_digest_recoverable(hash_digest)
        .map_err(|e| Error::InvalidPrivKey(e.to_string()))?;
    Ok((signature.to_vec(), recover_id.to_byte()))
}

/// https://stackoverflow.com/questions/69762108/implementing-ethereum-personal-sign-eip-191-from-go-ethereum-gives-different-s
fn privkey_seed(eth_hex_private_key: &str) -> Result<Vec<u8>, Error> {
    let raw_sign_key: [u8; 32] = hex::decode(eth_hex_private_key)
        .map_err(|e| Error::InvalidPrivKey(e.to_string()))?
        .try_into()
        .map_err(|_| Error::InvalidPrivKey("raw eth private key should be 32 length u8 array".into()))?;
    let sign_key = SigningKey::from_slice(&raw_sign_key)
        .map_err(|e| Error::InvalidPrivKey(e.to_string()))?;
    let (mut signature, id) = person_sign(&sign_key, SIGN_MESSAGE)?;
    signature.push(id + 27);
    Ok(signature)
}

/// create the private key from seed
pub fn private_key_from_seed(seed: &[u8]) -> Result<Vec<u8>, Error> {
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
        let raw_priv_key = sha256_bytes(&effective_seed);
        let mut fs_repr = FsRepr::default();
        fs_repr
            .read_be(&raw_priv_key[..])
            .expect("failed to read raw_priv_key");
        if Fs::from_repr(fs_repr).is_ok() {
            return Ok(raw_priv_key);
        } else {
            effective_seed = raw_priv_key;
        }
    }
}

/// Zklink private key from the hex formatted eth private key
pub fn privkey_from_eth_privkey(eth_hex_private_key: &str) -> Result<Vec<u8>, Error> {
    let seed = privkey_seed(eth_hex_private_key)?;
    private_key_from_seed(&seed)
}

/// Zklink private key from a raw scalar serialized as a byte slice
pub fn privkey_from_slice(private_key: &[u8]) -> Result<PrivateKey, Error> {
    let mut fs_repr = FsRepr::default();
    fs_repr
        .read_be(private_key)
        .map_err(|_| Error::common("couldn't read private key repr"))?;
    let private_key = FLPrivateKey::<Engine>(
        Fs::from_repr(fs_repr).expect("couldn't read private key from repr"),
    );
    Ok(private_key)
}

fn privkey_to_pubkey_internal(private_key: &[u8]) -> Result<PublicKey, Error> {
    let p_g = FixedGenerators::SpendingKeyGenerator;
    let sk = privkey_from_slice(private_key)?;
    Ok(JUBJUB_PARAMS.with(|params| PublicKey::from_private(&sk, p_g, params)))
}

/// get the public key from private key
pub fn privkey_to_pubkey(private_key: &[u8]) -> Result<Vec<u8>, Error> {
    let mut pubkey_buf = Vec::with_capacity(PACKED_POINT_SIZE);

    let pubkey = privkey_to_pubkey_internal(private_key)?;

    pubkey
        .write(&mut pubkey_buf)
        .expect("failed to write pubkey to buffer");

    Ok(pubkey_buf)
}

/// get the public key hash from private key
pub fn privkey_to_pubkey_hash(private_key: &[u8]) -> Result<Vec<u8>, Error> {
    Ok(utils::pub_key_hash(&privkey_to_pubkey_internal(
        private_key,
    )?))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::crypto::{privkey_to_pubkey, privkey_to_pubkey_hash, sign_musig, verify_musig};

    #[test]
    fn test_sign() {
        let eth_private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let seed = privkey_seed(eth_private_key).unwrap();
        assert_eq!(hex::encode(&seed), "4e676849675aa768792f3f0cb7a993f55caea561635ceea06f8e0fc70c62b8e57adffc47ebc1a2c2377fce88d76dc51d78f7749ac12d9f8fc7bfe0ce694175c21c");
        let private_key_raw = privkey_from_eth_privkey(eth_private_key).unwrap();
        let pub_key = privkey_to_pubkey(&private_key_raw).unwrap();
        assert_eq!(
            hex::encode(&pub_key),
            "7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510"
        );
        let pub_key_hash = privkey_to_pubkey_hash(&private_key_raw).unwrap();
        assert_eq!(
            hex::encode(pub_key_hash),
            "d8d5fb6a6caef06aa3dc2abdcdc240987e5330fe"
        );
        let msg: [u8; 32] = [
            166, 250, 15, 177, 151, 97, 14, 156, 48, 11, 76, 17, 0, 13, 17, 66, 241, 162, 16, 186,
            55, 222, 87, 213, 109, 241, 137, 184, 73, 251, 47, 208,
        ];
        let sig = sign_musig(&private_key_raw, &msg).unwrap();
        let verify_result = verify_musig(&msg, &sig);
        assert!(verify_result.is_ok());
    }
}
