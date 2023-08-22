//! Utils for signing zklink transactions.

pub mod error;
mod pk_signer;
pub mod signature;
pub mod utils;
pub use pk_signer::ZkLinkSigner;

pub use franklin_crypto::bellman::pairing::bn256::{Bn256 as Engine, Fr};
use franklin_crypto::rescue::bn256::Bn256RescueParams;
pub(crate) use franklin_crypto::{
    alt_babyjubjub::AltJubjubBn256,
    eddsa::{PrivateKey as EddsaPrivKey, PublicKey as EddsaPubkey, Signature as EddsaSignature},
    jubjub::JubjubEngine,
};

const PACKED_POINT_SIZE: usize = 32;
const SIGNATURE_SIZE: usize = 96;

thread_local! {
    pub(crate) static JUBJUB_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
    pub(crate) static RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}

type Fs = <Engine as JubjubEngine>::Fs;

pub struct Signature(EddsaSignature<Engine>);
impl AsRef<EddsaSignature<Engine>> for Signature {
    fn as_ref(&self) -> &EddsaSignature<Engine> {
        &self.0
    }
}
impl From<EddsaSignature<Engine>> for Signature {
    fn from(value: EddsaSignature<Engine>) -> Self {
        Self(value)
    }
}

pub struct PrivateKey(EddsaPrivKey<Engine>);
impl AsRef<EddsaPrivKey<Engine>> for PrivateKey {
    fn as_ref(&self) -> &EddsaPrivKey<Engine> {
        &self.0
    }
}
impl From<EddsaPrivKey<Engine>> for PrivateKey {
    fn from(value: EddsaPrivKey<Engine>) -> Self {
        Self(value)
    }
}

pub struct PublicKey(EddsaPubkey<Engine>);
impl AsRef<EddsaPubkey<Engine>> for PublicKey {
    fn as_ref(&self) -> &EddsaPubkey<Engine> {
        &self.0
    }
}
impl From<EddsaPubkey<Engine>> for PublicKey {
    fn from(value: EddsaPubkey<Engine>) -> Self {
        Self(value)
    }
}

