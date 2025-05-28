pub mod base32;
pub mod base64;
pub mod base85;

pub mod base_mod {
    use bincode::{self, Error};
    use bitvec::{field::BitField, order::Msb0, vec::BitVec};
    use serde::Serialize;
    #[derive(Debug)]
    pub enum MyError {
        OutOfBounds {},
        DecodingError {}, // other error variants...
    }

    /// A trait for types that can be encoded using a custom encoding scheme.
    ///
    /// # Required Methods
    /// - `new() -> Self`: Constructs a new instance of the implementing type.
    /// - `append(&mut self, to_add: char) -> Result<(), MyError>`: Appends a character to the encoded output.
    /// - `pad(&mut self)`: Pads the encoded output as necessary to meet encoding requirements.
    /// - `get_chunk_size() -> usize`: Returns the number of bits per encoding chunk.
    /// - `get_encode_table() -> &'static [char]`: Returns the encoding table used for mapping values to characters.
    ///
    /// # Provided Methods
    /// - `encode<T: AsRef<[u8]>>(bytes: &T) -> Result<Self, Error>`: Encodes a byte slice into the implementing type using the encoding table and chunk size.
    /// - `encode_serialize<T: Serialize>(bytes: &T) -> Result<Self, Error>`: Serializes a value using `bincode` and then encodes the resulting bytes.
    ///
    /// # Errors
    /// Both `append` and the provided methods may return errors if encoding fails or serialization fails.
    ///
    /// # Example
    /// ```rust,ignore
    /// let encoded = MyEncoder::encode(&data)?;
    /// ```
    pub trait Encodable {
        fn new() -> Self
        where
            Self: Sized;

        fn append(&mut self, to_add: char) -> Result<(), MyError>
        where
            Self: Sized;

        fn pad(&mut self) -> ()
        where
            Self: Sized;

        fn get_chunk_size() -> usize
        where
            Self: Sized;

        fn get_encode_table() -> &'static [char]
        where
            Self: Sized;

        fn encode<T: AsRef<[u8]>>(bytes: &T) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let chunk_size = Self::get_chunk_size();
            let encode_table = Self::get_encode_table();
            let mut to_ret = Self::new();
            let as_bitvec: BitVec<u8, Msb0> = BitVec::from_slice(bytes.as_ref());
            for x in as_bitvec.chunks(chunk_size) {
                let mut val: u8 = 0;
                for bit in x {
                    val = (*bit as u8) | val << 1;
                }
                if x.len() < chunk_size {
                    val = val << (chunk_size - x.len())
                }
                let _ = to_ret.append(encode_table[val as usize]);
            }
            to_ret.pad();
            return Ok(to_ret);
        }
        fn encode_serialize<T: Serialize>(bytes: &T) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let chunk_size = Self::get_chunk_size();
            let encode_table = Self::get_encode_table();

            let byte_slice = bincode::serialize(bytes)?;
            let mut to_ret = Self::new();
            let as_bitvec: BitVec<u8, Msb0> = BitVec::from_vec(byte_slice);
            for x in as_bitvec.chunks(chunk_size) {
                let mut val: u8 = 0;
                for bit in x {
                    val = (*bit as u8) | val << 1;
                }
                if x.len() < chunk_size {
                    val = val << (chunk_size - x.len())
                }
                let _ = to_ret.append(encode_table[val as usize]);
            }
            to_ret.pad();
            return Ok(to_ret);
        }
    }
    pub trait Decodable {
        fn get_container(&self) -> &String;
        fn get_decode_table() -> &'static phf::Map<char, u8>;
        fn get_chunk_size() -> usize;
        fn get_byte_size() -> usize;
        fn decode(&self) -> Result<Vec<u8>, MyError> {
            let mut to_ret: Vec<u8> = vec![];
            let mut bits: BitVec<u8, Msb0> = BitVec::new();
            let decode_table = Self::get_decode_table();
            let chunk_size = Self::get_chunk_size();
            let byte_size = Self::get_byte_size();
            for symbol in self.get_container().chars() {
                if symbol == '=' {
                    break;
                }
                match decode_table.get(&symbol) {
                    Some(value) => {
                        for bit_place in (0..=chunk_size - 1).rev() {
                            // println!(
                            //     "bit_place {:} value & (1 <<bit_place) != 0 {:}",
                            //     bit_place,
                            //     value & (1 << bit_place) != 0
                            // );
                            bits.push(value & (1 << bit_place) != 0);
                        }
                    }
                    None => {
                        log::warn!("Tried to decode invalid symbol: {}", symbol);
                        return Err(MyError::DecodingError {});
                    }
                }
            }

            for x in bits.chunks(byte_size) {
                if x.len() < byte_size {
                    break;
                }
                // println!("chunk {:}", x);
                // println!("chunk {:}", x.load::<u8>());
                to_ret.push(x.load());
            }
            return Ok(to_ret);
        }
    }
}
