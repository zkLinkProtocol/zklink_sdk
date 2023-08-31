use zklink_crypto::zklink_signer::error::ZkSignerError;
use zklink_crypto::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_crypto::zklink_signer::public_key::PackedPublicKey;
use zklink_crypto::zklink_signer::signature::ZkLinkSignature;

pub fn verify_musig(signature: ZkLinkSignature, msg: &[u8]) -> Result<bool, ZkSignerError> {
    signature.verify_musig(msg)
}

pub fn get_public_key_hash(public_key: PackedPublicKey) -> PubKeyHash {
    public_key.public_key_hash()
}
