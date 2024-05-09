use super::error::StarkSignerError as Error;
use crate::starknet_signer::ecdsa_signature::StarkEcdsaSignature;
use crate::starknet_signer::typed_data::TypedData;
use crate::starknet_signer::StarkEip712Signature;
use starknet_core::crypto::compute_hash_on_elements;
use starknet_core::types::FieldElement;
use starknet_signers::SigningKey;

pub struct StarkSigner(pub SigningKey);

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
    pub fn sign_message(&self, msg: &TypedData, addr: &str) -> Result<StarkEip712Signature, Error> {
        let addr = FieldElement::from_hex_be(addr).map_err(|e| Error::SignError(e.to_string()))?;
        let hash = msg.get_message_hash(addr)?;
        let signature = self
            .0
            .sign(&hash)
            .map_err(|e| Error::sign_error(e.to_string()))?;
        let s = StarkEip712Signature {
            pub_key: self.public_key(),
            signature: StarkEcdsaSignature {
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
    use crate::starknet_signer::typed_data::message::{TxMessage, TypedDataMessage};
    use crate::starknet_signer::typed_data::TypedData;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestSignature {
        signature: StarkEip712Signature,
    }

    #[test]
    fn test_signature_verify() {
        let sig_str = "0x02647618b4fe405d0dccbdfd25c20bfdeb87631a332491c633943e6f59f16ef307b3c5b947d6f8dd9c8d97be61172d1bc38f6c412d218f285c9425a2260afe5d029a21fa05eeb4b03729658858f72e67c610d9011effb46a479d64bf7b909506";
        let addr = "0x04A69b67bcaBfA7D3CCb96e1d25C2e6fC93589fE24A6fD04566B8700ff97a71a";
        let transfer = TxMessage {
            amount: "0.0012345678998".to_string(),
            fee: "0.00000001".to_string(),
            nonce: "1".to_string(),
            to: "0x0322546b712D87B8565C33530A6396D85f024F2C99ff564019a5Fc4c38e0F740".to_string(),
            token: "USDC".to_string(),
            transaction: "Transfer".to_string(),
        };

        let message = transfer.clone();
        let typed_data = TypedData::new(
            TypedDataMessage::Transaction { message },
            "SN_GOERLI".to_string(),
        );
        let signature = StarkEip712Signature::from_hex(sig_str).unwrap();
        let is_ok = signature.verify(&typed_data, addr).unwrap();
        assert!(is_ok);
    }
}
