#![allow(unused_doc_comments)]
use crate::basic_types::pack::{is_fee_amount_packable, is_token_amount_packable};
use crate::basic_types::params::{
    GLOBAL_ASSET_ACCOUNT_ID, MARGIN_TOKENS_NUMBER, MAX_ACCOUNT_ID, MAX_CHAIN_ID, MAX_NONCE, MAX_ORDER_NONCE, MAX_PRICE,
    MAX_SLOT_ID, MAX_SUB_ACCOUNT_ID, MAX_TOKEN_ID, MIN_PRICE, USDX_TOKEN_ID_LOWER_BOUND, USDX_TOKEN_ID_UPPER_BOUND,
    USED_POSITION_NUMBER, USED_POSITION_PAIR_ID_RANGE,
};
use crate::params::{PAIR_SYMBOL_BYTES, WITHDRAW_FEE_RATIO_DENOMINATOR};
use crate::prelude::{
    AccountId, ChainId, ContractPrice, Nonce, PairId, Parameter, SlotId, SpotPriceInfo, SubAccountId, TokenId,
    ZkLinkAddress,
};
use num::{BigUint, Zero};
pub use validator::{Validate, ValidationError};

/// Check transaction account value validation
///
/// - account id should <= MAX_ACCOUNT_ID
/// - account id should not be GLOBAL_ASSET_ACCOUNT_ID(not invalid in transaction)
pub fn account_validator(account_id: &AccountId) -> Result<(), ValidationError> {
    if *account_id > MAX_ACCOUNT_ID {
        return Err(ValidationError::new("account id out of range"));
    }
    if *account_id == GLOBAL_ASSET_ACCOUNT_ID {
        return Err(ValidationError::new("account eq GLOBAL_ASSET_ACCOUNT_ID"));
    }
    Ok(())
}

/// Check transaction sub_account value validation
///
/// - sub_account id should <= MAX_SUB_ACCOUNT_ID
pub fn sub_account_validator(sub_account_id: &SubAccountId) -> Result<(), ValidationError> {
    if *sub_account_id > MAX_SUB_ACCOUNT_ID {
        return Err(ValidationError::new("sub_account id out of range"));
    }
    Ok(())
}

/// Check layer1 unpackable amount value validation
///
/// - amount should <= u128::MAX
pub fn amount_unpackable(amount: &BigUint) -> Result<(), ValidationError> {
    if *amount > BigUint::from(u128::MAX) {
        return Err(ValidationError::new("amount out of range"));
    }
    Ok(())
}

/// Check layer1 packable amount value validation
///
/// - amount should <= 34359738367000000000000000000000000000u128
/// - amount should keep same after pack and unpack
pub fn amount_packable(amount: &BigUint) -> Result<(), ValidationError> {
    if !is_token_amount_packable(amount) {
        return Err(ValidationError::new("amount is not packable"));
    }
    Ok(())
}

/// Check layer1 packable amount value validation
///
/// - fee should <= 20470000000000000000000000000000000u128
/// - fee should keep same after pack and unpack
pub fn fee_packable(fee: &BigUint) -> Result<(), ValidationError> {
    if !is_fee_amount_packable(fee) {
        return Err(ValidationError::new("fee is not packable"));
    }
    Ok(())
}

/// Check token value validation
///
/// - token id should <= MAX_TOKEN_ID
/// - token id should not use 0 and [2,16]
pub fn token_validator(token_id: &TokenId) -> Result<(), ValidationError> {
    if *token_id > MAX_TOKEN_ID {
        return Err(ValidationError::new("token id out of range"));
    }
    if **token_id >= USDX_TOKEN_ID_LOWER_BOUND && **token_id <= USDX_TOKEN_ID_UPPER_BOUND {
        return Err(ValidationError::new("token id should not use 0 or [2, 16]"));
    }
    Ok(())
}

/// Check contract token pair value validation
///
/// - pair id should <= MAX_TOKEN_ID
/// - pair id should not use 0
pub fn pair_validator(pair_id: &PairId) -> Result<(), ValidationError> {
    if !USED_POSITION_PAIR_ID_RANGE.contains(&(**pair_id as u8)) {
        return Err(ValidationError::new("pair id out of range"));
    }
    Ok(())
}

/// Check zklink address value validation
///
/// - zklink address should not be 0 and GLOBAL_ASSET_ACCOUNT_ADDRESS 0xffffffffffffffffffffffffffffffffffffffff
pub fn zklink_address_validator(zklink_address: &ZkLinkAddress) -> Result<(), ValidationError> {
    if zklink_address.is_zero() {
        return Err(ValidationError::new("zklink address is 0"));
    }
    if zklink_address.is_global_account_address() {
        return Err(ValidationError::new("zklink address is global asset account address"));
    }
    Ok(())
}

