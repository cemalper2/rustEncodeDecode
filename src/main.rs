use rs_encoder::base64::base64::{Base64};
use rs_encoder::base_mod::{BitGroupedEncoding, BitGroupedDecoding};


fn main() {
    let x = b"a213f444vfvdfvdvdscvdf34r023";
    println!("encoded x {:}", Base64::encode(&x).unwrap());
    println!(
        "decode x {:?}",
        Base64::encode(&x).unwrap().decode().unwrap().as_slice()
    );
}
