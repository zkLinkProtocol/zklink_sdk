// Built-in imports
use std::fmt;
use zksync_eth_signer::error::SignerError;
use zksync_eth_signer::{ZkLinkSigner};
// Workspace uses
use zklink_crypto::PrivateKey;
use zklink_types::{Layer1TxData, PubKeyHash};
use zklink_types::tx::{ChangePubKey, PackedEthSignature};
use zklink_rpc::types::ChainResp;
use crate::credentials::WalletCredentials;
// Local imports

fn signing_failed_error(err: impl ToString) -> SignerError {
    SignerError::SigningFailed(err.to_string())
}

pub struct Signer<S: ZkLinkSigner> {
    pub(crate) private_key: PrivateKey,
    pub pubkey_hash: PubKeyHash,
    pub(crate) eth_signer: Option<S>,
}

impl<S: ZkLinkSigner> fmt::Debug for Signer<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pk_contents = Vec::new();
        self.private_key
            .write(&mut pk_contents)
            .expect("Failed writing the private key contents");
        f.debug_struct("Signer")
            .field("pubkey_hash", &self.pubkey_hash)
            .finish()
    }
}

impl<S: ZkLinkSigner> Signer<S> {
    pub fn new(private_key: PrivateKey, eth_signer: Option<S>) -> Self {
        let pubkey_hash = PubKeyHash::from_privkey(&private_key);

        Self {
            private_key,
            pubkey_hash,
            eth_signer,
        }
    }

    /// Construct a `Signer` with the given credentials
    pub fn with_credentials(credentials: WalletCredentials<S>) -> Self {
        Self::new(
            credentials.zksync_private_key,
            credentials.eth_signer,
        )
    }

    pub fn pubkey_hash(&self) -> &PubKeyHash {
        &self.pubkey_hash
    }

    pub fn sign_layer_two_message(&self, message:&[u8]) -> TxSignature {
        TxSignature::sign_musig(&self.private_key, message)
    }

    /// see eip191, pretend 'Ethereum Signed Message' to the message
    pub async fn sign_layer_one_message(&self, message: Layer1TxData) -> Result<PackedEthSignature, SignerError> {
        let eth_signer = self
            .eth_signer
            .as_ref()
            .ok_or(SignerError::MissingEthSigner)?;
        let eth_signature = eth_signer
            .sign_message(message)
            .await
            .map_err(signing_failed_error)?;

        // it's bothersome, we will fix it later
        let eth_signature = match eth_signature {
            TxLayer1Signature::EthereumSignature(packed_signature) => Ok(packed_signature),
            TxLayer1Signature::EIP1271Signature(..) => Err(SignerError::CustomError(
                "Can't sign tx message with EIP1271 signer".to_string(),
            )),
            TxLayer1Signature::StarkSignature(..) => Err(SignerError::CustomError(
                "Can't sign tx message with Stark signer".to_string(),
            )),
        }?;

        Ok(eth_signature)
    }

    pub async fn sign_change_pub_key_ecdsa_auth_data(&self, tx: &ChangePubKey, chain_config: &ChainResp) -> Result<PackedEthSignature, SignerError> {
        let sign_bytes = tx.
            get_eth_eip712_signed_data_of_chain(chain_config.layer_one_chain_id, &chain_config.main_contract)
            .map_err(signing_failed_error)?;

        let eth_signer = self
            .eth_signer
            .as_ref()
            .ok_or(SignerError::MissingEthSigner)?;

        // sign_bytes is a eip712 data, use sign_raw_message
        let eth_signature = eth_signer
            .sign_raw_message(sign_bytes.into())
            .await
            .map_err(signing_failed_error)?;

        let eth_signature = match eth_signature {
            TxLayer1Signature::EthereumSignature(packed_signature) => Ok(packed_signature),
            TxLayer1Signature::EIP1271Signature(..) => Err(SignerError::CustomError(
                "Can't sign ChangePubKey message with EIP1271 signer".to_string(),
            )),
            TxLayer1Signature::StarkSignature(..) => Err(SignerError::CustomError(
                "Can't sign ChangePubKey message with Stark signer".to_string(),
            )),
        }?;

        Ok(eth_signature)
    }
}
