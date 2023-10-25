use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::basic_types::{tx_hash::TxHash, GetBytes, Nonce, SubAccountId};
use crate::prelude::{AutoDeleveraging, ContractMatching, Funding, Liquidation, UpdateGlobalVar};
use crate::tx_type::change_pubkey::ChangePubKey;
use crate::tx_type::deposit::Deposit;
use crate::tx_type::forced_exit::ForcedExit;
use crate::tx_type::full_exit::FullExit;
use crate::tx_type::order_matching::OrderMatching;
use crate::tx_type::transfer::Transfer;
use crate::tx_type::withdraw::Withdraw;
use crate::tx_type::TxTrait;

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
    ChangePubKey(Box<ChangePubKey>),
    ForcedExit(Box<ForcedExit>),
    OrderMatching(Box<OrderMatching>),
    ContractMatching(Box<ContractMatching>),
    Liquidation(Box<Liquidation>),
    AutoDeleveraging(Box<AutoDeleveraging>),
    UpdateGlobalVar(Box<UpdateGlobalVar>),
    Funding(Box<Funding>),
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

impl From<ContractMatching> for ZkLinkTx {
    fn from(tx: ContractMatching) -> Self {
        Self::ContractMatching(Box::new(tx))
    }
}

impl From<Liquidation> for ZkLinkTx {
    fn from(tx: Liquidation) -> Self {
        Self::Liquidation(Box::new(tx))
    }
}

impl From<AutoDeleveraging> for ZkLinkTx {
    fn from(tx: AutoDeleveraging) -> Self {
        Self::AutoDeleveraging(Box::new(tx))
    }
}

impl From<UpdateGlobalVar> for ZkLinkTx {
    fn from(tx: UpdateGlobalVar) -> Self {
        Self::UpdateGlobalVar(Box::new(tx))
    }
}

impl From<Funding> for ZkLinkTx {
    fn from(tx: Funding) -> Self {
        Self::Funding(Box::new(tx))
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
            ZkLinkTx::ContractMatching(tx) => tx.validate(),
            ZkLinkTx::Liquidation(tx) => tx.validate(),
            ZkLinkTx::AutoDeleveraging(tx) => tx.validate(),
            ZkLinkTx::UpdateGlobalVar(tx) => tx.validate(),
            ZkLinkTx::Funding(tx) => tx.validate(),
        }
    }

    /// Check tx correct
    pub fn is_valid(&self) -> bool {
        match self {
            ZkLinkTx::Transfer(tx) => tx.is_valid(),
            ZkLinkTx::Withdraw(tx) => tx.is_valid(),
            ZkLinkTx::ChangePubKey(tx) => tx.is_valid(),
            ZkLinkTx::ForcedExit(tx) => tx.is_valid(),
            ZkLinkTx::OrderMatching(tx) => tx.is_valid(),
            ZkLinkTx::Deposit(tx) => tx.is_valid(),
            ZkLinkTx::FullExit(tx) => tx.is_valid(),
            ZkLinkTx::ContractMatching(tx) => tx.is_valid(),
            ZkLinkTx::Liquidation(tx) => tx.is_valid(),
            ZkLinkTx::AutoDeleveraging(tx) => tx.is_valid(),
            ZkLinkTx::UpdateGlobalVar(tx) => tx.is_valid(),
            ZkLinkTx::Funding(tx) => tx.is_valid(),
        }
    }

    /// Returns the hash of the transaction.
    pub fn tx_hash(&self) -> TxHash {
        let tx_hash = match self {
            ZkLinkTx::Transfer(tx) => tx.tx_hash(),
            ZkLinkTx::Withdraw(tx) => tx.tx_hash(),
            ZkLinkTx::ChangePubKey(tx) => tx.tx_hash(),
            ZkLinkTx::ForcedExit(tx) => tx.tx_hash(),
            ZkLinkTx::Deposit(tx) => tx.tx_hash(),
            ZkLinkTx::FullExit(tx) => tx.tx_hash(),
            ZkLinkTx::OrderMatching(tx) => tx.tx_hash(),
            ZkLinkTx::ContractMatching(tx) => tx.get_bytes(),
            ZkLinkTx::Liquidation(tx) => tx.get_bytes(),
            ZkLinkTx::AutoDeleveraging(tx) => tx.get_bytes(),
            ZkLinkTx::UpdateGlobalVar(tx) => tx.get_bytes(),
            ZkLinkTx::Funding(tx) => tx.get_bytes(),
        };

        let mut out = [0u8; 32];
        out.copy_from_slice(&tx_hash);
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
            ZkLinkTx::Liquidation(tx) => vec![tx.sub_account_id],
            ZkLinkTx::ContractMatching(tx) => vec![tx.sub_account_id],
            ZkLinkTx::AutoDeleveraging(tx) => vec![tx.sub_account_id],
            ZkLinkTx::Funding(tx) => vec![tx.sub_account_id],
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
            ZkLinkTx::FullExit(tx) => Nonce((tx.serial_id & 0xffffffff) as u32),
            ZkLinkTx::Deposit(tx) => Nonce((tx.serial_id & 0xffffffff) as u32),
            ZkLinkTx::Liquidation(tx) => tx.sub_account_nonce,
            ZkLinkTx::AutoDeleveraging(tx) => tx.sub_account_nonce,
            ZkLinkTx::UpdateGlobalVar(tx) => Nonce((tx.serial_id & 0xffffffff) as u32),
            ZkLinkTx::Funding(tx) => tx.sub_account_nonce,
            _ => Nonce(u32::MAX),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zklink_tx_deserde() {
        let s = r#"{"type":"ChangePubKey","chainId":1,"accountId":2,"subAccountId":4,"newPkHash":"0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe","feeToken":1,"fee":"100","nonce":100,"signature":{"pubKey":"0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510","signature":"8ae9ee90a7c19ced45bb70cf9cee0b392659cea87891a5021fe558f8e18b8680865ca2f008f75ba69146e7b5412c223d1339443aa4be18a9f62142fdefb79600"},"ethAuthData":{"type":"EthECDSA","ethSignature":"0x66eeec379a192c64ac44bf3b2cafbb0ebb2fca8c7c1699095599e8173d618e860dae34989661497834cd89bf5e5772bda322050a4d8d958011d192eda69df8dc1b"},"ts":1695105758}"#;
        let tx: Result<ZkLinkTx, _> = serde_json::from_str(s);
        println!("{tx:?}");
        assert!(tx.is_ok());
    }
}
