use std::str::FromStr;

use clap::Parser;

use hex;

use bls::{
    blst_implementations::{PublicKey, PublicKeyBytes, SecretKey},
    Keypair,
};
#[derive(Parser, Debug)]
pub struct Keygen {}

impl Keygen {
    pub fn run(&self) {
        let keypair = Keypair::random();
        let sk = SecretKey::deserialize(keypair.sk.serialize().as_bytes()).unwrap();
        assert_eq!(sk.public_key().as_hex_string(), keypair.pk.as_hex_string());
        assert_eq!(
            PublicKey::from_str(keypair.pk.as_hex_string().as_str())
                .unwrap()
                .as_hex_string(),
            keypair.pk.as_hex_string()
        );
        assert_eq!(
            PublicKey::from_str(keypair.pk.as_hex_string().as_str())
                .unwrap()
                .compress(),
            PublicKeyBytes::from_str(keypair.pk.as_hex_string().as_str()).unwrap()
        );

        println!(
            "pk:\t{}\nsk:\t{}",
            keypair.pk.as_hex_string(),
            hex::encode(keypair.sk.serialize().as_bytes())
        );
    }
}
