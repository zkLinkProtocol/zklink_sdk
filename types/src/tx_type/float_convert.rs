use super::error::FloatConvertError as Error;
use num::{BigUint, ToPrimitive};

pub struct BitConvert;

impl BitConvert {
    /// Сonverts a set of bits to a set of bytes in direct order.
    #[allow(clippy::wrong_self_convention)]
    pub fn into_bytes(bits: Vec<bool>) -> Vec<u8> {
        assert_eq!(bits.len() % 8, 0);
        let mut message_bytes: Vec<u8> = vec![];

        let byte_chunks = bits.chunks(8);
        for byte_chunk in byte_chunks {
            let mut byte = 0u8;
            for (i, bit) in byte_chunk.iter().enumerate() {
                if *bit {
                    byte |= 1 << i;
                }
            }
            message_bytes.push(byte);
        }

        message_bytes
    }

    /// Сonverts a set of bits to a set of bytes in reverse order for each byte.
    #[allow(clippy::wrong_self_convention)]
    pub fn into_bytes_ordered(bits: Vec<bool>) -> Vec<u8> {
        assert_eq!(bits.len() % 8, 0);
        let mut message_bytes: Vec<u8> = vec![];

        let byte_chunks = bits.chunks(8);
        for byte_chunk in byte_chunks {
            let mut byte = 0u8;
            for (i, bit) in byte_chunk.iter().rev().enumerate() {
                if *bit {
                    byte |= 1 << i;
                }
            }
            message_bytes.push(byte);
        }

        message_bytes
    }

    /// Сonverts a set of Big Endian bytes to a set of bits.
    pub fn from_be_bytes(bytes: &[u8]) -> Vec<bool> {
        let mut bits = vec![];
        for byte in bytes {
            let mut temp = *byte;
            for _ in 0..8 {
                bits.push(temp & 0x80 == 0x80);
                temp <<= 1;
            }
        }
        bits
    }
}
/// Convert Uint to the floating-point and vice versa.
pub struct FloatConversions;

impl FloatConversions {
    /// Packs a BigUint less than 2 ^ 128 to a floating-point number with an exponent base = 10 that is less or equal to initial number.
    /// Can lose accuracy with small parameters `exponent_len` and `mantissa_len`.
    pub fn pack(number: &BigUint, exponent_len: usize, mantissa_len: usize) -> Vec<u8> {
        let uint = number.to_u128().expect("Only u128 allowed");

        let mut vec = Self::to_float(uint, exponent_len, mantissa_len, 10).expect("packing error");
        vec.reverse();
        BitConvert::into_bytes_ordered(vec)
    }

    /// Packs a BigUint less than 2 ^ 128 to a floating-point number with an exponent base = 10 that is greater or equal to initial number.
    /// Can lose accuracy with small parameters `exponent_len` and `mantissa_len`.
    pub fn pack_up(number: &BigUint, exponent_len: usize, mantissa_len: usize) -> Vec<u8> {
        let uint = number.to_u128().expect("Only u128 allowed");

        let mut vec =
            Self::to_float_up(uint, exponent_len, mantissa_len, 10).expect("packing error");
        vec.reverse();
        BitConvert::into_bytes_ordered(vec)
    }

    /// Unpacks a floating point number with the given parameters.
    /// Returns `None` for numbers greater than 2 ^ 128.
    pub fn unpack(data: &[u8], exponent_len: usize, mantissa_len: usize) -> Option<u128> {
        if exponent_len + mantissa_len != data.len() * 8 {
            return None;
        }

        let bits = BitConvert::from_be_bytes(data);

        let mut mantissa = 0u128;
        for (i, bit) in bits[0..mantissa_len].iter().rev().enumerate() {
            if *bit {
                mantissa = mantissa.checked_add(1u128 << i)?;
            }
        }

        let mut exponent_pow = 0u32;
        for (i, bit) in bits[mantissa_len..(mantissa_len + exponent_len)]
            .iter()
            .rev()
            .enumerate()
        {
            if *bit {
                exponent_pow = exponent_pow.checked_add(1u32 << i)?;
            }
        }

        let exponent = 10u128.checked_pow(exponent_pow)?;

        mantissa.checked_mul(exponent)
    }

