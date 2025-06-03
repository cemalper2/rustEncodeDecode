use rs_encoder::base64::base64::{Base64};
use rs_encoder::base85::base85::{Base85};
use rs_encoder::base_mod::{BitGroupedEncoding, BitGroupedDecoding};


fn main() {
    let x = b"a213f444vfvdfvdvdscvdf34r023";
    println!("Base64 encoded x {:}", Base64::encode(&x).unwrap());
    println!("Base85 encoded x {:}", Base85::encode(&x).unwrap());
    println!(
        "decode x {:?} {:?}",
        Base64::encode(&x).unwrap().decode().unwrap().as_slice(),
        Base85::encode(&x).unwrap().decode().unwrap().as_slice()
    );
}
