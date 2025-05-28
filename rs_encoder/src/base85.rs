pub mod base85 {
    use std::fmt::Display;

    use bincode::{self, Error};

    #[derive(Debug, Default)]
    pub struct Base85 {
        container: String,
    }
    impl Base85 {
        fn new() -> Self {
            return Self {
                container: (String::new()),
            };
        }

        fn encode<T: AsRef<[u8]>>(bytes: &T) -> Result<Self, Error> {
            let data = bytes.as_ref();
            let mut buffer: u32 = 0;
            let mut result = Self::new();

            // Helper to encode a 32-bit buffer into Base85 characters
            fn encode_chunk(mut buffer: u32, count: usize) -> String {
                let mut encoded = String::new();
                let output_len = if count == 4 { 5 } else { count + 1 };

                // If less than 4 bytes, pad by shifting left
                if count < 4 {
                    buffer <<= (4 - count) * 8;
                }

                if buffer == 0 {
                    // Special case for zero
                    encoded.push('z');
                    return encoded;
                }
                for i in 0..output_len as u32 {
                    let power = 85_u32.pow(4 - i);
                    let c = ((buffer / power) % 85) as u8 + 33;
                    encoded.push(c as char);
                }

                encoded
            }

            // Process input 4 bytes at a time
            for chunk in data.chunks(4) {
                buffer = 0;
                for &byte in chunk {
                    buffer = (buffer << 8) | (byte as u32);
                }

                let encoded = encode_chunk(buffer, chunk.len());
                result.container.push_str(&encoded);
            }

            Ok(result)
        }

        // fn encode_serialize<T: Serialize>(bytes: &T) -> Result<Self, Error>
        // where
        //     Self: Sized,
        // {
        //     let chunk_size = Self::get_chunk_size();
        //     let encode_table = Self::get_encode_table();

        //     let byte_slice = bincode::serialize(bytes)?;
        //     let mut to_ret = Self::new();
        //     let as_bitvec: BitVec<u8, Msb0> = BitVec::from_vec(byte_slice);
        //     for x in as_bitvec.chunks(chunk_size) {
        //         let mut val: u8 = 0;
        //         for bit in x {
        //             val = (*bit as u8) | val << 1;
        //         }
        //         if x.len() < chunk_size {
        //             val = val << (chunk_size - x.len())
        //         }
        //         let _ = to_ret.append(encode_table[val as usize]);
        //     }
        //     to_ret.pad();
        //     return Ok(to_ret);
        // }
    }
    impl Display for Base85 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})", self.container)
        }
    }

    mod tests {
        use super::*;
        #[test]
        fn test_base85_encode() {
            let test_vector: Vec<(&[u8], &str)> = vec![
                (b"hello world", "BOu!rD]j7BEbo7"),
                (b"Base85 encoding", "6=FqH3&MgiDI[TqBl7P"),
                (b"Base85", "6=FqH3&L"),
                (b"Base851", "6=FqH3&ND"),
                (b"Base8512", "6=FqH3&NEG"),
                (b"3", "1B"),
                (&[0x00, 0x01, 0x41, 0xFF], "!!,Cc"),
                (&[0x00, 0x01, 0x41, 0xFF, 0x00], "!!,Ccz"),
                (
                    &[
                        0x00, 0x01, 0x41, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x32,
                        0x43, 0x21,
                    ],
                    "!!,Ccz!!!We6Ql",
                ),
            ];
            for (input, expected) in test_vector {
                let encoded = Base85::encode(&input).unwrap();
                println!("Expected: {}, Encoded: {}", expected, encoded.container);
                assert_eq!(encoded.container, expected);
            }
        }
    }
}
