pub mod base85 {
    use crate::base_mod::EncodeError;
    use bincode::{self, Error};
    use log::{info};
    use std::{fmt::Display};

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

        pub fn encode<T: AsRef<[u8]>>(bytes: &T) -> Result<Self, Error> {
            let data = bytes.as_ref();
            let mut buffer: u32;
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
        pub fn decode(&self) -> Result<Vec<u8>, EncodeError> {
            let mut to_ret: Vec<u8> = vec![];
            let mut chunk_vec: Vec<u8> = vec![];

            fn decode_chunk(chunk_vec: &mut Vec<u8>) -> Vec<u8> {
                let mut decoded_int_vec: Vec<u8>;
                let mut decoded_int = 0_u32;
                let mut pad_length: usize = 0;
                while chunk_vec.len() < 5 {
                    chunk_vec.push(b'u');
                    pad_length += 1;
                }
                for (index, digit) in chunk_vec.iter().enumerate() {
                    decoded_int += (digit - 33) as u32 * 85_u32.pow(4 - index as u32);
                }
                info!("Decoded int: {decoded_int}");
                decoded_int_vec = decoded_int.to_be_bytes().to_vec();
                if pad_length > 0 {
                    decoded_int_vec.truncate(4 - pad_length);
                }
                decoded_int_vec
            }

            for character in self.container.chars() {
                if character as char == 'z' {
                    if !chunk_vec.is_empty() {
                        return Err(EncodeError::DecodingError {});
                    }
                    let mut decoded_int_vec = (0 as u32).to_be_bytes().to_vec();
                    to_ret.append(&mut decoded_int_vec); //4-byte 0s
                    continue;
                }
                chunk_vec.push(character as u8);
                if chunk_vec.len() == 5 {
                    to_ret.append(&mut decode_chunk(&mut chunk_vec));
                    chunk_vec.clear();
                }
            }

            if chunk_vec.len() > 0 {
                to_ret.append(&mut decode_chunk(&mut chunk_vec));
            }
            return Ok(to_ret);
        }
    }
    impl Display for Base85 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})", self.container)
        }
    }

    #[allow(unused_imports)]
    mod tests {
        use super::Base85;
        #[test]
        fn test_new_base85_is_empty() {
            let b = Base85::new();
            assert_eq!(b.container, "");
        }

        #[test]
        fn test_new_base85_debug_default() {
            let b1 = Base85::new();
            let b2 = Base85::default();
            assert_eq!(b1.container, b2.container);
        }
        #[test]
        fn test_base85_encode_decode() {
            let test_vector: Vec<(&[u8], &str)> = vec![
                (b"hello world", "BOu!rD]j7BEbo7"),
                (b"Base85 encoding", "6=FqH3&MgiDI[TqBl7P"),
                (b"Base85", "6=FqH3&L"),
                (b"Base851", "6=FqH3&ND"),
                (b"Base8512", "6=FqH3&NEG"),
                (b"3", "1B"),
                (b"", ""),
                (&[0xFF as u8; 7], "s8W-!s8W*"),
                (&[0x00, 0x01, 0x41, 0xFF], "!!,Cc"),
                (&[0x00, 0x01, 0x41, 0xFF, 0x00, 0x00, 0x00, 0x00], "!!,Ccz"),
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
                let decoded = encoded.decode().unwrap();
                assert_eq!(input, decoded);
            }
        }
        #[test]
        fn test_base85_decode() {
            let test_vector: Vec<(&str, &[u8])> = vec![
                ("k44rf", &[0xe6, 0xf2, 0x9a, 0x26]),
                ("", &[]),
                ("Y", &[]),
                (
                    "k44rfYXCfd",
                    &[0xe6, 0xf2, 0x9a, 0x26, 0xb0, 0x44, 0x42, 0x61],
                ),
                ("6=FqH3&L", b"Base85"),
                ("3&L", b"85"),
                ("s8W-!s8W*", &[0xFF as u8; 7]),
                (
                    "!!,Ccz!!!We6Ql",
                    &[
                        0x00, 0x01, 0x41, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x32,
                        0x43, 0x21,
                    ],
                ),
                ("!!,Ccz", &[0x00, 0x01, 0x41, 0xFF, 0x00, 0x00, 0x00, 0x00]),
            ];
            for (input, expected) in test_vector {
                let mut encoded = Base85::new();
                encoded.container = input.to_string();
                let decoded = encoded.decode().unwrap();
                println!("Expected: {:?}, Decoded: {:?}", expected, decoded);
                assert_eq!(decoded, expected);
            }
        }
    }
}
