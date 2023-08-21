//! Utils for signing zklink transactions.

pub mod error;
pub mod signature;
pub mod utils;
mod pk_signer;
pub use pk_signer::ZkLinkSigner;

pub use franklin_crypto::bellman::pairing::bn256::{Bn256 as Engine, Fr};
use franklin_crypto::rescue::bn256::Bn256RescueParams;
use franklin_crypto::{
    alt_babyjubjub::AltJubjubBn256,
    eddsa::{PrivateKey as EddsaPrivKey, PublicKey as EddsaPubkey, Signature as EddsaSignature},
    jubjub::JubjubEngine,
};

const PACKED_POINT_SIZE: usize = 32;
const SIGNATURE_SIZE: usize = 96;

pub type Fs = <Engine as JubjubEngine>::Fs;
pub type Signature = EddsaSignature<Engine>;
pub type PrivateKey = EddsaPrivKey<Engine>;
pub type PublicKey = EddsaPubkey<Engine>;

thread_local! {
    pub(crate) static JUBJUB_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
    pub(crate) static RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}
