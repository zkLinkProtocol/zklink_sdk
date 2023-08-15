//! Utils for signing zksync transactions.
//! This crate is compiled into wasm to be used in `zksync.js`.

mod utils;

const PACKED_POINT_SIZE: usize = 32;
const PACKED_SIGNATURE_SIZE: usize = 64;

pub use franklin_crypto::bellman::pairing::bn256::{Bn256 as Engine, Fr};
use franklin_crypto::rescue::bn256::Bn256RescueParams;

pub type Fs = <Engine as JubjubEngine>::Fs;

thread_local! {
    pub static JUBJUB_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
    pub static RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}

use wasm_bindgen::prelude::*;

use franklin_crypto::{
    alt_babyjubjub::{edwards, fs::FsRepr, AltJubjubBn256, FixedGenerators},
    bellman::pairing::ff::{PrimeField, PrimeFieldRepr},
    eddsa::{PrivateKey, PublicKey, Seed, Signature as EddsaSignature},
    jubjub::JubjubEngine,
};

pub type Signature = EddsaSignature<Engine>;

use crate::crypto::utils::set_panic_hook;
use sha2::{Digest, Sha256};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
/// This method initializes params for current thread, otherwise they will be initialized when signing
/// first message.
pub fn zksync_crypto_init() {
    JUBJUB_PARAMS.with(|_| {});
    RESCUE_PARAMS.with(|_| {});
    set_panic_hook();
}

