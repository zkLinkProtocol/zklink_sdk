use crate::error::ClientError;
use crate::signer::Signer;
use crate::{AccountType, Wallet};
use num::BigUint;
use zklink_interface::{ChangePubKeyAuthRequest, TxSignature};
use zklink_provider::response::{AccountQuery, ChainResp, TokenResp};
use zklink_provider::rpc::ZkLinkRpcClient;
use zklink_signers::eth_signer::eth_signature::TxEthSignature;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;

use zklink_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_types::basic_types::params::MAIN_SUB_ACCOUNT_ID;
use zklink_types::basic_types::tx_hash::TxHash;
use zklink_types::basic_types::{AccountId, ChainId, Nonce, SubAccountId, TokenId, ZkLinkAddress};
use zklink_types::tx_type::order_matching::Order;

impl<P> Wallet<P>
where
    P: ZkLinkRpcClient + Sync + Clone,
{
    pub async fn new(
        provider: P,
        signer: Signer,
        address: ZkLinkAddress,
        account_type: AccountType,
    ) -> Result<Self, ClientError> {
        let chains = provider.get_support_chains().await?;
        let chains = chains.iter().map(|c| (c.chain_id, c.clone())).collect();
        let tokens = provider.tokens().await?;
        let account_info = provider
            .account_info(AccountQuery::Address(address.clone()))
            .await?;

        let wallet = Wallet {
            provider,
            signer,
            address,
            account_info,
            account_type,
            chains,
            tokens,
        };

        Ok(wallet)
    }

    /// Returns the wallet address.
    pub fn address(&self) -> ZkLinkAddress {
        self.address.clone()
    }

    /// Returns the current account id
    pub fn account_id(&self) -> AccountId {
        self.account_info.id
    }

    /// Returns the current account pub key hash
    pub fn account_pubkey_hash(&self) -> PubKeyHash {
        self.account_info.pub_key_hash
    }

    pub fn account_nonce(&self) -> Nonce {
        self.account_info.nonce
    }

    /// Updates account info stored in the wallet.
    pub async fn update_account_info(&mut self) -> Result<(), ClientError> {
        self.account_info = self
            .provider
            .account_info(AccountQuery::Address(self.address.clone()))
            .await?;

        Ok(())
    }

    /// Returns `true` if signing key for account was set in zkLink network.
    /// In other words, returns `true` if `ChangePubKey` operation was performed for the
    /// account.
    ///
    /// If this method has returned `false`, one must send a `ChangePubKey` transaction
    /// via `Wallet::start_change_pubkey` method.
    pub async fn is_signing_key_set(&self) -> Result<bool, ClientError> {
        let signer_pub_key_hash = self.signer.pub_key_hash;

        let key_set = self.account_pubkey_hash() == signer_pub_key_hash;
        Ok(key_set)
    }

    pub fn get_chain(&self, chain_id: &ChainId) -> Option<ChainResp> {
        self.chains.get(chain_id).cloned()
    }

    pub fn get_token(&self, token_id: &TokenId) -> Option<TokenResp> {
        self.tokens.get(token_id).cloned()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn submit_change_pub_key(
        &self,
        chain_id: ChainId,
        sub_account_id: SubAccountId,
        fee_token: TokenId,
        fee: Option<BigUint>,
        nonce: Option<Nonce>,
        new_pubkey_hash: &[u8],
        auth_request: ChangePubKeyAuthRequest,
    ) -> Result<TxSignature, ClientError> {
        let chain_config = self
            .get_chain(&chain_id)
            .ok_or_else(|| ClientError::NetworkNotSupported(chain_id))?;

        let account_id = self.account_id();

        let nonce = self.resolve_nonce(nonce);
        let current_time = chrono::Utc::now().timestamp() as u32;

        self.signer.sign_change_pub_key(
            account_id,
            chain_id,
            sub_account_id,
            fee_token,
            fee.unwrap_or_default(),
            Some(new_pubkey_hash),
            nonce,
            chain_config.main_contract,
            chain_config.layer_one_chain_id,
            self.address.clone(),
            auth_request,
            current_time.into(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn submit_transfer(
        &self,
        from_sub_account_id: SubAccountId,
        to: ZkLinkAddress,
        to_sub_account_id: SubAccountId,
        token_id: TokenId,
        amount: BigUint,
        fee: Option<BigUint>,
        nonce: Option<Nonce>,
    ) -> Result<TxSignature, ClientError> {
        let account_id = self.account_id();

        let token = self
            .get_token(&token_id)
            .ok_or_else(|| ClientError::UnknownToken)?;

        let nonce = self.resolve_nonce(nonce);
        let current_time = chrono::Utc::now().timestamp() as u32;

        self.signer.sign_transfer(
            account_id,
            from_sub_account_id,
            to,
            to_sub_account_id,
            token_id,
            token.symbol,
            amount,
            fee.unwrap_or_default(),
            nonce,
            current_time.into(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn submit_withdraw(
        &self,
        to_chain_id: ChainId,
        sub_account_id: SubAccountId,
        to: ZkLinkAddress,
        l2_source_token_id: TokenId,
        l1_target_token_id: TokenId,
        amount: BigUint,
        fee: Option<BigUint>,
        nonce: Option<Nonce>,
        fast_withdraw: bool,
        withdraw_fee_ratio: u16,
    ) -> Result<TxSignature, ClientError> {
        let account_id = self.account_id();

        let l2_source_token = self
            .get_token(&l2_source_token_id)
            .ok_or_else(|| ClientError::UnknownToken)?;

        let nonce = self.resolve_nonce(nonce);
        let current_time = chrono::Utc::now().timestamp() as u32;
        self.signer.sign_withdraw(
            account_id,
            to_chain_id,
            sub_account_id,
            to,
            l2_source_token_id,
            l2_source_token.symbol,
            l1_target_token_id,
            amount,
            fee.unwrap_or_default(),
            nonce,
            fast_withdraw,
            withdraw_fee_ratio,
            current_time.into(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn submit_forced_exit(
        &self,
        to_chain_id: ChainId,
        sub_account_id: SubAccountId,
        target: ZkLinkAddress,
        target_sub_account_id: SubAccountId,
        l2_source_token_id: TokenId,
        l1_target_token_id: TokenId,
        nonce: Option<Nonce>,
        exit_amount: BigUint,
    ) -> Result<TxSignature, ClientError> {
        let account_id = self.account_id();
        let nonce = self.resolve_nonce(nonce);
        let current_time = chrono::Utc::now().timestamp() as u32;

        self.signer.sign_forced_exit(
            account_id,
            to_chain_id,
            sub_account_id,
            target,
            target_sub_account_id,
            l2_source_token_id,
            l1_target_token_id,
            nonce,
            exit_amount,
            current_time.into(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn submit_order_matching(
        &self,
        sub_account_id: SubAccountId,
        taker: Order,
        maker: Order,
        fee_token_id: TokenId,
        fee: Option<BigUint>,
        expect_base_amount: BigUint,
        expect_quote_amount: BigUint,
    ) -> Result<TxSignature, ClientError> {
        let account_id = self.account_id();

        self.signer.sign_order_matching(
            account_id,
            sub_account_id,
            taker,
            maker,
            fee_token_id,
            fee.unwrap_or_default(),
            expect_base_amount,
            expect_quote_amount,
        )
    }

    fn resolve_nonce(&self, nonce: Option<Nonce>) -> Nonce {
        match nonce {
            Some(nonce) => nonce,
            None => self.account_nonce(),
        }
    }

    pub async fn submit_tx(
        &self,
        tx_signature: TxSignature,
        submitter_signer: Option<ZkLinkSigner>,
    ) -> Result<TxHash, ClientError> {
        let tx_l1_signature = match tx_signature.eth_signature {
            Some(eth_signature) => match self.account_type {
                AccountType::CREATE2 => Some(TxEthSignature::EIP1271Signature(eth_signature)),
                _ => Some(TxEthSignature::EthereumSignature(eth_signature)),
            },
            None => None,
        };

        if !tx_signature.tx.is_validate() {
            return Err(ClientError::IncorrectTx);
        }

        let reduced_sub_account = tx_signature.tx.asset_reduced_sub_account();
        // MAIN_SUB_ACCOUNT_ID doesn't need any submitter
        let submitter_signature = if reduced_sub_account.is_empty()
            || reduced_sub_account
                .iter()
                .all(|sid| *sid == MAIN_SUB_ACCOUNT_ID)
        {
            None
        } else {
            if let Some(signer) = submitter_signer {
                Some(signer.sign_musig(tx_signature.tx.hash().as_ref())?)
            } else {
                return Err(ClientError::MissSubmitterSigner);
            }
        };

        let tx_hash = self
            .provider
            .tx_submit(tx_signature.tx, tx_l1_signature, submitter_signature)
            .await?;
        Ok(tx_hash)
    }
}