/// Check chain id validation
///
/// - chain id should <= MAX_CHAIN_ID
pub fn chain_id_validator(chain_id: &ChainId) -> Result<(), ValidationError> {
    if *chain_id > MAX_CHAIN_ID {
        return Err(ValidationError::new("chain id out of range"));
    }
    Ok(())
}

/// Check boolean flag value validation
///
/// - boolean should be 0 or 1
pub fn boolean_validator(boolean: u8) -> Result<(), ValidationError> {
    if boolean > 1u8 {
        return Err(ValidationError::new("boolean value should be 0 or 1"));
    }
    Ok(())
}

/// Check direction flag value validation
///
/// - direction should <= 1
pub fn direction_validator(direction: u8) -> Result<(), ValidationError> {
    if direction > 1u8 {
        return Err(ValidationError::new("direction value should be 0 or 1"));
    }
    Ok(())
}

/// Check ratio value validation
///
/// - withdraw rate should <= 10000(withdraw rate 100.00%)
pub fn withdraw_fee_ratio_validator(ratio: u16) -> Result<(), ValidationError> {
    if ratio > WITHDRAW_FEE_RATIO_DENOMINATOR {
        return Err(ValidationError::new("ratio out of range"));
    }
    Ok(())
}

/// Check margin currency ratio value validation
///
/// - margin_ratio should <= 100
pub fn margin_rate_validator(margin_ratio: u8) -> Result<(), ValidationError> {
    if margin_ratio > 100u8 {
        return Err(ValidationError::new("margin ratio out of range"));
    }
    Ok(())
}

/// Check contracts prices infos
///
/// - contracts_prices must be ordered by pair_id from smallest to largest
pub fn contract_prices_validator(contracts_prices: &[ContractPrice]) -> Result<(), ValidationError> {
    if contracts_prices.len() != USED_POSITION_NUMBER {
        return Err(ValidationError::new("contract prices number mismatch"));
    }
    for (info, pair_id) in contracts_prices.iter().zip(0..USED_POSITION_NUMBER) {
        if let Err(e) = info.validate() {
            return Err(ValidationError::new(e.into_errors().into_keys().last().unwrap()));
        }
        if *info.pair_id != pair_id as u16 {
            return Err(ValidationError::new("contracts prices array are wrong order"));
        }
    }
    Ok(())
}

/// Check margin price infos
///
/// - The tokens of margin_prices and margin tokens must correspond to each other in order
pub fn margin_prices_validator(margin_prices: &[SpotPriceInfo]) -> Result<(), ValidationError> {
    if margin_prices.len() != MARGIN_TOKENS_NUMBER {
        return Err(ValidationError::new("margin prices token mismatch"));
    }
    for info in margin_prices.iter() {
        if let Err(e) = info.validate() {
            return Err(ValidationError::new(e.into_errors().into_keys().last().unwrap()));
        }
    }
    Ok(())
}

/// Check order matching price value validation
///
/// - price should > MIN_PRICE
/// - price should < MAX_PRICE
pub fn price_validator(price: &BigUint) -> Result<(), ValidationError> {
    if *price <= BigUint::from(MIN_PRICE) || *price >= BigUint::from(MAX_PRICE) {
        return Err(ValidationError::new("price value out of range"));
    }
    Ok(())
}

/// Check contract matching price value validation
///
/// - price should < MAX_PRICE
pub fn external_price_validator(price: &BigUint) -> Result<(), ValidationError> {
    if *price >= BigUint::from(MAX_PRICE) {
        return Err(ValidationError::new("price value out of range"));
    }
    Ok(())
}

/// Check slot id validation
///
/// - slot_id should <= MAX_SLOT_ID
pub fn slot_id_validator(slot_id: &SlotId) -> Result<(), ValidationError> {
    if *slot_id > MAX_SLOT_ID {
        return Err(ValidationError::new("slot id out of range"));
    }
    Ok(())
}

/// Check nonce validation
///
/// - nonce should < MAX_NONCE
pub fn nonce_validator(nonce: &Nonce) -> Result<(), ValidationError> {
    if *nonce >= MAX_NONCE {
        return Err(ValidationError::new("The nonce has reached its maximum."));
    }
    Ok(())
}

