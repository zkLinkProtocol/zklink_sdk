use crate::error::ClientError;
use web3::types::H256;
use zklink_crypto::eth_signer::eth_signature::TxEthSignature;
use zklink_crypto::eth_signer::pk_signer::PrivateKeySigner;
use zklink_crypto::eth_signer::EthereumSigner;
use zklink_crypto::zklink_signer::pk_signer::ZkLinkSigner;

pub struct WalletCredentials<S: EthereumSigner> {
    pub(crate) eth_signer: Option<S>,
    pub(crate) zklink_signer: ZkLinkSigner,
}

impl<S: EthereumSigner> WalletCredentials<S> {
    /// Creates wallet credentials from the provided Ethereum signer.
    ///
    /// ## Arguments
    ///
    /// - `eth_signer`: Abstract signer that signs messages and transactions.
    /// - `network`: Network this wallet is used on.
    pub async fn from_eth_signer(eth_signer: S) -> Result<Self, ClientError> {
        // Pre-defined message to generate seed from.
        const MESSAGE: &str =
            "Sign this message to create a private key to interact with zkLink's layer 2 services.\nNOTE: This application is powered by zkLink's multi-chain network.\n\nOnly sign this message for a trusted client!";

        let eth_sign_message = format!("{}", MESSAGE).into_bytes();

        let signature = eth_signer
            .sign_message(eth_sign_message.clone().as_slice())
            .map_err(ClientError::EthSigningError)?;

        let packed_signature =
            if let TxEthSignature::EthereumSignature(packed_signature) = signature {
                packed_signature
            } else {
                return Err(ClientError::IncorrectCredentials);
            };

        // Generate seed, and then zkSync private key.
        let signature_bytes = packed_signature.serialize_packed();
        let zklink_signer = ZkLinkSigner::new_from_seed(&signature_bytes)?;

        Ok(Self {
            eth_signer: Some(eth_signer),
            zklink_signer,
        })
    }

    /// Creates wallet credentials from the provided seed.
    /// zkSync private key will be randomly generated and Ethereum signer will not be set.
    /// Wallet created with such credentials won't be capable of performing on-chain operations,
    /// such as deposits and full exits.
    ///
    /// ## Arguments
    ///
    /// - `eth_address`: Address of the corresponding Ethereum wallet.
    /// - `seed`: A random bytearray to generate private key from. Must be at least 32 bytes long.
    pub fn from_seed(seed: &[u8]) -> Result<Self, ClientError> {
        let zklink_signer = ZkLinkSigner::new_from_seed(seed)?;

        Ok(Self {
            eth_signer: None,
            zklink_signer,
        })
    }

    /// Creates wallet credentials from the provided keys.
    ///
    /// ## Arguments
    ///
    /// - `private_key`: Private key of a zkSync account.
    /// - `eth_private_key`: Private key of a corresponding Ethereum wallet. If not set, on-chain operations won't be allowed for Wallet.
    pub fn from_pk(
        private_key: &[u8],
        eth_private_key: Option<H256>,
    ) -> Result<WalletCredentials<PrivateKeySigner>, ClientError> {
        let eth_signer = eth_private_key.map(PrivateKeySigner::new);
        let zklink_signer = ZkLinkSigner::new_from_bytes(private_key)?;

        Ok(WalletCredentials {
            eth_signer,
            zklink_signer,
        })
    }
}
