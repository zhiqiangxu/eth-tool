use clap::Parser;

use crate::util::remove_hex_prefix;
use bls::blst_implementations::{PublicKeyBytes, SignatureBytes};
use consensus_types::{DepositData, Hash256};
use std::str::FromStr;
use tree_hash::TreeHash;

#[derive(Parser, Debug)]
pub struct DataRoot {
    #[arg(long)]
    pubkey: String,
    #[arg(long)]
    withdrawal_credentials: String,
    #[arg(long)]
    amount: u64,
    #[arg(long)]
    signature: String,
}

impl DataRoot {
    pub fn run(&self) {
        let d = DepositData {
            pubkey: PublicKeyBytes::from_str(self.pubkey.as_str()).unwrap(),
            withdrawal_credentials: Hash256::from_slice(
                hex::decode(remove_hex_prefix(self.withdrawal_credentials.as_str()))
                    .unwrap()
                    .as_slice(),
            ),
            amount: self.amount,
            signature: SignatureBytes::from_str(self.signature.as_str()).unwrap(),
        };

        println!(
            "deposit_data_root:{}",
            hex::encode(d.tree_hash_root().as_bytes())
        );
    }
}