/// Check order nonce validation
///
/// - nonce should < MAX_ORDER_NONCE
pub fn order_nonce_validator(nonce: &Nonce) -> Result<(), ValidationError> {
    if *nonce >= MAX_ORDER_NONCE {
        return Err(ValidationError::new("The order nonce has reached its maximum."));
    }
    Ok(())
}

/// Check funding rate validation
///
/// - funding rate should not eq i16::MIN(for the convenience of the circuit)
pub fn funding_rate_validator(rate: i16) -> Result<(), ValidationError> {
    if rate == i16::MIN {
        return Err(ValidationError::new("The funding rate disables i16 minimum value"));
    }
    Ok(())
}

/// Check parameter validation
///
pub fn parameter_validator(param: &Parameter) -> Result<(), ValidationError> {
    match param {
        Parameter::FundingInfos { infos } => {
            if infos.len() != USED_POSITION_NUMBER {
                return Err(ValidationError::new("update funding infos number mismatch"));
            }
            for funding_info in infos {
                if let Err(e) = funding_info.validate() {
                    return Err(ValidationError::new(e.into_errors().into_keys().last().unwrap()));
                }
            }
        }
        Parameter::FeeAccount { account_id } | Parameter::InsuranceFundAccount { account_id } => {
            account_validator(account_id)?
        }
        Parameter::MarginInfo {
            margin_id,
            token_id,
            ratio,
        } => {
            if **margin_id >= MARGIN_TOKENS_NUMBER as u8 {
                return Err(ValidationError::new("margin id out of range"));
            }
            token_validator(token_id)?;
            margin_rate_validator(*ratio)?;
        }
        Parameter::ContractInfo {
            pair_id,
            symbol,
            initial_margin_rate,
            maintenance_margin_rate,
        } => {
            pair_validator(pair_id)?;
            if !symbol.is_ascii() {
                return Err(ValidationError::new("pair symbol are not ascii chars"));
            }
            if symbol.len() > PAIR_SYMBOL_BYTES {
                return Err(ValidationError::new("pair symbol chars length out of range"));
            }
            if *initial_margin_rate >= 1000 || *maintenance_margin_rate >= 1000 {
                return Err(ValidationError::new("initial or maintenance margin rate out of range"));
            }
        }
    }
    Ok(())
}

/// Check adl size validation
///
/// 0 < adl_size <= u128::MAX
pub fn adl_size_unpackable(size: &BigUint) -> Result<(), ValidationError> {
    if size.is_zero() {
        return Err(ValidationError::new("adl size is 0"));
    }
    amount_unpackable(size)
}

