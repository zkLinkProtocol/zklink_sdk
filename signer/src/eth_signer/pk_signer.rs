use super::packed_eth_signature::PackedEthSignature;
use super::raw_tx::{RawTransaction, Transaction};
use super::EthSignerError;
use parity_crypto::publickey::{sign, KeyPair};

use crate::eth_signer::{Address, H256};
use secp256k1::SecretKey;

#[derive(Clone)]
pub struct EthSigner {
    private_key: H256,
}

impl std::fmt::Debug for EthSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PrivateKeySigner")
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
    pub fn get_address(&self) -> Result<Address, EthSignerError> {
        Ok(KeyPair::from_secret((self.private_key).into())
            .map_err(|_| EthSignerError::DefineAddress)?
            .address())
    }

    /// Signs and returns the RLP-encoded transaction.
    pub fn sign_transaction(&self, raw_tx: RawTransaction) -> Result<Vec<u8>, EthSignerError> {
        let key = SecretKey::from_slice(self.private_key.as_bytes()).unwrap();

        let gas_price = match raw_tx.max_fee_per_gas {
            Some(val) => val,
            None => raw_tx.gas_price,
        };
        let tx = Transaction {
            to: raw_tx.to,
            nonce: raw_tx.nonce,
            gas: raw_tx.gas,
            gas_price,
            value: raw_tx.value,
            data: raw_tx.data,
            transaction_type: raw_tx.transaction_type,
            access_list: raw_tx.access_list.unwrap_or_default(),
            max_priority_fee_per_gas: raw_tx.max_priority_fee_per_gas.unwrap_or_default(),
        };

        let signed = tx.sign(&key, raw_tx.chain_id);
        Ok(signed.raw_transaction.0)
    }

    /// The sign method calculates an Ethereum specific signature with:
    /// sign(keccak256("\x19Ethereum Signed Message:\n" + len(message) + message))).
    /// Signs message using ethereum private key, results are identical to signature created
    /// using `geth`, `ethecore/lib/types/src/gas_counter.rsrs.js`, etc. No hashing and prefixes required.
    pub fn sign_message(&self, msg: &[u8]) -> Result<PackedEthSignature, EthSignerError> {
        let signed_bytes = PackedEthSignature::message_to_signed_bytes(msg);
        self.sign_byted_data(&signed_bytes)
    }

    pub fn sign_byted_data(&self, msg: &H256) -> Result<PackedEthSignature, EthSignerError> {
        let secret_key = self.private_key.into();
        let signature =
            sign(&secret_key, msg).map_err(|err| EthSignerError::SigningFailed(err.to_string()))?;

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

impl From<&H256> for EthSigner {
    fn from(private_key: &H256) -> Self {
        Self {
            private_key: *private_key,
        }
    }
}

#[cfg(test)]
mod test {
    use super::EthSigner;
    use super::RawTransaction;
    use web3::types::{H160, H256, U256, U64};

    #[tokio::test]
    async fn test_generating_signed_raw_transaction() {
        let private_key = H256::from([5; 32]);
        let private_key = hex::encode(private_key.as_bytes());
        let signer = EthSigner::try_from(private_key.as_str()).unwrap();
        let raw_transaction = RawTransaction {
            nonce: U256::from(1u32),
            to: Some(H160::default()),
            gas: Default::default(),
            gas_price: U256::from(2u32),
            max_fee_per_gas: Some(U256::from(2u32)),
            max_priority_fee_per_gas: Some(U256::from(1u32)),
            value: Default::default(),
            data: vec![1, 2, 3],
            chain_id: 270,
            transaction_type: Some(U64::from(1u32)),
            access_list: None,
        };
        let raw_tx = signer.sign_transaction(raw_transaction.clone()).unwrap();
        assert_ne!(raw_tx.len(), 1);
        // precalculated signature with right algorithm implementation
        let precalculated_raw_tx: Vec<u8> = vec![
            1, 248, 100, 130, 1, 14, 1, 2, 128, 148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 128, 131, 1, 2, 3, 192, 1, 160, 98, 201, 238, 158, 215, 98, 23, 231,
            221, 161, 170, 16, 54, 85, 187, 107, 12, 228, 218, 139, 103, 164, 17, 196, 178, 185,
            252, 243, 186, 175, 93, 230, 160, 93, 204, 205, 5, 46, 187, 231, 211, 102, 133, 200,
            254, 119, 94, 206, 81, 8, 143, 204, 14, 138, 43, 183, 214, 209, 166, 16, 116, 176, 44,
            52, 133,
        ];
        assert_eq!(raw_tx, precalculated_raw_tx);
    }
}
