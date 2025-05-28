use serde::Serialize;
use rs_encoder::base64::base64::{Base64};
use rs_encoder::base_mod::{Encodable, Decodable};
#[derive(Serialize)]
struct hede {
    a: i32,
    b: String,
}

fn main() {
    let x = b"a213f444vfvdfvdvdscvdf34r023";
    let y = [0xfffffff, 0xfffffff, 4, 5243243, 65];
    let hede_str = hede {
        a: 0xfffffff,
        b: "ZZZZZZ".to_string(),
    };
    println!("encoded x {:}", Base64::encode(&x).unwrap());
    println!("hede {:}",Base64::encode_serialize(&hede_str).unwrap());
    println!(
        "decode x {:?}",
        Base64::encode(&x).unwrap().decode().unwrap().as_slice()
    );
}
