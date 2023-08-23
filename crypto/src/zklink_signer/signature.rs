use super::error::ZkSignerError as Error;
use super::JUBJUB_PARAMS;
use super::RESCUE_PARAMS;
use super::{utils, EddsaSignature, PACKED_POINT_SIZE, SIGNATURE_SIZE};
use franklin_crypto::alt_babyjubjub::{edwards, fs::FsRepr, FixedGenerators};
use franklin_crypto::bellman::pairing::bn256::Bn256 as Engine;
use franklin_crypto::bellman::pairing::ff::{PrimeField, PrimeFieldRepr};
use franklin_crypto::eddsa::PublicKey;
use franklin_crypto::jubjub::JubjubEngine;
use crate::zklink_signer::public_key::PackedPublicKey;

pub struct PackedSignature(EddsaSignature<Engine>);
impl AsRef<EddsaSignature<Engine>> for PackedSignature {
    fn as_ref(&self) -> &EddsaSignature<Engine> {
        &self.0
    }
}

impl From<EddsaSignature<Engine>> for PackedSignature {
    fn from(value: EddsaSignature<Engine>) -> Self {
        Self(value)
    }
}

/// ZkLink signature
/// [0..32] - packed public key of signer.
/// [32..64] - packed r point of the signature.
/// [64..96] - s point of the signature.
pub struct ZkLinkSignature {
    pub pub_key: PackedPublicKey,
    pub signature: PackedSignature,
}

impl ZkLinkSignature {
    // /// Create a ZkLinkSignature from u8 slice
    // pub fn new_from_slice(slice: &[u8]) -> Result<Self, Error> {
    //     if slice.len() != SIGNATURE_SIZE {
    //         return Err(Error::InvalidSignature("Signature length is not 96 bytes. Make sure it contains both the public key and the signature itself.".into()));
    //     }
    //     let mut raw = [0; SIGNATURE_SIZE];
    //     raw.copy_from_slice(slice);
    //     Ok(Self(raw))
    // }
    //
    // /// Create a ZkLinkSignature from hex string which starts with 0x or not
    // pub fn from_hex(s: &str) -> Result<Self, Error> {
    //     let s = s.strip_prefix("0x").unwrap_or(s);
    //     let raw = hex::decode(s).map_err(|_|Error::InvalidSignature("invalid signature string".into()))?;
    //     Self::new_from_slice(&raw)
    // }
    //
    // /// converts signature to a hex string with the 0x prefix
    // pub fn as_hex(&self) -> String {
    //     format!("0x{}", hex::encode(self.0))
    // }

    pub fn verify_musig(&self, msg: &[u8]) -> Result<bool, Error> {
        let pubkey = self.pub_key.as_ref();
        let signature = self.signature.as_ref();

        let msg = utils::rescue_hash_tx_msg(msg);
        let value = JUBJUB_PARAMS.with(|jubjub_params| {
            RESCUE_PARAMS.with(|rescue_params| {
                pubkey.verify_musig_rescue(
                    &msg,
                    signature,
                    FixedGenerators::SpendingKeyGenerator,
                    rescue_params,
                    jubjub_params,
                )
            })
        });

        Ok(value)
    }

}

#[cfg(test)]
mod test {
    use crate::zklink_signer::ZkLinkSigner;

    #[test]
    fn test_signature() {
        let eth_private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(&eth_private_key).unwrap();
        let msg = b"hello world";
        let signature = zk_signer.sign_musig(msg).unwrap();
        let verify = signature.verify_musig(msg).unwrap();
        assert!(verify);
    }
}
