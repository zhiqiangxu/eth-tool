use clap::Parser;

use bls::blst_implementations::SecretKey;
#[derive(Parser, Debug)]
pub struct Keyinfo {
    #[arg(long)]
    sk: String,
}

impl Keyinfo {
    pub fn run(&self) {
        let sk = SecretKey::deserialize(hex::decode(self.sk.as_bytes()).unwrap().as_ref()).unwrap();
        println!("pk:\t{}", sk.public_key().as_hex_string(),);
    }
}
