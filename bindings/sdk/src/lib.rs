use crate::crypto::{get_public_key_hash, verify_musig};
use crate::network::{zklink_main_net_url, zklink_test_net_url};
use crate::rpc_request_builder::{
    build_change_pubkey_request_with_create2data,
    build_change_pubkey_request_with_eth_ecdsa_auth_data,
    build_change_pubkey_request_with_onchain_auth_data, build_deposit_request,
    build_forced_exit_request, build_full_exit_request, build_order_matching_request,
    build_transfer_request, build_withdraw_request,
};

mod crypto;
mod network;
mod rpc_request_builder;
mod type_convert;
use chrono::{DateTime, Utc};

use zklink_signers::eth_signer::error::EthSignerError;
use zklink_signers::eth_signer::eth_signature::TxEthSignature;
use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_signers::eth_signer::pk_signer::PrivateKeySigner;
use zklink_signers::eth_signer::H256;

use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_signers::zklink_signer::public_key::PackedPublicKey;
use zklink_signers::zklink_signer::signature::{
    json_str_of_zklink_signature, PackedSignature, ZkLinkSignature,
};

use zklink_types::basic_types::error::TypeError;
use zklink_types::basic_types::tx_hash::TxHash;
use zklink_types::basic_types::zklink_address::ZkLinkAddress;
use zklink_types::basic_types::{
    AccountId, BigUint, BlockNumber, ChainId, EthBlockId, Nonce, PairId, PriorityOpId, SlotId,
    SubAccountId, TimeStamp, TokenId,
};
use zklink_types::tx_builder::{
    ChangePubKeyBuilder, DepositBuilder, ForecedExitBuilder, FullExitBuilder, OrderMatchingBuilder,
    TransferBuilder, WithdrawBuilder,
};
use zklink_types::tx_type::change_pubkey::ChangePubKey;
use zklink_types::tx_type::change_pubkey::ChangePubKeyAuthData;
use zklink_types::tx_type::change_pubkey::Create2Data;
use zklink_types::tx_type::deposit::Deposit;
use zklink_types::tx_type::forced_exit::ForcedExit;
use zklink_types::tx_type::full_exit::FullExit;
use zklink_types::tx_type::order_matching::Order;
use zklink_types::tx_type::order_matching::OrderMatching;
use zklink_types::tx_type::transfer::Transfer;
use zklink_types::tx_type::withdraw::Withdraw;

use zklink_interface::error::SignError;
use zklink_interface::sign_change_pubkey::{
    create_signed_change_pubkey, eth_signature_of_change_pubkey,
};
use zklink_interface::sign_forced_exit::create_signed_forced_exit;
use zklink_interface::sign_order::create_signed_order;
use zklink_interface::sign_order_matching::create_signed_order_matching;
use zklink_interface::sign_transfer::create_signed_transfer;
use zklink_interface::sign_withdraw::create_signed_withdraw;
use zklink_interface::ChangePubKeyAuthRequest;

type TimeStampMicro = DateTime<Utc>;

include!(concat!(env!("OUT_DIR"), "/ffi.uniffi.rs"));
