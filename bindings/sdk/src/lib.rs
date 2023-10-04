mod crypto;
mod network;
mod type_convert;
mod zklink_tx;

use crate::crypto::{get_public_key_hash, verify_musig};
use crate::network::{zklink_main_net_url, zklink_test_net_url};
use crate::zklink_tx::*;

use zklink_sdk_signers::eth_signer::error::EthSignerError;
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::eth_signer::{Address, H256};

use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_sdk_signers::zklink_signer::public_key::PackedPublicKey;
use zklink_sdk_signers::zklink_signer::signature::{PackedSignature, ZkLinkSignature};

use zklink_sdk_types::basic_types::tx_hash::TxHash;
use zklink_sdk_types::basic_types::zklink_address::ZkLinkAddress;
use zklink_sdk_types::basic_types::{
    AccountId, BigUint, BlockNumber, ChainId, EthBlockId, Nonce, PairId, PriorityOpId, SlotId,
    SubAccountId, TimeStamp, TokenId,
};
use zklink_sdk_types::error::TypeError;
use zklink_sdk_types::prelude::{
    ChangePubKey, ChangePubKeyAuthData, ChangePubKeyBuilder, Create2Data, Deposit, DepositBuilder,
    ForcedExit, ForcedExitBuilder, FullExit, FullExitBuilder, Order, OrderMatching,
    OrderMatchingBuilder, Transfer, TransferBuilder, TxLayer1Signature, TxSignature, Withdraw,
    WithdrawBuilder, ZkLinkTx,
};
use zklink_sdk_types::tx_type::{TxTrait, ZkSignatureTrait};

use zklink_sdk_interface::error::SignError;
use zklink_sdk_interface::sign_change_pubkey::{
    create_signed_change_pubkey, eth_signature_of_change_pubkey,
};
use zklink_sdk_interface::sign_forced_exit::create_signed_forced_exit;
use zklink_sdk_interface::sign_order::create_signed_order;
use zklink_sdk_interface::sign_order_matching::create_signed_order_matching;
use zklink_sdk_interface::sign_transfer::create_signed_transfer;
use zklink_sdk_interface::sign_withdraw::create_signed_withdraw;
use zklink_sdk_interface::signer::Signer;
use zklink_sdk_interface::ChangePubKeyAuthRequest;

include!(concat!(env!("OUT_DIR"), "/ffi.uniffi.rs"));
