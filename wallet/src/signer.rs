use crate::error::ClientError;
use crate::error::ClientError::EthSigningError;
use num::BigUint;
use zklink_interface::{ChangePubKeyAuthRequest, TxSignature};
use zklink_signers::eth_signer::error::EthSignerError;
use zklink_signers::eth_signer::error::EthSignerError::MissingEthSigner;
use zklink_signers::eth_signer::eth_signature::TxEthSignature;
use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_signers::eth_signer::pk_signer::PrivateKeySigner;
use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_types::basic_types::{
    AccountId, ChainId, Nonce, SlotId, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use zklink_types::tx_type::change_pubkey::{ChangePubKey, ChangePubKeyAuthData};
use zklink_types::tx_type::forced_exit::ForcedExit;
use zklink_types::tx_type::order_matching::{Order, OrderMatching};
use zklink_types::tx_type::transfer::Transfer;
use zklink_types::tx_type::withdraw::Withdraw;
// Local imports

fn signing_failed_error(err: impl ToString) -> EthSignerError {
    EthSignerError::SigningFailed(err.to_string())
}

pub struct Signer {
    pub zklink_signer: ZkLinkSigner,
    pub(crate) eth_signer: Option<PrivateKeySigner>,
    pub pub_key_hash: PubKeyHash,
    pub(crate) account_id: Option<AccountId>,
}

impl Signer {
    pub fn new(
        pk_bytes: &[u8],
        eth_signer: Option<PrivateKeySigner>,
    ) -> Result<Self, ZkSignerError> {
        let zklink_signer = ZkLinkSigner::new_from_bytes(pk_bytes)?;
        let pub_key_hash = zklink_signer.public_key().public_key_hash();

        Ok(Self {
            zklink_signer,
            eth_signer,
            pub_key_hash,
            account_id: None,
        })
    }

    pub fn set_account_id(&mut self, account_id: Option<AccountId>) {
        self.account_id = account_id;
    }

    pub fn get_account_id(&self) -> Option<AccountId> {
        self.account_id
    }

    pub fn sign_layer_two_message(&self, message: &[u8]) -> Result<ZkLinkSignature, ZkSignerError> {
        Ok(self.zklink_signer.sign_musig(message)?)
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
            .map_err(signing_failed_error)?;

        Ok(eth_signature)
    }

    pub async fn sign_change_pub_key_ecdsa_auth_data(
        &self,
        tx: &ChangePubKey,
        l1_client_id: u32,
        main_contract: &ZkLinkAddress,
    ) -> Result<PackedEthSignature, ClientError> {
        let typed_data = tx.to_eip712_request_payload(l1_client_id, &main_contract)?;
        let eth_signer = self
            .eth_signer
            .as_ref()
            .ok_or(EthSigningError(MissingEthSigner))?;

        // sign_bytes is a eip712 data, use sign_raw_message
        let eth_signature = eth_signer
            .sign_byted_data(&typed_data.data_hash)
            .map_err(signing_failed_error)?;

        Ok(eth_signature)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn sign_change_pub_key(
        &self,
        account_id: AccountId,
        chain_id: ChainId,
        sub_account_id: SubAccountId,
        fee_token: TokenId,
        fee: BigUint,
        new_pubkey_hash: &[u8],
        nonce: Nonce,
        main_contract: ZkLinkAddress,
        l1_client_id: u32,
        account_address: ZkLinkAddress,
        auth_request: ChangePubKeyAuthRequest,
        ts: TimeStamp,
    ) -> Result<TxSignature, ClientError> {
        let new_pk_hash = PubKeyHash::from_bytes(new_pubkey_hash)?;
        let mut tx = ChangePubKey {
            chain_id,
            account_id,
            sub_account_id,
            new_pk_hash,
            fee_token,
            fee,
            nonce,
            signature: Default::default(),
            eth_auth_data: ChangePubKeyAuthData::OnChain,
            ts,
        };
        let eth_auth_data: Result<ChangePubKeyAuthData, ClientError> = match auth_request {
            ChangePubKeyAuthRequest::OnChain => Ok(ChangePubKeyAuthData::OnChain),
            ChangePubKeyAuthRequest::EthECDSA => {
                let eth_signature = self
                    .sign_change_pub_key_ecdsa_auth_data(&tx, l1_client_id, &main_contract)
                    .await?;

                Ok(ChangePubKeyAuthData::EthECDSA { eth_signature })
            }
            ChangePubKeyAuthRequest::EthCreate2 { data: create2 } => {
                // check create2 data
                let from_address = create2.get_address(self.pub_key_hash.data.to_vec());
                if from_address.as_bytes() != account_address.as_bytes() {
                    Err(ClientError::IncorrectTx)
                } else {
                    Ok(ChangePubKeyAuthData::EthCreate2 { data: create2 })
                }
            }
        };
        tx.eth_auth_data = eth_auth_data?;
        tx.signature = self.sign_layer_two_message(&tx.get_bytes())?;

        Ok(TxSignature {
            tx: tx.into(),
            eth_signature: None,
        })
    }

    pub async fn sign_transfer(
        &self,
        account_id: AccountId,
        from_sub_account_id: SubAccountId,
        to: ZkLinkAddress,
        to_sub_account_id: SubAccountId,
        token_id: TokenId,
        token_symbol: String,
        amount: BigUint,
        fee: BigUint,
        nonce: Nonce,
        ts: TimeStamp,
    ) -> Result<TxSignature, ClientError> {
        let mut tx = Transfer {
            account_id,
            from_sub_account_id,
            to,
            to_sub_account_id,
            token: token_id,
            amount,
            fee,
            nonce,
            signature: Default::default(),
            ts,
        };

        tx.signature = self.sign_layer_two_message(&tx.get_bytes())?;

        let message = tx
            .get_ethereum_sign_message(&token_symbol)
            .as_bytes()
            .to_vec();
        let eth_signature = self.sign_layer_one_message(&message.as_slice()).await?;

        Ok(TxSignature {
            tx: tx.into(),
            eth_signature: Some(eth_signature),
        })
    }

    pub async fn sign_withdraw(
        &self,
        account_id: AccountId,
        to_chain_id: ChainId,
        sub_account_id: SubAccountId,
        to: ZkLinkAddress,
        l2_source_token_id: TokenId,
        l2_source_token_symbol: String,
        l1_target_token_id: TokenId,
        amount: BigUint,
        fee: BigUint,
        nonce: Nonce,
        fast_withdraw: bool,
        withdraw_fee_ratio: u16,
        ts: TimeStamp,
    ) -> Result<TxSignature, ClientError> {
        let fast_withdraw = if fast_withdraw { 1u8 } else { 0u8 };

        let mut tx = Withdraw {
            to_chain_id,
            account_id,
            sub_account_id,
            to,
            l2_source_token: l2_source_token_id,
            l1_target_token: l1_target_token_id,
            amount,
            fee,
            nonce,
            fast_withdraw,
            withdraw_fee_ratio,
            signature: Default::default(),
            ts,
        };

        tx.signature = self.sign_layer_two_message(&tx.get_bytes())?;

        let message = tx.get_ethereum_sign_message(&l2_source_token_symbol);
        let message = message.as_bytes().to_vec();
        let eth_signature = self.sign_layer_one_message(&message.as_slice()).await?;

        Ok(TxSignature {
            tx: tx.into(),
            eth_signature: Some(eth_signature),
        })
    }

    pub async fn sign_forced_exit(
        &self,
        account_id: AccountId,
        to_chain_id: ChainId,
        sub_account_id: SubAccountId,
        target: ZkLinkAddress,
        target_sub_account_id: SubAccountId,
        l2_source_token_id: TokenId,
        l1_target_token_id: TokenId,
        nonce: Nonce,
        exit_amount: BigUint,
        ts: TimeStamp,
    ) -> Result<TxSignature, ClientError> {
        let mut tx = ForcedExit {
            to_chain_id,
            initiator_account_id: account_id,
            initiator_sub_account_id: sub_account_id,
            initiator_nonce: nonce,
            target,
            target_sub_account_id,
            l2_source_token: l2_source_token_id,
            l1_target_token: l1_target_token_id,
            exit_amount,
            signature: Default::default(),
            ts,
        };

        tx.signature = self.sign_layer_two_message(&tx.get_bytes())?;
        Ok(TxSignature {
            tx: tx.into(),
            eth_signature: None,
        })
    }

    pub fn sign_order(
        &self,
        account_id: AccountId,
        sub_account_id: SubAccountId,
        slot_id: SlotId,
        nonce: Nonce,
        base_token_id: TokenId,
        quote_token_id: TokenId,
        amount: BigUint,
        price: BigUint,
        is_sell: bool,
        fee_ratio1: u8,
        fee_ratio2: u8,
    ) -> Result<Order, ClientError> {
        let is_sell = if is_sell { 1u8 } else { 0u8 };

        let mut order = Order {
            account_id,
            sub_account_id,
            slot_id,
            nonce,
            base_token_id,
            quote_token_id,
            amount,
            price,
            is_sell,
            fee_ratio1,
            fee_ratio2,
            signature: Default::default(),
        };

        order.signature = self.sign_layer_two_message(&order.get_bytes())?;
        Ok(order)
    }

    pub async fn sign_order_matching(
        &self,
        account_id: AccountId,
        sub_account_id: SubAccountId,
        taker: Order,
        maker: Order,
        fee_token_id: TokenId,
        fee: BigUint,
        expect_base_amount: BigUint,
        expect_quote_amount: BigUint,
    ) -> Result<TxSignature, ClientError> {
        let mut tx = OrderMatching {
            account_id,
            sub_account_id,
            taker,
            maker,
            fee_token: fee_token_id,
            fee,
            expect_base_amount,
            expect_quote_amount,
            signature: Default::default(), // left unset because fee is unknown now
        };

        tx.signature = self.sign_layer_two_message(&tx.get_bytes())?;
        Ok(TxSignature {
            tx: tx.into(),
            eth_signature: None,
        })
    }
}