#[cfg(test)]
mod validators_tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_account_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "account_validator")]
            pub account_id: AccountId,
        }

        impl Mock {
            pub fn new(account_id: AccountId) -> Self {
                Self { account_id }
            }
        }
        /// should success
        let mock = Mock::new(MAX_ACCOUNT_ID);
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(MAX_ACCOUNT_ID + 1);
        assert!(mock.validate().is_err());
        /// invalid
        let mock = Mock::new(GLOBAL_ASSET_ACCOUNT_ID);
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_sub_account_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "sub_account_validator")]
            pub sub_account_id: SubAccountId,
        }

        impl Mock {
            pub fn new(sub_account_id: SubAccountId) -> Self {
                Self { sub_account_id }
            }
        }
        /// should success
        let mock = Mock::new(MAX_SUB_ACCOUNT_ID);
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(MAX_SUB_ACCOUNT_ID + 1);
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_amount_unpackable() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "amount_unpackable")]
            pub amount: BigUint,
        }

        impl Mock {
            pub fn new(amount: BigUint) -> Self {
                Self { amount }
            }
        }
        /// should success
        let mock = Mock::new(BigUint::from(u128::MAX));
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(BigUint::from(u128::MAX) + BigUint::from(1u128));
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_amount_packable() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "amount_packable")]
            pub amount: BigUint,
        }

        impl Mock {
            pub fn new(amount: BigUint) -> Self {
                Self { amount }
            }
        }
        /// should success
        let mock = Mock::new(BigUint::from(34359738367000000000000000000000000000u128));
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(BigUint::from(34359738367000000000000000000000000001u128));
        assert!(mock.validate().is_err());
        /// unpackable
        let mock = Mock::new(BigUint::from(34359738366999999999999999999999999999u128));
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_fee_packable() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "fee_packable")]
            pub fee: BigUint,
        }

        impl Mock {
            pub fn new(fee: BigUint) -> Self {
                Self { fee }
            }
        }
        /// should success
        let mock = Mock::new(BigUint::from(20470000000000000000000000000000000u128));
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(BigUint::from(20469999999999999999999999999999999u128));
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_token_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "token_validator")]
            pub token_id: TokenId,
        }

        impl Mock {
            pub fn new(token_id: TokenId) -> Self {
                Self { token_id }
            }
        }
        /// should success
        let mock = Mock::new(MAX_TOKEN_ID);
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(MAX_TOKEN_ID + 1);
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_zklink_address_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "zklink_address_validator")]
            pub zklink_address: ZkLinkAddress,
        }

        impl Mock {
            pub fn new(zklink_address: ZkLinkAddress) -> Self {
                Self { zklink_address }
            }
        }
        /// should success
        let v1: Vec<u8> = vec![1; 32];
        let mock = Mock::new(ZkLinkAddress::from(v1));
        assert!(mock.validate().is_ok());
        /// out of range
        let v2: Vec<u8> = vec![0; 32];
        let mock = Mock::new(ZkLinkAddress::from(v2));
        assert!(mock.validate().is_err());
        let v3: Vec<u8> = vec![0xff; 32];
        let mock = Mock::new(ZkLinkAddress::from(v3));
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_boolean_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "boolean_validator")]
            pub boolean: u8,
        }

        impl Mock {
            pub fn new(boolean: u8) -> Self {
                Self { boolean }
            }
        }
        /// should success
        let mock = Mock::new(0);
        assert!(mock.validate().is_ok());
        let mock = Mock::new(1);
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(2);
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_chain_id_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "chain_id_validator")]
            pub chain_id: ChainId,
        }

        impl Mock {
            pub fn new(chain_id: ChainId) -> Self {
                Self { chain_id }
            }
        }
        /// should success
        let mock = Mock::new(MAX_CHAIN_ID);
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(MAX_CHAIN_ID + 1);
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_withdraw_fee_ratio_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "withdraw_fee_ratio_validator")]
            pub withdraw_fee_ratio: u16,
        }

        impl Mock {
            pub fn new(withdraw_fee_ratio: u16) -> Self {
                Self { withdraw_fee_ratio }
            }
        }
        /// should success
        let mock = Mock::new(10000u16);
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(10001u16);
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_price_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "price_validator")]
            pub price: BigUint,
        }

        impl Mock {
            pub fn new(price: BigUint) -> Self {
                Self { price }
            }
        }
        /// should success
        let mock = Mock::new(BigUint::from(MIN_PRICE + 1));
        assert!(mock.validate().is_ok());
        let mock = Mock::new(BigUint::from(MAX_PRICE - 1));
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(BigUint::from(MIN_PRICE));
        assert!(mock.validate().is_err());
        let mock = Mock::new(BigUint::from(MAX_PRICE));
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_external_price_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "external_price_validator")]
            pub price: BigUint,
        }

        impl Mock {
            pub fn new(price: BigUint) -> Self {
                Self { price }
            }
        }
        /// should success
        let mock = Mock::new(BigUint::from(0u8));
        assert!(mock.validate().is_ok());
        let mock = Mock::new(BigUint::from(MAX_PRICE - 1));
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(BigUint::from(MAX_PRICE));
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_slot_id_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "slot_id_validator")]
            pub slot_id: SlotId,
        }

        impl Mock {
            pub fn new(slot_id: SlotId) -> Self {
                Self { slot_id }
            }
        }
        /// should success
        let mock = Mock::new(MAX_SLOT_ID);
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(MAX_SLOT_ID + 1);
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_nonce_validate() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "nonce_validator")]
            pub nonce: Nonce,
        }

        impl Mock {
            pub fn new(nonce: Nonce) -> Self {
                Self { nonce }
            }
        }
        /// should success
        let mock = Mock::new(MAX_NONCE - 1);
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(MAX_NONCE);
        assert!(mock.validate().is_err());
    }

    #[test]
    fn test_adl_size_unpackable() {
        #[derive(Debug, Validate)]
        struct Mock {
            #[validate(custom = "adl_size_unpackable")]
            pub adl_size: BigUint,
        }

        impl Mock {
            pub fn new(adl_size: BigUint) -> Self {
                Self { adl_size }
            }
        }
        /// should success
        let mock = Mock::new(BigUint::from(u128::MAX));
        assert!(mock.validate().is_ok());
        /// out of range
        let mock = Mock::new(BigUint::from(u128::MAX) + BigUint::from(1u128));
        assert!(mock.validate().is_err());
        /// is zero
        let mock = Mock::new(BigUint::zero());
        assert!(mock.validate().is_err());
    }
}
