pub mod error;
pub mod pk_signer;
pub mod pubkey_hash;
pub mod public_key;
pub mod signature;
pub mod utils;
#[cfg(target_arch = "wasm32")]
pub mod wasm_binding;

pub use franklin_crypto::bellman::pairing::bn256::{Bn256 as Engine, Fr};
use franklin_crypto::rescue::bn256::Bn256RescueParams;
pub(crate) use franklin_crypto::{
    alt_babyjubjub::AltJubjubBn256,
    eddsa::{PrivateKey as EddsaPrivKey, PublicKey as EddsaPubkey, Signature as EddsaSignature},
};

const PACKED_POINT_SIZE: usize = 32;
const SIGNATURE_SIZE: usize = 96;

const NEW_PUBKEY_HASH_BYTES_LEN: usize = 20;
const NEW_PUBKEY_HASH_WIDTH: usize = NEW_PUBKEY_HASH_BYTES_LEN * 8;

thread_local! {
    pub(crate) static JUBJUB_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
    pub(crate) static RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();

}
