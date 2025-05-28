pub mod base32 {
    use bincode::{self, Error};
    use bitvec::prelude::*;
    use log;
    use phf::phf_map;
    use serde::Serialize;
    use std::fmt::Display;
    use crate::base_mod::{Decodable, Encodable, MyError};

    macro_rules! phf_zip_map {
        ([$($k:expr),*], [$($v:expr),*]) => {
            phf::phf_map! {
                $(
                    $k => $v,
                )*
            }
        };
    }
    const DECODE_TABLE: phf::Map<char, u8> = phf_zip_map!(
        [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '2', '3', '4', '5', '6', '7'
            ],
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31
            ]
    );

    const CHUNK_SIZE: usize = 5;
    const BYTE_SIZE: usize = 8;

    const ENCODE_TABLE: [char; 32] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '2', '3', '4', '5', '6', '7'];

    #[derive(Debug, Default)]
    pub struct Base32 {
        container: String,
    }


    impl Base32 {
      
    }


    impl Encodable for Base32 {
        fn new() -> Self where Self: Sized {
            return Self {
                container: (String::new()),
            };        
        }
    
        fn append(&mut self, to_add: char) -> Result<(), MyError> where Self: Sized {
            assert!(ENCODE_TABLE.contains(&to_add));
            self.container.push(to_add);
            return Ok(());
        }
    
        fn pad(&mut self) -> () where Self: Sized {
            if self.container.len() == 0 {
                return;
            }
            match self.container.len() % 8 {
                7 => self.container.push_str("="),
                5 => self.container.push_str("==="),
                4 => self.container.push_str("===="),
                2 => self.container.push_str("======"),
                _ => {}
            }
        }
    
        fn get_chunk_size() -> usize where Self: Sized {
            return CHUNK_SIZE
        }
    
        fn get_encode_table() -> &'static [char] where Self: Sized {
            return &ENCODE_TABLE;
        }
    }

            impl Decodable for Base32 {
        fn get_container(&self) -> &String {
            return &self.container;
        }
        fn get_decode_table() -> &'static phf::Map<char, u8> {
            return &DECODE_TABLE;
        }
        fn get_chunk_size() -> usize {
            return CHUNK_SIZE;
        }
        fn get_byte_size() -> usize {
            return BYTE_SIZE;
        }
    }


    impl Display for Base32 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})", self.container)
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, PartialEq, Debug, Deserialize)]
        struct TestStruct {
            a: i32,
            b: String,
        }
        #[test]
        fn test_printout() {
            let x = b"a213f444vfvdfvdvdscvdf34r023";
            let y = [0xfffffff, 0xfffffff, 4, 5243243, 65];
            let hede_str = TestStruct {
                a: 0xfffffff,
                b: "ZZZZZZ".to_string(),
            };
            println!("encoded x {:}", Base32::encode(&x).unwrap());
            println!("hede {:}", Base32::encode_serialize(&hede_str).unwrap());
            println!(
                "decode x {:?}",
                Base32::encode(&x).unwrap().decode().unwrap().as_slice()
            );
        }

        #[test]
        fn test_base32_encode_decode_string() {
            let test_vector: Vec<(&[u8], &str)> = vec![
                (b"rust", "OJ2XG5A="),
                (b"hello world", "NBSWY3DPEB3W64TMMQ======"),
                (b"hello world", "NBSWY3DPEB3W64TMMQ======"),
                (b"hello worldrst", "NBSWY3DPEB3W64TMMRZHG5A="),
                (b"423refdscfcXsdqhello world", "GQZDG4TFMZSHGY3GMNMHGZDRNBSWY3DPEB3W64TMMQ======"),
            ];
            for (input, expected) in test_vector {
                let encoded = Base32::encode(&input).unwrap();
                assert_eq!(encoded.container, expected);
                let decoded = encoded.decode().unwrap();
                assert_eq!(decoded.as_slice(), input);
            }
        }

        #[test]
        fn test_base32_encode_decode_struct() {
            let input = TestStruct {
                a: 42,
                b: "test".to_string(),
            };
            let encoded = Base32::encode_serialize(&input).unwrap();
            let decoded: Vec<u8> = encoded.decode().unwrap();
            let deserialized: TestStruct = bincode::deserialize(&decoded).unwrap();
            assert_eq!(deserialized, input);
        }

        #[test]
        fn test_base32_padding() {
            let input = b"p";
            let encoded = Base32::encode(&input).unwrap();
            assert!(encoded.container.ends_with("=="));
        }

        #[test]
        fn test_base32_invalid_decode() {
            let mut invalid_base32 = Base32::new();
            invalid_base32.container = "invalid!".to_string();
            let result = invalid_base32.decode();
            assert!(result.is_err());
        }

        #[test]
        fn test_base32_empty_input() {
            let input: &[u8] = b"";
            let encoded = Base32::encode(&input).unwrap();
            let decoded = encoded.decode().unwrap();
            assert!(decoded.is_empty());
        }
    }


}