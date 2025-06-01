pub mod base64 {

    use std::fmt::Display;

    use crate::base_mod::{BitGroupedDecoding, BitGroupedEncoding, MyError};

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
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
        ],
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
        ]
    );

    const CHUNK_SIZE: usize = 6;
    const BYTE_SIZE: usize = 8;

    const ENCODE_TABLE: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    #[derive(Debug, Default)]
    pub struct Base64 {
        container: String,
    }

    impl BitGroupedEncoding for Base64 {
        fn new() -> Self
        where
            Self: Sized,
        {
            return Self {
                container: (String::new()),
            };
        }

        fn append(&mut self, to_add: char) -> Result<(), MyError>
        where
            Self: Sized,
        {
            assert!(ENCODE_TABLE.contains(&to_add));
            self.container.push(to_add);
            return Ok(());
        }

        fn pad(&mut self) -> ()
        where
            Self: Sized,
        {
            if self.container.len() == 0 {
                return;
            }
            match self.container.len() % 4 {
                3 => self.container.push_str("="),
                2 => self.container.push_str("=="),
                _ => {}
            }
        }

        fn get_chunk_size() -> usize
        where
            Self: Sized,
        {
            return CHUNK_SIZE;
        }

        fn get_encode_table() -> &'static [char]
        where
            Self: Sized,
        {
            return &ENCODE_TABLE;
        }
    }

    impl BitGroupedDecoding for Base64 {
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

    impl Display for Base64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})", self.container)
        }
    }

    #[cfg(test)]
    #[allow(unused_imports)]
    mod tests {
        use super::*;

        #[test]
        fn test_printout() {
            let x = b"a213f444vfvdfvdvdscvdf34r023";
            let y = [0xfffffff, 0xfffffff, 4, 5243243, 65];
            println!("encoded x {:}", Base64::encode(&x).unwrap());
            println!(
                "decode x {:?}",
                Base64::encode(&x).unwrap().decode().unwrap().as_slice()
            );
        }

        #[test]
        fn test_base64_encode_decode_string() {
            let test_vector: Vec<(&[u8], &str)> = vec![
                (b"rust", "cnVzdA=="),
                (b"hello world", "aGVsbG8gd29ybGQ="),
                (b"base64", "YmFzZTY0"),
                (b"test", "dGVzdA=="),
                (b"test123", "dGVzdDEyMw=="),
                (b"test1234", "dGVzdDEyMzQ="),
                (b"test12341231", "dGVzdDEyMzQxMjMx"),
                (b"", ""),
                (b"r323r", "cjMyM3I="),
            ];
            for (input, expected) in test_vector {
                let encoded = Base64::encode(&input).unwrap();
                assert_eq!(encoded.container, expected);
                let decoded = encoded.decode().unwrap();
                assert_eq!(decoded.as_slice(), input);
            }
        }

        #[test]
        fn test_base64_padding() {
            let input = b"p";
            let encoded = Base64::encode(&input).unwrap();
            assert!(encoded.container.ends_with("=="));
        }

        #[test]
        fn test_base64_invalid_decode() {
            let mut invalid_base64 = Base64::new();
            invalid_base64.container = "invalid!".to_string();
            let result = invalid_base64.decode();
            assert!(result.is_err());
        }

        #[test]
        fn test_base64_empty_input() {
            let input: &[u8] = b"";
            let encoded = Base64::encode(&input).unwrap();
            let decoded = encoded.decode().unwrap();
            assert!(decoded.is_empty());
        }
    }
}
