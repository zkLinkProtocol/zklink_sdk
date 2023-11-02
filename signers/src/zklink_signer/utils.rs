use super::RESCUE_PARAMS;
use super::{Engine, Fr};
use franklin_crypto::{
    bellman::{pairing::ff::PrimeField, BitIterator},
    circuit::multipack,
    rescue::rescue_hash,
};

const PAD_MSG_BEFORE_HASH_BITS_LEN: usize = 736;

pub fn bytes_into_be_bits(bytes: &[u8]) -> Vec<bool> {
    let mut bits = Vec::with_capacity(bytes.len() * 8);
    for byte in bytes {
        let mut temp = *byte;
        for _ in 0..8 {
            bits.push(temp & 0x80 == 0x80);
            temp <<= 1;
        }
    }
    bits
}

pub fn pack_bits_into_bytes(bits: &[bool]) -> Vec<u8> {
    let mut message_bytes: Vec<u8> = Vec::with_capacity(bits.len() / 8);
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

pub fn pack_bits_into_bytes_le(bits: &[bool]) -> Vec<u8> {
    let mut message_bytes: Vec<u8> = Vec::with_capacity(bits.len() / 8);
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

pub fn append_le_fixed_width(content: &mut Vec<bool>, x: &Fr, width: usize) {
    let mut token_bits: Vec<bool> = BitIterator::new(x.into_repr()).collect();
    token_bits.reverse();
    token_bits.resize(width, false);
    content.extend(token_bits);
}

fn rescue_hash_fr(input: Vec<bool>) -> Fr {
    RESCUE_PARAMS.with(|params| {
        let packed = multipack::compute_multipacking::<Engine>(&input);
        let sponge_output = rescue_hash::<Engine>(params, &packed);
        assert_eq!(sponge_output.len(), 1, "rescue hash problem");
        sponge_output[0]
    })
}

pub(crate) fn rescue_hash_elements(input: &[Fr]) -> Fr {
    RESCUE_PARAMS.with(|params| {
        let sponge_output = rescue_hash::<Engine>(params, input);
        assert_eq!(sponge_output.len(), 1, "rescue hash problem");
        sponge_output[0]
    })
}

pub fn rescue_hash_tx_msg(msg: &[u8]) -> Vec<u8> {
    let mut msg_bits = bytes_into_be_bits(msg);
    assert!(msg_bits.len() <= PAD_MSG_BEFORE_HASH_BITS_LEN);
    msg_bits.resize(PAD_MSG_BEFORE_HASH_BITS_LEN, false);
    let hash_fr = rescue_hash_fr(msg_bits);
    let mut hash_bits = Vec::new();
    append_le_fixed_width(&mut hash_bits, &hash_fr, 256);
    pack_bits_into_bytes(&hash_bits)
}

fn get_bits_le_fixed(fr: &Fr, size: usize) -> Vec<bool> {
    let mut bits: Vec<bool> = Vec::with_capacity(size);
    let repr = fr.into_repr();
    let repr: &[u64] = repr.as_ref();
    let n = std::cmp::min(repr.len() * 64, size);
    for i in 0..n {
        let part = i / 64;
        let bit = i - (64 * part);
        bits.push(repr[part] & (1 << bit) > 0);
    }
    let n = bits.len();
    bits.extend((n..size).map(|_| false));
    bits
}

pub fn rescue_hash_orders(msg: &[u8]) -> Vec<u8> {
    assert_eq!(msg.len(), 178);
    let msg_bits = bytes_into_be_bits(msg);
    let hash_fr = rescue_hash_fr(msg_bits);
    let hash_bits = get_bits_le_fixed(&hash_fr, 248);
    pack_bits_into_bytes_le(&hash_bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rescue_hash_tx_msg() {
        let msg = [1u8, 2u8, 3u8, 4u8];
        let hash = rescue_hash_tx_msg(&msg);

        assert_eq!(
            hash,
            vec![
                249, 154, 208, 123, 96, 89, 132, 235, 231, 63, 56, 200, 153, 131, 27, 183, 128, 71,
                26, 245, 208, 120, 49, 246, 233, 72, 230, 84, 66, 150, 170, 27
            ]
        );
    }

    #[test]
    fn test_rescue_hash_orders() {
        let msg = [1u8; 178];
        let hash = rescue_hash_orders(&msg);
        assert_eq!(
            hash,
            vec![
                165, 52, 198, 24, 171, 190, 215, 122, 29, 12, 31, 190, 98, 145, 72, 245, 89, 202,
                199, 73, 239, 213, 234, 218, 74, 182, 95, 119, 141, 75, 253
            ]
        );
    }
}
