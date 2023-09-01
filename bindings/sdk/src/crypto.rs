use std::sync::Arc;
use zklink_crypto::zklink_signer::error::ZkSignerError;
use zklink_crypto::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_crypto::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_crypto::zklink_signer::public_key::PackedPublicKey;
use zklink_crypto::zklink_signer::signature::ZkLinkSignature;
use zklink_types::tx_type::change_pubkey::ChangePubKey;
use zklink_types::tx_type::forced_exit::ForcedExit;
use zklink_types::tx_type::transfer::Transfer;
use zklink_types::tx_type::withdraw::Withdraw;

pub fn verify_musig(signature: ZkLinkSignature, msg: &[u8]) -> Result<bool, ZkSignerError> {
    signature.verify_musig(msg)
}

pub fn get_public_key_hash(public_key: PackedPublicKey) -> PubKeyHash {
    public_key.public_key_hash()
}

