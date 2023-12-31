use crate::basic_types::{AccountId, ChainId, Nonce, SlotId, SubAccountId, TokenId};

/// Maximum precision of token amount
pub const TOKEN_MAX_PRECISION: u8 = 18;
/// Maximum number of chains allowed => The width of every token chain partition.(global asset tree)
pub const MAX_CHAIN_ID: ChainId = ChainId(u8::pow(2, CHAIN_SUB_TREE_DEPTH as u32) - 1);
pub const MAX_SUB_ACCOUNT_ID: SubAccountId =
    SubAccountId(u8::pow(2, SUB_ACCOUNT_TREE_DEPTH as u32) - 1);
/// Depth of sub-account tree allowed (be used for multiple different partition dex).
pub const SUB_ACCOUNT_TREE_DEPTH: usize = 5;
/// Depth of the orders subtree for each account.
pub const ORDER_SUB_TREE_DEPTH: usize = 16;
/// Depth of the chains subtree for global asset tree(located GLOBAL_ASSET_ACCOUNT_ID's balance tree, sub_account_id => chain_id).
pub const CHAIN_SUB_TREE_DEPTH: usize = SUB_ACCOUNT_TREE_DEPTH;
/// Depth of the account subtree that used for the current circuit chunk branch
pub const USED_ACCOUNT_SUBTREE_DEPTH: usize = 24;
/// Depth of the position subtree that used for the current circuit construct available position subtree
pub const USED_POSITION_SUBTREE_DEPTH: usize = 2;
pub const USED_BALANCE_SUBTREE_DEPTH: usize = 16;
/// The total account number and maximum account id allowed for the current zklink layer2(if not enough, modify this parameter and update circuit).
pub const TOTAL_ACCOUNT_NUMBER: usize = usize::pow(2, USED_ACCOUNT_SUBTREE_DEPTH as u32);
pub const MAX_ACCOUNT_ID: AccountId = AccountId(TOTAL_ACCOUNT_NUMBER as u32 - 1);
/// The total token number and maximum token id allowed for the current zklink layer2(if not enough, modify this parameter and update circuit).
pub const TOTAL_TOKEN_NUMBER: usize = usize::pow(2, USED_BALANCE_SUBTREE_DEPTH as u32);
pub const MAX_TOKEN_ID: TokenId = TokenId(TOTAL_TOKEN_NUMBER as u32 - 1);
/// One slot is a leaf of order subtree, slot number = 2 ^ ORDER_SUB_TREE_DEPTH, max slot id = slot number - 1
pub const TOTAL_SLOT_NUMBER: usize = usize::pow(2, ORDER_SUB_TREE_DEPTH as u32);
pub const MAX_SLOT_ID: SlotId = SlotId(TOTAL_SLOT_NUMBER as u32 - 1);

/// contract pair number bit width
pub const PAIR_BIT_WIDTH: usize = 8;
pub const PAIR_SYMBOL_BYTES: usize = 15;
/// slot number bit width
pub const SLOT_BIT_WIDTH: usize = 16;
/// Order nonce bit width
pub const ORDER_NONCE_BIT_WIDTH: usize = 24;
pub const MAX_ORDER_NONCE: Nonce = Nonce(u32::pow(2, ORDER_NONCE_BIT_WIDTH as u32) - 1);
pub const CHAIN_ID_BIT_WIDTH: usize = 8;
pub const ACCOUNT_ID_BIT_WIDTH: usize = 32;
pub const SUB_ACCOUNT_ID_BIT_WIDTH: usize = 8;
pub const PRICE_BIT_WIDTH: usize = 120;
pub const MIN_PRICE: u128 = 1;
/// decimals of price in order will be improved with TOKEN_MAX_PRECISION(18)
/// the bit width of price in pubdata is PRICE_BIT_WIDTH(120)
/// so the max price of price that order can submit is
/// (2 ** 120 - 1) / 10 ^18 = 1329227995784915872
pub const MAX_PRICE: u128 = 1329227995784915872000000000000000000;