    /// Packs a u128 to a floating-point number with the given parameters that is less or equal to integer.
    /// Can lose accuracy with small parameters `exponent_len` and `mantissa_len`.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_float(
        integer: u128,
        exponent_length: usize,
        mantissa_length: usize,
        exponent_base: u32,
    ) -> Result<Vec<bool>, Error> {
        let exponent_base = u128::from(exponent_base);

        let max_power = (1 << exponent_length) - 1;

        let max_exponent = exponent_base.saturating_pow(max_power);

        let max_mantissa = (1u128 << mantissa_length) - 1;

        if integer > (max_mantissa.saturating_mul(max_exponent)) {
            return Err(Error::TooBigInteger);
        }

        // The algortihm is as follows: calculate minimal exponent
        // such that integer <= max_mantissa * exponent_base ^ exponent,
        // then if this minimal exponent is 0 we can choose mantissa equals integer and exponent equals 0
        // else we need to check two variants:
        // 1) with that minimal exponent
        // 2) with that minimal exponent minus 1
        let mut exponent: usize = 0;
        let mut exponent_temp: u128 = 1;
        while integer > max_mantissa * exponent_temp {
            exponent_temp *= exponent_base;
            exponent += 1;
        }
        let (exponent, mantissa) = if exponent == 0 {
            (0, integer)
        } else {
            let mantissa = integer / exponent_temp;
            let variant1 = mantissa * exponent_temp;
            let variant2 = max_mantissa * exponent_temp / exponent_base;
            let diff1 = integer - variant1;
            let diff2 = integer - variant2;
            if diff1 < diff2 {
                (exponent, mantissa)
            } else {
                (exponent - 1, max_mantissa)
            }
        };

        // encode into bits. First bits of mantissa in LE order

        let mut encoding = Vec::with_capacity(exponent_length + mantissa_length);

        for i in 0..exponent_length {
            if exponent & (1 << i) != 0 {
                encoding.push(true);
            } else {
                encoding.push(false);
            }
        }

        for i in 0..mantissa_length {
            if mantissa & (1 << i) != 0 {
                encoding.push(true);
            } else {
                encoding.push(false);
            }
        }

        debug_assert_eq!(encoding.len(), exponent_length + mantissa_length);
        Ok(encoding)
    }

    /// Packs a u128 to a floating-point number with the given parameters that is greater or equal to integer.
    /// Can lose accuracy with small parameters `exponent_len` and `mantissa_len`.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_float_up(
        integer: u128,
        exponent_length: usize,
        mantissa_length: usize,
        exponent_base: u32,
    ) -> Result<Vec<bool>, Error> {
        let exponent_base = u128::from(exponent_base);

        let max_power = (1 << exponent_length) - 1;

        let max_exponent = exponent_base.saturating_pow(max_power);

        let max_mantissa = (1u128 << mantissa_length) - 1;

        if integer > (max_mantissa.saturating_mul(max_exponent)) {
            return Err(Error::TooBigInteger);
        }

        // The algortihm is as follows: calculate minimal exponent
        // such that integer <= max_mantissa * exponent_base ^ exponent,
        // then mantissa is calculated as integer divided by exponent_base ^ exponent and rounded up
        let mut exponent: usize = 0;
        let mut exponent_temp: u128 = 1;
        while integer > max_mantissa * exponent_temp {
            exponent_temp *= exponent_base;
            exponent += 1;
        }
        let mut mantissa = integer / exponent_temp;
        if integer % exponent_temp != 0 {
            mantissa += 1;
        }

        // encode into bits. First bits of mantissa in LE order

        let mut encoding = Vec::with_capacity(exponent_length + mantissa_length);

        for i in 0..exponent_length {
            if exponent & (1 << i) != 0 {
                encoding.push(true);
            } else {
                encoding.push(false);
            }
        }

        for i in 0..mantissa_length {
            if mantissa & (1 << i) != 0 {
                encoding.push(true);
            } else {
                encoding.push(false);
            }
        }

        debug_assert_eq!(encoding.len(), exponent_length + mantissa_length);
        Ok(encoding)
    }
}
