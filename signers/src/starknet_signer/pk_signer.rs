use super::error::StarkSignerError as Error;
use crate::starknet_signer::ecdsa_signature::StarkSignature;
use crate::starknet_signer::typed_data::TypedData;
use crate::starknet_signer::StarkECDSASignature;
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
    pub fn sign_message(&self, msg: &TypedData, addr: &str) -> Result<StarkECDSASignature, Error> {
        let addr = FieldElement::from_hex_be(addr).map_err(|e| Error::SignError(e.to_string()))?;
        let hash = msg.get_message_hash(addr)?;
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
    use crate::starknet_signer::typed_data::message::{TxMessage, TypedDataMessage};
    use crate::starknet_signer::typed_data::TypedData;
    use serde::{Deserialize, Serialize};
    use starknet_core::crypto::Signature;
    use starknet_signers::VerifyingKey;
    use std::str::FromStr;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestSignature {
        signature: StarkECDSASignature,
    }

    #[test]
    fn test_starknet_sign() {
        let private_key = "0x02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159";
        let stark_signer = StarkSigner::new_from_hex_str(private_key).unwrap();
        let msg_hash = "0x51d5faacb1bdeb6293d52fd4be0a7c62417cb73962cdd6aff385b67239cf081";
        let signature = stark_signer
            .0
            .sign(&FieldElement::from_hex_be(msg_hash).unwrap())
            .unwrap();
        //let pub_key = stark_signer.public_key();
        let pub_key = "0x3b478eae5afdd35358abcc7955bba7acda3d16f4485b62f2497f78ed6bc7126";
        let verifying_key = VerifyingKey::from_scalar(FieldElement::from_hex_be(pub_key).unwrap());
        let is_ok = verifying_key
            .verify(&FieldElement::from_hex_be(msg_hash).unwrap(), &signature)
            .unwrap();
        println!("{:?}", is_ok);
    }

    #[test]
    fn test_signature_verify() {
        let pubkey = "1082125475812817975721104073212648033952831721853656627074253194227094744819";
        let sig_str = "0x02647618b4fe405d0dccbdfd25c20bfdeb87631a332491c633943e6f59f16ef306f72dfce21313b636bef4afff3fdc929e5c3d01e3a1f586690ef7db7ebc280a042b54b6bfc5970163d3b9166fd8f24671dfbf850554eeaf12a5e8f4db06c7f3";
        let addr = "0x04A69b67bcaBfA7D3CCb96e1d25C2e6fC93589fE24A6fD04566B8700ff97a71a";
        let pub_key = FieldElement::from_str(&pubkey).unwrap();

        let transfer = TxMessage {
            amount: "0.0012345678998".to_string(),
            fee: "0.00000001".to_string(),
            nonce: "1".to_string(),
            to: "0x5505a8cd4594dbf79d8c59c0df1414ab871ca896".to_string(),
            token: "USDC".to_string(),
            transaction: "Transfer".to_string(),
        };

        let message = transfer.clone();
        let typed_data = TypedData::new(
            TypedDataMessage::Transaction { message },
            "SN_GOERLI".to_string(),
        );
        let addr = FieldElement::from_hex_be(&addr).unwrap();
        let msg_hash = typed_data.get_message_hash(addr).unwrap();
        println!("{:?}", msg_hash);
        let signature = StarkECDSASignature::from_hex(sig_str).unwrap();
        let verifying_key = VerifyingKey::from_scalar(pub_key);
        let is_ok = verifying_key
            .verify(
                &FieldElement::from_hex_be(&hex::encode(msg_hash.to_bytes_be())).unwrap(),
                &Signature {
                    s: signature.signature.s,
                    r: signature.signature.r,
                },
            )
            .unwrap();
        assert!(is_ok);
    }
}
