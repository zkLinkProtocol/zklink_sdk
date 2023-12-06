use super::error::StarkSignerError as Error;
use crate::starknet_signer::ecdsa_signature::StarkSignature;
use crate::starknet_signer::StarkECDSASignature;
use starknet::core::crypto::compute_hash_on_elements;
use starknet::core::types::FieldElement;
use starknet_signers::SigningKey;

pub struct StarkSigner(SigningKey);

impl Default for StarkSigner {
    fn default() -> Self {
        Self::new()
    }
}

impl StarkSigner {
    pub fn new() -> Self {
        let signing_key = SigningKey::from_random();
        Self(signing_key)
    }
    pub fn public_key(&self) -> FieldElement {
        let verifying_key = self.0.verifying_key();
        verifying_key.scalar()
    }

    pub fn new_from_hex_str(hex_str: &str) -> Result<Self, Error> {
        let private_key =
            FieldElement::from_hex_be(hex_str).map_err(|e| Error::InvalidPrivKey(e.to_string()))?;
        let signing_key = SigningKey::from_secret_scalar(private_key);
        Ok(Self(signing_key))
    }

    /// 1. get the hash of the message
    /// 2. sign hash
    pub fn sign_message(&self, msg: &[u8]) -> Result<StarkECDSASignature, Error> {
        let hash = Self::get_msg_hash(msg);
        let signature = self
            .0
            .sign(&hash)
            .map_err(|e| Error::sign_error(e.to_string()))?;
        let s = StarkECDSASignature {
            pub_key: self.public_key(),
            signature: StarkSignature {
                s: signature.s,
                r: signature.r,
            },
        };
        Ok(s)
    }

    /// 1. change msg to FieldElement list
    /// 2. compute hash of the FieldElement list
    pub fn get_msg_hash(msg: &[u8]) -> FieldElement {
        let elements: Vec<_> = msg
            .chunks(32)
            .map(|val| FieldElement::from_byte_slice_be(val).unwrap())
            .collect();
        compute_hash_on_elements(&elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestSignature {
        signature: StarkECDSASignature,
    }

    #[test]
    fn test_starknet_sign() {
        let private_key = "0x02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159";
        let stark_signer = StarkSigner::new_from_hex_str(private_key).unwrap();
        let msg = b"hello world";
        let signature = stark_signer.sign_message(msg).unwrap();
        let is_ok = signature.verify(msg).unwrap();
        assert!(is_ok);
        let data = TestSignature { signature };
        let s = serde_json::to_string(&data).unwrap();
        println!("{s}");
        let data2: TestSignature = serde_json::from_str(&s).unwrap();
        println!("{data2:?}");
    }
}
