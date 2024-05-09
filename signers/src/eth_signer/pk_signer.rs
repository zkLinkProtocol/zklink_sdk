use super::packed_eth_signature::PackedEthSignature;
use super::EthSignerError;

use crate::eth_signer::{Address, H256};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::TxHash;
use ethers::utils::hash_message;
use k256::ecdsa::SigningKey;

#[derive(Clone)]
pub struct EthSigner {
    private_key: H256,
}

impl std::fmt::Debug for EthSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**EthSigner**")
    }
}

impl EthSigner {
    #[cfg(feature = "ffi")]
    pub fn new(s: &str) -> Result<Self, EthSignerError> {
        s.try_into()
    }

    pub fn random() -> Self {
        Self {
            private_key: H256::random(),
        }
    }

    /// Get Ethereum address that matches the private key.
    pub fn get_address(&self) -> Address {
        let key = SigningKey::from_slice(self.private_key.as_bytes()).unwrap();
        Address::from_slice(LocalWallet::from(key).address().as_bytes())
    }

    /// Signs and returns the RLP-encoded transaction.
    pub fn sign_transaction(
        &self,
        tx: &TypedTransaction,
    ) -> Result<PackedEthSignature, EthSignerError> {
        let key = SigningKey::from_slice(self.private_key.as_bytes()).unwrap();
        let signed = LocalWallet::from(key)
            .with_chain_id(tx.chain_id().unwrap_or_default().as_u64())
            .sign_transaction_sync(tx)
            .map_err(|err| EthSignerError::SigningFailed(err.to_string()))?;
        Ok(PackedEthSignature(signed))
    }

    /// The sign method calculates an Ethereum specific signature with:
    /// sign(keccak256("\x19Ethereum Signed Message:\n" + len(message) + message))).
    /// Signs message using ethereum private key, results are identical to signature created
    /// using `geth`, `ethecore/lib/types/src/gas_counter.rsrs.js`, etc. No hashing and prefixes required.
    pub fn sign_message(&self, msg: &[u8]) -> Result<PackedEthSignature, EthSignerError> {
        let hash = hash_message(msg);
        self.sign_hash(hash.as_bytes())
    }

    pub fn sign_hash(&self, hash: &[u8]) -> Result<PackedEthSignature, EthSignerError> {
        let tx_hash = TxHash::from_slice(hash);
        let key = SigningKey::from_slice(self.private_key.as_bytes()).unwrap();
        let signature = LocalWallet::from(key)
            .sign_hash(tx_hash)
            .map_err(|err| EthSignerError::SigningFailed(err.to_string()))?;
        Ok(PackedEthSignature(signature))
    }
}

impl TryFrom<&str> for EthSigner {
    type Error = EthSignerError;

    fn try_from(private_key: &str) -> Result<Self, Self::Error> {
        let s = private_key.strip_prefix("0x").unwrap_or(private_key);
        let raw = hex::decode(s).map_err(|_| EthSignerError::InvalidEthSigner)?;
        if raw.len() != 32 {
            return Err(EthSignerError::InvalidEthSigner);
        }
        let private_key = H256::from_slice(&raw);
        Ok(Self { private_key })
    }
}

impl From<H256> for EthSigner {
    fn from(private_key: H256) -> Self {
        Self { private_key }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_eth_signer() {
        let private_key = H256::from([5; 32]);
        let private_key = hex::encode(private_key.as_bytes());
        let signer = EthSigner::try_from(private_key.as_str()).unwrap();
        println!("{signer:?}");
        let msg = b"hello world";
        let signature = signer.sign_message(msg).unwrap();
        println!("msg signature {:?}", signature.as_hex());
        assert_eq!(signature.as_hex(), "0x8be74156ea8739cc0d524b91ca972c610f7f0cc5a4408a9ceece1fdd8395d92e6581158309b1a50545260c5100e529f22eb00fea97c41072c265f7d8fb08a9621b");
        let hash = [1; 32];
        let signature = signer.sign_hash(&hash).unwrap();
        println!("hash signature: {:?}", signature.as_hex());
        assert_eq!(signature.as_hex(), "0xe57f551f38c5ffd4f78fcd4eccdb6f8ea322dc6d6f639f49d0daf24684094eca629a2faaecdced17898068511142658c58325b7f9e648bec971b92a9820e08521c");

        let private_key = "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
        let pk_signer = EthSigner::try_from(private_key).unwrap();
        let address = pk_signer.get_address();
        assert_eq!(
            address,
            Address::from_str("0x9e372368c25056D44045e445d72d7B91cE3eE3B1").unwrap()
        );
        let message = b"hello world";
        let signature = pk_signer.sign_message(message).unwrap();
        assert_eq!(signature.as_hex(), "0xa9aa0710adb18f84d4bed8057382fc433c3dcff1bddf3b2b1c2cb11386ef3be4172b5d0688143759d4e744acc434ae4f96575c7fa9096971fd02fb3d2aaa77121c");
        let recover_addr = signature.signature_recover_signer(message).unwrap();
        assert_eq!(recover_addr, address);
    }

    #[test]
    fn test_eth_eip712() {
        use crate::eth_signer::eip712::eip712::EIP712Domain;
        use crate::eth_signer::eip712::eip712::TypedData;
        use crate::eth_signer::EIP712Address;
        use serde::{Deserialize, Serialize};
        use serde_json::json;

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Person {
            pub name: String,
            pub wallet: EIP712Address,
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Mail {
            pub from: Person,
            pub to: Person,
            pub contents: String,
        }

        let private_key = "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
        let pk_signer = EthSigner::try_from(private_key).unwrap();

        let message = json!(
          {
            "from": {
              "name": "Cow",
              "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
            },
            "to": {
              "name": "Bob",
              "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
            },
            "contents": "Hello, Bob!"
          }
        );

        let message: Mail = serde_json::from_value(message).expect("parse domain");
        let domain = EIP712Domain::new(
            "Ether Mail".into(),
            "1".into(),
            1,
            "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC".into(),
        )
        .unwrap();

        let typed_data = TypedData::new(domain, message).unwrap();

        let signature = pk_signer
            .sign_hash(typed_data.sign_hash().unwrap().as_ref())
            .unwrap();

        assert_eq!(signature.as_hex(), "0xbf24877c59766e95717686e71a0402ba12f5db4a8aa93ac6c30b5742925ebfc26c91d6b6bb949a2b0578c397e296830dde9cc3531adbb259c4b4b06441b1a9c51b");
    }
}