pub const TOKEN_BIT_WIDTH: usize = 16;
pub const TX_TYPE_BIT_WIDTH: usize = 8;
/// balance bit width
pub const BALANCE_BIT_WIDTH: usize = 128;
/// The maximum bit width allowed by multiplication and division
pub const NEW_PUBKEY_HASH_BYTES_LEN: usize = 20;
pub const NEW_PUBKEY_HASH_WIDTH: usize = NEW_PUBKEY_HASH_BYTES_LEN * 8;

/// Nonce bit width
pub const NONCE_BIT_WIDTH: usize = 32;
pub const MAX_NONCE: Nonce = Nonce(u32::MAX);
pub const LAYER1_ADDR_BIT_WIDTH: usize = 256;
/// Amount bit widths
pub const AMOUNT_BIT_WIDTH: usize = AMOUNT_EXPONENT_BIT_WIDTH + AMOUNT_MANTISSA_BIT_WIDTH;
pub const AMOUNT_EXPONENT_BIT_WIDTH: usize = 5;
pub const AMOUNT_MANTISSA_BIT_WIDTH: usize = 35;

/// Fee bit widths
pub const FEE_BIT_WIDTH: usize = FEE_EXPONENT_BIT_WIDTH + FEE_MANTISSA_BIT_WIDTH;
pub const FEE_EXPONENT_BIT_WIDTH: usize = 5;
pub const FEE_MANTISSA_BIT_WIDTH: usize = 11;

/// Timestamp bit width
pub const SIMP_TIMESTAMP_BIT_WIDTH: usize = 4 * 8;

/// Fr element encoding
pub const FR_BIT_WIDTH: usize = 254;
/// Size of the data that is signed for withdraw tx
pub const SIGNED_WITHDRAW_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + CHAIN_ID_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + SUB_ACCOUNT_ID_BIT_WIDTH
    + LAYER1_ADDR_BIT_WIDTH
    + 2 * TOKEN_BIT_WIDTH
    + BALANCE_BIT_WIDTH
    + 2 * FEE_EXPONENT_BIT_WIDTH
    + 2 * FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + 8 // withdraw to l1
    + SIMP_TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for transfer tx
pub const SIGNED_TRANSFER_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + 2 * SUB_ACCOUNT_ID_BIT_WIDTH
    + LAYER1_ADDR_BIT_WIDTH
    + TOKEN_BIT_WIDTH
    + AMOUNT_EXPONENT_BIT_WIDTH
    + AMOUNT_MANTISSA_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + SIMP_TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for forced exit tx
pub const SIGNED_FORCED_EXIT_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + CHAIN_ID_BIT_WIDTH
    + 2 * SUB_ACCOUNT_ID_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + LAYER1_ADDR_BIT_WIDTH
    + 2 * TOKEN_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + BALANCE_BIT_WIDTH
    + 8 // withdraw to l1
    + SIMP_TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for change pubkey tx
pub const SIGNED_CHANGE_PUBKEY_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + CHAIN_ID_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + SUB_ACCOUNT_ID_BIT_WIDTH
    + NEW_PUBKEY_HASH_WIDTH
    + TOKEN_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + SIMP_TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for order_matching tx
pub const SIGNED_ORDER_MATCHING_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + SUB_ACCOUNT_ID_BIT_WIDTH
    + FR_BIT_WIDTH / 8 * 8
    + TOKEN_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + 2 * BALANCE_BIT_WIDTH;

/// Size of the data that is signed for contract_matching tx
pub const SIGNED_CONTRACT_MATCHING_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + SUB_ACCOUNT_ID_BIT_WIDTH
    + FR_BIT_WIDTH / 8 * 8
    + TOKEN_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH;

/// Size of the data that is signed for spot order
pub const SIGNED_ORDER_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + SUB_ACCOUNT_ID_BIT_WIDTH
    + SLOT_BIT_WIDTH
    + ORDER_NONCE_BIT_WIDTH
    + 2 * TOKEN_BIT_WIDTH
    + PRICE_BIT_WIDTH
    + 2 * FEE_RATIO_BIT_WIDTH
    + AMOUNT_BIT_WIDTH
    + HAS_SUBSIDY_BIT_WIDTH
    + IS_SELL_BIT_WIDTH; // order -> is_sell

