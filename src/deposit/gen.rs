use bls::blst_implementations::{SecretKey, SignatureBytes};
use clap::Parser;
use consensus_types::{ChainSpec, DepositMessage, Hash256, SignedRoot};
use hex;

#[derive(Parser, Debug)]
pub struct Gen {
    #[arg(long)]
    seckey: String,
    #[arg(long)]
    withdrawal_credentials: String,
    #[arg(long)]
    amount: u64,
}

fn remove_hex_prefix(hex_string: &str) -> &str {
    if hex_string.starts_with("0x") {
        &hex_string[2..]
    } else {
        hex_string
    }
}

fn decode_0xhex(hex_string: &str) -> Vec<u8> {
    return hex::decode(remove_hex_prefix(hex_string)).unwrap();
}

impl Gen {
    pub fn run(&self) {
        let sk = SecretKey::deserialize(decode_0xhex(self.seckey.as_str()).as_slice()).unwrap();
        let d = DepositMessage {
            pubkey: sk.public_key().compress(),
            withdrawal_credentials: Hash256::from_slice(
                hex::decode(remove_hex_prefix(self.withdrawal_credentials.as_str()))
                    .unwrap()
                    .as_slice(),
            ),
            amount: self.amount,
        };

        let spec = ChainSpec::default();
        let msg = d.signing_root(spec.get_deposit_domain());
        println!("{}", SignatureBytes::from(sk.sign(msg)).to_string());
    }
}
