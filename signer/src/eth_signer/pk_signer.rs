use super::packed_eth_signature::PackedEthSignature;
use super::EthSignerError;

use crate::eth_signer::{Address, H256};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::TxHash;
use ethers::utils::hash_message;
use k256::ecdsa::SigningKey;

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
        let key = SigningKey::from_slice(self.private_key.as_bytes())
            .map_err(|_| EthSignerError::DefineAddress)?;
        Ok(Address::from_slice(LocalWallet::from(key).address().as_bytes()))
    }

    /// Signs and returns the RLP-encoded transaction.
    pub fn sign_transaction(&self, tx: &TypedTransaction) -> Result<PackedEthSignature, EthSignerError> {
        let key = SigningKey::from_slice(self.private_key.as_bytes())
            .map_err(|_| EthSignerError::DefineAddress)?;
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
        let key = SigningKey::from_slice(self.private_key.as_bytes())
            .map_err(|_| EthSignerError::DefineAddress)?;
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

impl From<&H256> for EthSigner {
    fn from(private_key: &H256) -> Self {
        Self {
            private_key: *private_key,
        }
    }
}