/// Size of the data that is signed for contract order
pub const SIGNED_CONTRACT_ORDER_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + SUB_ACCOUNT_ID_BIT_WIDTH
    + SLOT_BIT_WIDTH
    + ORDER_NONCE_BIT_WIDTH
    + PAIR_BIT_WIDTH
    + PRICE_BIT_WIDTH
    + 2 * FEE_RATIO_BIT_WIDTH
    + HAS_SUBSIDY_BIT_WIDTH
    + AMOUNT_BIT_WIDTH
    + 8; // contract -> direction
pub const CONTRACT_BYTES: usize = SIGNED_CONTRACT_ORDER_BIT_WIDTH / 8;

/// Size of the data that is signed for contract order
pub const SIGNED_LIQUIDATION_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH * 2
    + SUB_ACCOUNT_ID_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + FR_BIT_WIDTH / 8 * 8
    + TOKEN_BIT_WIDTH
    + FEE_BIT_WIDTH;

/// Size of the data that is signed for contract order
pub const SIGNED_AUTO_DELEVERAGING_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + SUB_ACCOUNT_ID_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + FR_BIT_WIDTH / 8 * 8
    + ACCOUNT_ID_BIT_WIDTH
    + PAIR_BIT_WIDTH
    + AMOUNT_BIT_WIDTH
    + PRICE_BIT_WIDTH
    + TOKEN_BIT_WIDTH
    + FEE_BIT_WIDTH;

/// Size of the data that is signed for single funding
pub const SIGNED_FUNDING_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + SUB_ACCOUNT_ID_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + TOKEN_BIT_WIDTH
    + FEE_BIT_WIDTH;

/// Size of the data that is signed for batch funding
pub const SIGNED_BATCH_FUNDING_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + SUB_ACCOUNT_ID_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + FR_BIT_WIDTH / 8 * 8
    + TOKEN_BIT_WIDTH
    + FEE_BIT_WIDTH;

/// 0 can not be used as token id
pub const TOKEN_ID_ZERO: u32 = 0;
pub const CONTRACT_PRICE_BIT_WIDTH: usize = PRICE_BIT_WIDTH + PAIR_BIT_WIDTH;
pub const CONTRACT_PRICE_BYTES: usize = CONTRACT_PRICE_BIT_WIDTH / 8;
pub const MARGIN_PRICE_BIT_WIDTH: usize = PRICE_BIT_WIDTH + TOKEN_BIT_WIDTH;
pub const MARGIN_PRICE_BYTES: usize = MARGIN_PRICE_BIT_WIDTH / 8;

pub const IS_SELL_BIT_WIDTH: usize = 8;
pub const HAS_SUBSIDY_BIT_WIDTH: usize = 8;
/// The required length for rescue hash in the SDK.
pub const ORDERS_BIT_WIDTH: usize = 1424;
pub const ORDERS_BYTES: usize = ORDERS_BIT_WIDTH / 8;
pub const RESCUE_HASH_INPUT_BYTES: usize = ORDERS_BIT_WIDTH / 8;

pub const USD_TOKEN_ID: u32 = 1;
pub const USDX_TOKEN_ID_LOWER_BOUND: u32 = USD_TOKEN_ID + 1;
pub const USDX_TOKEN_ID_UPPER_BOUND: u32 = 16;
pub const MARGIN_TOKENS_NUMBER: usize = 3;
pub const USED_POSITION_NUMBER: usize = 2usize.pow(USED_POSITION_SUBTREE_DEPTH as u32);
pub const USED_POSITION_PAIR_ID_RANGE: std::ops::Range<u8> = 0..USED_POSITION_NUMBER as u8;

/// The account used to store the remaining assets of the tokens for contracts of layer1.
/// The token balances of this account are used in withdraw to layer one or create exit proof.
///
/// There are two kind of accounts:
/// * Normal account(id = \[0, 2-MAX_ACCOUNT_ID\])
/// * Global asset account(id = 1)
pub const GLOBAL_ASSET_ACCOUNT_ID: AccountId = AccountId(1);
/// All fee related values
pub const FEE_RATIO_BIT_WIDTH: usize = 8;
pub const SIGNED_FUNDING_RATE_BIT_WIDTH: usize = 16;
pub const FUNDING_RATE_BYTES: usize = SIGNED_FUNDING_RATE_BIT_WIDTH / 8;
