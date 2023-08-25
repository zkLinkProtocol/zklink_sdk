// Built-in imports
use zklink_crypto::eth_signer::error::EthSignerError;
use zklink_crypto::eth_signer::eth_signature::TxEthSignature;
use zklink_crypto::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_crypto::eth_signer::EthereumSigner;
use zklink_crypto::zklink_signer::error::ZkSignerError;
use zklink_crypto::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_crypto::zklink_signer::signature::ZkLinkSignature;
use zklink_crypto::zklink_signer::ZkLinkSigner;
// Local imports

fn signing_failed_error(err: impl ToString) -> EthSignerError {
    EthSignerError::SigningFailed(err.to_string())
}

pub struct Signer<S: EthereumSigner> {
    pub zklink_signer: ZkLinkSigner,
    pub(crate) eth_signer: Option<S>,
}

impl<S: EthereumSigner> Signer<S> {
    pub fn new(pk_bytes: &[u8], eth_signer: Option<S>) -> Result<Self, ZkSignerError> {
        let zklink_signer = ZkLinkSigner::new_from_bytes(pk_bytes)?;

        Ok(Self {
            zklink_signer,
            eth_signer,
        })
    }

    pub fn pubkey_hash(&self) -> PubKeyHash {
        self.zklink_signer.public_key.public_key_hash()
    }

    pub fn sign_layer_two_message(&self, message: &[u8]) -> Result<ZkLinkSignature, ZkSignerError> {
        self.zklink_signer.sign_musig(message)
    }

    /// see eip191, pretend 'Ethereum Signed Message' to the message
    pub async fn sign_layer_one_message(
        &self,
        message: &[u8],
    ) -> Result<PackedEthSignature, EthSignerError> {
        let eth_signer = self
            .eth_signer
            .as_ref()
            .ok_or(EthSignerError::MissingEthSigner)?;
        let eth_signature = eth_signer
            .sign_message(message)
            .await
            .map_err(signing_failed_error)?;

        // it's bothersome, we will fix it later
        let eth_signature = match eth_signature {
            TxEthSignature::EthereumSignature(packed_signature) => Ok(packed_signature),
            TxEthSignature::EIP1271Signature(..) => Err(EthSignerError::CustomError(
                "Can't sign tx message with EIP1271 signer".to_string(),
            )),
            // TxLayer1Signature::StarkSignature(..) => Err(SignerError::CustomError(
            //     "Can't sign tx message with Stark signer".to_string(),
            // )),
        }?;

        Ok(eth_signature)
    }

    // pub async fn sign_change_pub_key_ecdsa_auth_data(&self, tx: &ChangePubKey, chain_config: &ChainResp) -> Result<PackedEthSignature, SignerError> {
    //     let sign_bytes = tx.
    //         get_eth_eip712_signed_data_of_chain(chain_config.layer_one_chain_id, &chain_config.main_contract)
    //         .map_err(signing_failed_error)?;
    //
    //     let eth_signer = self
    //         .eth_signer
    //         .as_ref()
    //         .ok_or(SignerError::MissingEthSigner)?;
    //
    //     // sign_bytes is a eip712 data, use sign_raw_message
    //     let eth_signature = eth_signer
    //         .sign_raw_message(sign_bytes.into())
    //         .await
    //         .map_err(signing_failed_error)?;
    //
    //     let eth_signature = match eth_signature {
    //         TxEthSignature::EthereumSignature(packed_signature) => Ok(packed_signature),
    //         TxEthSignature::EIP1271Signature(..) => Err(EthSignerError::CustomError(
    //             "Can't sign ChangePubKey message with EIP1271 signer".to_string(),
    //         )),
    //         // TxLayer1Signature::StarkSignature(..) => Err(SignerError::CustomError(
    //         //     "Can't sign ChangePubKey message with Stark signer".to_string(),
    //         // )),
    //     }?;

    //     Ok(eth_signature)
    // }
}
