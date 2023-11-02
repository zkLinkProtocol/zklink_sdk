pub struct BitConvert;

impl BitConvert {
    /// Сonverts a set of bits to a set of bytes in direct order.
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
