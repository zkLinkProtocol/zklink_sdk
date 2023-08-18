//! Utils for signing zklink transactions.

pub mod error;
pub mod signature;
pub mod utils;
pub mod zklink_private_key;

use error::Error;
pub use franklin_crypto::bellman::pairing::bn256::{Bn256 as Engine, Fr};

use franklin_crypto::rescue::bn256::Bn256RescueParams;
use franklin_crypto::{
    alt_babyjubjub::AltJubjubBn256,
    eddsa::{PublicKey, Signature as EddsaSignature},
    jubjub::JubjubEngine,
};

const PACKED_POINT_SIZE: usize = 32;
const PACKED_SIGNATURE_SIZE: usize = 64;

pub type Fs = <Engine as JubjubEngine>::Fs;
pub type Signature = EddsaSignature<Engine>;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    pub static JUBJUB_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
    pub static RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}

/// get the public key hash from public key
pub fn pub_key_hash(pubkey: &[u8]) -> Result<Vec<u8>, Error> {
    let pubkey = JUBJUB_PARAMS
        .with(|params| PublicKey::read(pubkey, params))
        .map_err(|_| Error::common("couldn't read public key"))?;
    Ok(utils::pub_key_hash(&pubkey))
}
