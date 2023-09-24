use std::sync::Arc;
use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey;
use zklink_sdk_types::tx_type::deposit::Deposit;
use zklink_sdk_types::tx_type::forced_exit::ForcedExit;
use zklink_sdk_types::tx_type::full_exit::FullExit;
use zklink_sdk_types::tx_type::order_matching::OrderMatching;
use zklink_sdk_types::tx_type::transfer::Transfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;

pub fn zklink_tx_from_change_pubkey(tx: Arc<ChangePubKey>) -> ZkLinkTx {
    (*tx).clone().into()
}

pub fn zklink_tx_from_transfer(tx: Arc<Transfer>) -> ZkLinkTx {
    (*tx).clone().into()
}

pub fn zklink_tx_from_deposit(tx: Arc<Deposit>) -> ZkLinkTx {
    (*tx).clone().into()
}

pub fn zklink_tx_from_withdraw(tx: Arc<Withdraw>) -> ZkLinkTx {
    (*tx).clone().into()
}

pub fn zklink_tx_from_forced_exit(tx: Arc<ForcedExit>) -> ZkLinkTx {
    (*tx).clone().into()
}

pub fn zklink_tx_from_full_exit(tx: Arc<FullExit>) -> ZkLinkTx {
    (*tx).clone().into()
}

pub fn zklink_tx_from_order_matching(tx: Arc<OrderMatching>) -> ZkLinkTx {
    (*tx).clone().into()
}
