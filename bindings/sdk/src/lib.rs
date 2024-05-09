mod crypto;
mod network;
mod type_convert;

use crate::crypto::{get_public_key_hash, verify_musig};
use crate::network::{zklink_main_net_url, zklink_test_net_url};

use zklink_sdk_signers::eth_signer::error::EthSignerError;
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::eth_signer::{Address, H256};

use zklink_sdk_signers::starknet_signer::error::StarkSignerError;
use zklink_sdk_signers::starknet_signer::{StarkEip712Signature, StarkSigner};

use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_sdk_signers::zklink_signer::public_key::PackedPublicKey;
use zklink_sdk_signers::zklink_signer::signature::{PackedSignature, ZkLinkSignature};

use zklink_sdk_types::basic_types::tx_hash::TxHash;
use zklink_sdk_types::basic_types::zklink_address::ZkLinkAddress;
use zklink_sdk_types::basic_types::GetBytes;
use zklink_sdk_types::basic_types::{
    AccountId, BigUint, BlockNumber, ChainId, EthBlockId, MarginId, Nonce, PairId, PriorityOpId,
    SlotId, SubAccountId, TimeStamp, TokenId,
};
use zklink_sdk_types::error::TypeError;
use zklink_sdk_types::prelude::*;
use zklink_sdk_types::tx_type::{TxTrait, ZkSignatureTrait};

use zklink_sdk_interface::error::SignError;
use zklink_sdk_interface::sign_change_pubkey::{
    create_signed_change_pubkey, eth_signature_of_change_pubkey,
};
use zklink_sdk_interface::signer::L1SignerType;
use zklink_sdk_interface::signer::{L1Type, Signer};
use zklink_sdk_interface::ChangePubKeyAuthRequest;

use zklink_sdk_signers::starknet_signer::typed_data::message::Message;
use zklink_sdk_signers::starknet_signer::typed_data::message::TxMessage;
use zklink_sdk_signers::starknet_signer::typed_data::message::TypedDataMessage;
use zklink_sdk_signers::starknet_signer::typed_data::TypedData;

cfg_if::cfg_if! {
    if #[cfg(feature = "golang")] {
        include!(concat!(env!("OUT_DIR"), "/ffi.uniffi.rs"));
    } else if #[cfg(feature = "python")] {
        uniffi_macros::include_scaffolding!("ffi");
    }
}
