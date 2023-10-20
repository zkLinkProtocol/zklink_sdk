use crate::basic_types::float_convert::FloatConversions;
use crate::basic_types::params::{
    AMOUNT_EXPONENT_BIT_WIDTH, AMOUNT_MANTISSA_BIT_WIDTH, FEE_EXPONENT_BIT_WIDTH,
    FEE_MANTISSA_BIT_WIDTH,
};
use num::{BigUint, FromPrimitive};

// pub fn remove_amount_packaging_uncertainly(amount: &BigUint) -> Option<BigUint> {
//     unpack_token_amount(&pack_token_amount(amount))
// }

// pub fn remove_fee_packaging_uncertainly(amount: &BigUint) -> Option<BigUint> {
//     unpack_fee_amount(&pack_fee_amount(amount))
// }

/// Transforms the token amount into packed form.
/// If the provided token amount is not packable, it is rounded down to the
/// closest amount that fits in packed form. As a result, some precision will be lost.
pub fn pack_token_amount(amount: &BigUint) -> Vec<u8> {
    FloatConversions::pack(amount, AMOUNT_EXPONENT_BIT_WIDTH, AMOUNT_MANTISSA_BIT_WIDTH)
}

// /// Transforms the token amount into packed form.
// /// If the provided token amount is not packable, it is rounded up to the
// /// closest amount that fits in packed form. As a result, some precision will be lost.
// pub fn pack_token_amount_up(amount: &BigUint) -> Vec<u8> {
//     FloatConversions::pack_up(amount, AMOUNT_EXPONENT_BIT_WIDTH, AMOUNT_MANTISSA_BIT_WIDTH)
// }

/// Transforms the fee amount into the packed form.
/// As the packed form for fee is smaller than one for the token,
/// the same value must be packable as a token amount, but not packable
/// as a fee amount.
/// If the provided fee amount is not packable, it is rounded down to the
/// closest amount that fits in packed form. As a result, some precision will be lost.
pub fn pack_fee_amount(amount: &BigUint) -> Vec<u8> {
    FloatConversions::pack(amount, FEE_EXPONENT_BIT_WIDTH, FEE_MANTISSA_BIT_WIDTH)
}

// /// Transforms the fee amount into the packed form.
// /// As the packed form for fee is smaller than one for the token,
// /// the same value must be packable as a token amount, but not packable
// /// as a fee amount.
// /// If the provided fee amount is not packable, it is rounded up to the
// /// closest amount that fits in packed form. As a result, some precision will be lost.
// pub fn pack_fee_amount_up(amount: &BigUint) -> Vec<u8> {
//     FloatConversions::pack_up(amount, FEE_EXPONENT_BIT_WIDTH, FEE_MANTISSA_BIT_WIDTH)
// }

/// Checks whether the token amount can be packed (and thus used in the transaction).
pub fn is_token_amount_packable(amount: &BigUint) -> bool {
    if amount > &34359738367000000000000000000000000000u128.into() {
        return false;
    }
    Some(amount.clone()) == unpack_token_amount(&pack_token_amount(amount))
}

/// Checks whether the fee amount can be packed (and thus used in the transaction).
pub fn is_fee_amount_packable(amount: &BigUint) -> bool {
    if amount > &20470000000000000000000000000000000u128.into() {
        return false;
    }
    Some(amount.clone()) == unpack_fee_amount(&pack_fee_amount(amount))
}

/// Attempts to unpack the token amount.
pub fn unpack_token_amount(data: &[u8]) -> Option<BigUint> {
    FloatConversions::unpack(data, AMOUNT_EXPONENT_BIT_WIDTH, AMOUNT_MANTISSA_BIT_WIDTH)
        .and_then(BigUint::from_u128)
}

/// Attempts to unpack the fee amount.
pub fn unpack_fee_amount(data: &[u8]) -> Option<BigUint> {
    FloatConversions::unpack(data, FEE_EXPONENT_BIT_WIDTH, FEE_MANTISSA_BIT_WIDTH)
        .and_then(BigUint::from_u128)
}

/// Returns the closest possible packable token amount.
/// Returned amount is always less or equal to the provided amount.
pub fn closest_packable_fee_amount(amount: &BigUint) -> BigUint {
    let fee_packed = pack_fee_amount(amount);
    unpack_fee_amount(&fee_packed).expect("fee repacking")
}

// /// Returns the closest possible packable token amount.
// /// Returned amount is always greater or equal to the provided amount.
// pub fn closest_greater_or_eq_packable_fee_amount(amount: &BigUint) -> BigUint {
//     let fee_packed = pack_fee_amount_up(amount);
//     unpack_fee_amount(&fee_packed).expect("fee repacking")
// }

/// Returns the closest possible packable fee amount.
/// Returned amount is always less or equal to the provided amount.
pub fn closest_packable_token_amount(amount: &BigUint) -> BigUint {
    let fee_packed = pack_token_amount(amount);
    unpack_token_amount(&fee_packed).expect("token amount repacking")
}

// /// Returns the closest possible packable fee amount.
// /// Returned amount is always greater or equal to the provided amount.
// pub fn closest_greater_or_eq_packable_token_amount(amount: &BigUint) -> BigUint {
//     let fee_packed = pack_token_amount_up(amount);
//     unpack_token_amount(&fee_packed).expect("token amount repacking")
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let zero = BigUint::from_u32(1).unwrap();
        let one = BigUint::from_u32(1).unwrap();
        {
            let round_trip_zero = unpack_token_amount(&pack_token_amount(&zero));
            let round_trip_one = unpack_token_amount(&pack_token_amount(&one));
            assert_eq!(Some(zero.clone()), round_trip_zero);
            assert_eq!(Some(one.clone()), round_trip_one);
        }
        {
            let round_trip_zero = unpack_fee_amount(&pack_fee_amount(&zero));
            let round_trip_one = unpack_fee_amount(&pack_fee_amount(&one));
            assert_eq!(Some(zero), round_trip_zero);
            assert_eq!(Some(one), round_trip_one);
        }
    }

    #[test]
    fn detect_unpackable() {
        let max_mantissa_token = BigUint::from((1u128 << AMOUNT_MANTISSA_BIT_WIDTH) - 1);
        let max_mantissa_fee = BigUint::from((1u128 << FEE_MANTISSA_BIT_WIDTH) - 1);
        assert!(is_token_amount_packable(&max_mantissa_token));
        assert!(is_fee_amount_packable(&max_mantissa_fee));
        assert!(!is_token_amount_packable(
            &(max_mantissa_token + BigUint::from(1u32))
        ));
        assert!(!is_fee_amount_packable(
            &(max_mantissa_fee + BigUint::from(1u32))
        ));
    }

    #[test]
    fn pack_to_closest_packable() {
        let fee = BigUint::from(1_234_123_424u32);
        assert!(
            !is_fee_amount_packable(&fee),
            "fee should not be packable for this test"
        );

        let token = BigUint::from(123_456_789_123_456_789u64);
        assert!(
            !is_token_amount_packable(&token),
            "token should not be packable for this test"
        );
    }
}