#[wasm_bindgen(js_name = privateKeyFromSeed)]
pub fn private_key_from_seed(seed: &[u8]) -> Result<Vec<u8>, JsValue> {
    if seed.len() < 32 {
        return Err(JsValue::from_str("Seed is too short"));
    };

    let sha256_bytes = |input: &[u8]| -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(input);
        hasher.result().to_vec()
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

fn read_signing_key(private_key: &[u8]) -> Result<PrivateKey<Engine>, JsValue> {
    let mut fs_repr = FsRepr::default();
    fs_repr
        .read_be(private_key)
        .map_err(|_| JsValue::from_str("couldn't read private key repr"))?;
    Ok(PrivateKey::<Engine>(
        Fs::from_repr(fs_repr).expect("couldn't read private key from repr"),
    ))
}

fn privkey_to_pubkey_internal(private_key: &[u8]) -> Result<PublicKey<Engine>, JsValue> {
    let p_g = FixedGenerators::SpendingKeyGenerator;

    let sk = read_signing_key(private_key)?;

    Ok(JUBJUB_PARAMS.with(|params| PublicKey::from_private(&sk, p_g, params)))
}

#[wasm_bindgen(js_name = pubKeyHash)]
pub fn pub_key_hash(pubkey: &[u8]) -> Result<Vec<u8>, JsValue> {
    let pubkey = JUBJUB_PARAMS
        .with(|params| PublicKey::read(pubkey, params))
        .map_err(|_| JsValue::from_str("couldn't read public key"))?;
    Ok(utils::pub_key_hash(&pubkey))
}

#[wasm_bindgen]
pub fn private_key_to_pubkey_hash(private_key: &[u8]) -> Result<Vec<u8>, JsValue> {
    Ok(utils::pub_key_hash(&privkey_to_pubkey_internal(
        private_key,
    )?))
}

#[wasm_bindgen]
pub fn private_key_to_pubkey(private_key: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut pubkey_buf = Vec::with_capacity(PACKED_POINT_SIZE);

    let pubkey = privkey_to_pubkey_internal(private_key)?;

    pubkey
        .write(&mut pubkey_buf)
        .expect("failed to write pubkey to buffer");

    Ok(pubkey_buf)
}

#[wasm_bindgen(js_name = "rescueHash")]
pub fn rescue_hash_tx_msg(msg: &[u8]) -> Vec<u8> {
    utils::rescue_hash_tx_msg(msg)
}

/// `msg` should be represented by 2 concatenated
/// serialized orders of the swap transaction
#[wasm_bindgen(js_name = "rescueHashOrders")]
pub fn rescue_hash_orders(msg: &[u8]) -> Vec<u8> {
    utils::rescue_hash_orders(msg)
}

#[wasm_bindgen]
/// We use musig Schnorr signature scheme.
/// It is impossible to restore signer for signature, that is why we provide public key of the signer
/// along with signature.
/// [0..32] - packed public key of signer.
/// [32..64] - packed r point of the signature.
/// [64..96] - s poing of the signature.
pub fn sign_musig(private_key: &[u8], msg: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut packed_full_signature = Vec::with_capacity(PACKED_POINT_SIZE + PACKED_SIGNATURE_SIZE);
    let p_g = FixedGenerators::SpendingKeyGenerator;
    let private_key = read_signing_key(private_key)?;

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

#[wasm_bindgen]
pub fn verify_musig(msg: &[u8], signature: &[u8]) -> Result<bool, JsValue> {
    if signature.len() != PACKED_POINT_SIZE + PACKED_SIGNATURE_SIZE {
        return Err(JsValue::from_str("Signature length is not 96 bytes. Make sure it contains both the public key and the signature itself."));
    }

    let pubkey = &signature[..PACKED_POINT_SIZE];
    let pubkey = JUBJUB_PARAMS
        .with(|params| edwards::Point::read(&*pubkey, params).map(PublicKey))
        .map_err(|_| JsValue::from_str("couldn't read public key"))?;

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

fn deserialize_signature(bytes: &[u8]) -> Result<Signature, JsValue> {
    let (r_bar, s_bar) = bytes.split_at(PACKED_POINT_SIZE);

    let r = JUBJUB_PARAMS
        .with(|params| edwards::Point::read(r_bar, params))
        .map_err(|_| JsValue::from_str("Failed to parse signature"))?;

    let mut s_repr = FsRepr::default();
    s_repr
        .read_le(s_bar)
        .map_err(|_| JsValue::from_str("Failed to parse signature"))?;

    let s = <Engine as JubjubEngine>::Fs::from_repr(s_repr)
        .map_err(|_| JsValue::from_str("Failed to parse signature"))?;

    Ok(Signature { r, s })
}

///
/// func GenerateLayer2PrivateKeySeed(ethKey string) ([]byte, error) {
/// 	msg := "Sign this message to create a key to interact with zkLink's layer2 services.\nNOTE: This application is powered by zkLink protocol.\n\nOnly sign this message for a trusted client!"
///
/// 	privateKey, _ := crypto.HexToECDSA(strings.ToLower(ethKey))
/// 	hash := crypto.Keccak256([]byte(fmt.Sprintf("\x19Ethereum Signed Message:\n%d%s", len(msg), msg)))
/// 	signature, err := crypto.Sign(hash, privateKey)
/// 	signature[64] += 27
/// 	if err != nil {
/// 		return nil, err
/// 	}
///
/// 	return signature, nil
/// }


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign() {
        // let s = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        // let priv_key_raw = read_signing_key(&hex::decode(s).unwrap()).unwrap();
        // let seed = hex::decode(s).unwrap();
        let seed = hex::decode("4e676849675aa768792f3f0cb7a993f55caea561635ceea06f8e0fc70c62b8e57adffc47ebc1a2c2377fce88d76dc51d78f7749ac12d9f8fc7bfe0ce694175c21c").unwrap();
        let priv_key_raw = private_key_from_seed(&seed).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("{priv_key_raw:?}");
        let pub_key = private_key_to_pubkey(&priv_key_raw).unwrap();
        println!("pubkey calc  : {}", hex::encode(&pub_key));
        println!("pubkey expect: 5d29e3296af85d962dbfae8f1fbdc295e5d57eb4fddb8186de7c06be8df768ac");
        let pub_key_hash = private_key_to_pubkey_hash(&priv_key_raw).unwrap();
        println!("new pubkey hash: {}", hex::encode(pub_key_hash));
        let msg :[u8; 32] = [166, 250, 15, 177, 151, 97, 14, 156, 48, 11, 76, 17, 0, 13, 17, 66, 241, 162, 16, 186, 55, 222, 87, 213, 109, 241, 137, 184, 73, 251, 47, 208];
        let sig = sign_musig( &priv_key_raw, &msg).unwrap();
        let verify_result = verify_musig(&msg, &sig);
        assert!(verify_result.is_ok());
    }
}
