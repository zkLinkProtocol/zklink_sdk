use super::{EddsaPubkey, Engine};
use crate::zklink_signer::utils::{
    append_le_fixed_width, pack_bits_into_bytes, rescue_hash_elements,
};
use crate::zklink_signer::{NEW_PUBKEY_HASH_WIDTH, PACKED_POINT_SIZE};

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

impl PublicKey {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut pubkey_buf = Vec::with_capacity(PACKED_POINT_SIZE);
        self.as_ref()
            .write(&mut pubkey_buf)
            .expect("failed to write pubkey to buffer");
        let mut pubkey = [0; PACKED_POINT_SIZE];
        pubkey.copy_from_slice(&pubkey_buf);
        pubkey_buf
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
