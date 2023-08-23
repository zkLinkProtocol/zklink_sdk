use franklin_crypto::eddsa::PublicKey;
use web3::signing::SigningError;
use super::{EddsaPubkey, Engine};
use crate::zklink_signer::utils::{
    append_le_fixed_width, pack_bits_into_bytes, rescue_hash_elements,
};
use crate::zklink_signer::{JUBJUB_PARAMS, NEW_PUBKEY_HASH_WIDTH, PACKED_POINT_SIZE};
use crate::zklink_signer::error::ZkSignerError;

pub struct PackedPublicKey(EddsaPubkey<Engine>);
impl AsRef<EddsaPubkey<Engine>> for PackedPublicKey {
    fn as_ref(&self) -> &EddsaPubkey<Engine> {
        &self.0
    }
}
impl From<EddsaPubkey<Engine>> for PackedPublicKey {
    fn from(value: EddsaPubkey<Engine>) -> Self {
        Self(value)
    }
}

impl PackedPublicKey {
    /// converts public key to byte array
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut pubkey_buf = Vec::with_capacity(PACKED_POINT_SIZE);
        self.as_ref()
            .write(&mut pubkey_buf)
            .expect("failed to write pubkey to buffer");
        let mut pubkey = [0; PACKED_POINT_SIZE];
        pubkey.copy_from_slice(&pubkey_buf);
        pubkey_buf
    }


    /// converts public key to a hex string with the 0x prefix
    pub fn as_hex(&self) -> String {
        let bytes = self.as_bytes();
        format!("0x{}", hex::encode(bytes))
    }

    pub fn public_key_hash(&self) -> Vec<u8> {
        let (pub_x, pub_y) = self.as_ref().0.into_xy();
        let pub_key_hash = rescue_hash_elements(&[pub_x, pub_y]);
        let mut pub_key_hash_bits = Vec::with_capacity(NEW_PUBKEY_HASH_WIDTH);
        append_le_fixed_width(&mut pub_key_hash_bits, &pub_key_hash, NEW_PUBKEY_HASH_WIDTH);
        let mut bytes = pack_bits_into_bytes(&pub_key_hash_bits);
        bytes.reverse();
        bytes
    }
}
