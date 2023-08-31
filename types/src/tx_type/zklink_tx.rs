use parity_crypto::digest::sha256;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::basic_types::{tx_hash::TxHash, Nonce, SubAccountId};
use crate::tx_type::change_pubkey::ChangePubKey;
use crate::tx_type::deposit::Deposit;
use crate::tx_type::forced_exit::ForcedExit;
use crate::tx_type::full_exit::FullExit;
use crate::tx_type::order_matching::OrderMatching;
use crate::tx_type::transfer::Transfer;
use crate::tx_type::withdraw::Withdraw;

/// A set of L2 transaction type supported by the zklink network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZkLinkTxType {
    Deposit,
    FullExit,
    ChangePubKey,
    Transfer,
    Withdraw,
    ForcedExit,
    OrderMatching,
}

impl ZkLinkTxType {
    pub fn op_codes(&self) -> Vec<u8> {
        match self {
            ZkLinkTxType::Deposit => vec![Deposit::TX_TYPE],
            ZkLinkTxType::Transfer => vec![Transfer::TX_TYPE],
            ZkLinkTxType::Withdraw => vec![Withdraw::TX_TYPE],
            ZkLinkTxType::FullExit => vec![FullExit::TX_TYPE],
            ZkLinkTxType::ChangePubKey => vec![ChangePubKey::TX_TYPE],
            ZkLinkTxType::ForcedExit => vec![ForcedExit::TX_TYPE],
            ZkLinkTxType::OrderMatching => vec![OrderMatching::TX_TYPE],
        }
    }
}

/// A set of L2 transaction supported by the zklink network.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ZkLinkTx {
    FullExit(Box<FullExit>),
    Deposit(Box<Deposit>),
    Transfer(Box<Transfer>),
    Withdraw(Box<Withdraw>),
    #[doc(hidden)]
    ChangePubKey(Box<ChangePubKey>),
    ForcedExit(Box<ForcedExit>),
    OrderMatching(Box<OrderMatching>),
}

impl From<FullExit> for ZkLinkTx {
    fn from(full_exit: FullExit) -> Self {
        Self::FullExit(Box::new(full_exit))
    }
}

impl From<Deposit> for ZkLinkTx {
    fn from(deposit: Deposit) -> Self {
        Self::Deposit(Box::new(deposit))
    }
}

impl From<Transfer> for ZkLinkTx {
    fn from(transfer: Transfer) -> Self {
        Self::Transfer(Box::new(transfer))
    }
}

impl From<Withdraw> for ZkLinkTx {
    fn from(withdraw: Withdraw) -> Self {
        Self::Withdraw(Box::new(withdraw))
    }
}

impl From<ChangePubKey> for ZkLinkTx {
    fn from(change_pub_key: ChangePubKey) -> Self {
        Self::ChangePubKey(Box::new(change_pub_key))
    }
}

impl From<ForcedExit> for ZkLinkTx {
    fn from(tx: ForcedExit) -> Self {
        Self::ForcedExit(Box::new(tx))
    }
}

impl From<OrderMatching> for ZkLinkTx {
    fn from(tx: OrderMatching) -> Self {
        Self::OrderMatching(Box::new(tx))
    }
}

impl ZkLinkTx {
    /// Check tx format
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            ZkLinkTx::Transfer(tx) => tx.validate(),
            ZkLinkTx::Withdraw(tx) => tx.validate(),
            ZkLinkTx::ChangePubKey(tx) => tx.validate(),
            ZkLinkTx::ForcedExit(tx) => tx.validate(),
            ZkLinkTx::OrderMatching(tx) => tx.validate(),
            ZkLinkTx::Deposit(tx) => tx.validate(),
            ZkLinkTx::FullExit(tx) => tx.validate(),
        }
    }
    /// Returns the hash of the transaction.
    pub fn hash(&self) -> TxHash {
        let bytes = match self {
            ZkLinkTx::Transfer(tx) => tx.get_bytes(),
            ZkLinkTx::Withdraw(tx) => tx.get_bytes(),
            ZkLinkTx::ChangePubKey(tx) => tx.get_bytes(),
            ZkLinkTx::ForcedExit(tx) => tx.get_bytes(),
            ZkLinkTx::Deposit(tx) => tx.get_bytes(),
            ZkLinkTx::FullExit(tx) => tx.get_bytes(),
            ZkLinkTx::OrderMatching(tx) => tx.get_bytes(),
        };

        let hash = sha256(&bytes);
        let mut out = [0u8; 32];
        out.copy_from_slice(&hash);
        TxHash { data: out }
    }

    /// Return sub account ids which asset will be reduced
    /// * tx fee
    /// * transfer from
    /// * withdraw from
    ///
    /// used to check layer 2 tx submitter if exist in white list for special sub account
    pub fn asset_reduced_sub_account(&self) -> Vec<SubAccountId> {
        match self {
            // account pay fee
            // transfer from account
            ZkLinkTx::Transfer(tx) => vec![tx.from_sub_account_id],
            // account pay fee
            // withdraw from account
            ZkLinkTx::Withdraw(tx) => vec![tx.sub_account_id],
            // account pay fee
            ZkLinkTx::ChangePubKey(tx) => vec![tx.sub_account_id],
            // initiator pay fee
            // withdraw from target account
            ZkLinkTx::ForcedExit(tx) => vec![tx.initiator_sub_account_id, tx.target_sub_account_id],
            // account pay fee
            // sub account ids of order are same as tx.sub_account_id
            ZkLinkTx::OrderMatching(tx) => vec![tx.sub_account_id],
            _ => vec![],
        }
    }

    /// Returns the account nonce associated with transaction.
    pub fn nonce(&self) -> Nonce {
        match self {
            ZkLinkTx::Transfer(tx) => tx.nonce,
            ZkLinkTx::Withdraw(tx) => tx.nonce,
            ZkLinkTx::ChangePubKey(tx) => tx.nonce,
            ZkLinkTx::ForcedExit(tx) => tx.initiator_nonce,
            ZkLinkTx::OrderMatching(_tx) => Nonce(u32::MAX),
            ZkLinkTx::FullExit(tx) => Nonce((tx.serial_id & 0xffffffff) as u32),
            ZkLinkTx::Deposit(tx) => Nonce((tx.serial_id & 0xffffffff) as u32),
        }
    }

    /// Check tx correct
    pub fn is_validate(&self) -> bool {
        match self {
            ZkLinkTx::Transfer(tx) => tx.is_validate(),
            ZkLinkTx::Withdraw(tx) => tx.is_validate(),
            ZkLinkTx::ChangePubKey(tx) => tx.is_validate(),
            ZkLinkTx::ForcedExit(tx) => tx.is_validate(),
            ZkLinkTx::OrderMatching(tx) => tx.is_validate(),
            ZkLinkTx::FullExit(tx) => tx.is_validate(),
            ZkLinkTx::Deposit(tx) => tx.is_validate(),
        }
    }
}
