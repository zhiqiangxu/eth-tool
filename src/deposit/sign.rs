use crate::util::decode_0xhex;
use bls::blst_implementations::{SecretKey, SignatureBytes};
use clap::Parser;
use consensus_types::{ChainSpec, DepositData, DepositMessage, Hash256, SignedRoot};
use hex;
use tree_hash::TreeHash;

#[derive(Parser, Debug)]
pub struct Sign {
    #[arg(long)]
    seckey: String,
    #[arg(long)]
    withdrawal_credentials: String,
    #[arg(long)]
    amount: u64,
}

impl Sign {
    pub fn run(&self) {
        let sk = SecretKey::deserialize(decode_0xhex(self.seckey.as_str()).as_slice()).unwrap();
        let dm = DepositMessage {
            pubkey: sk.public_key().compress(),
            withdrawal_credentials: Hash256::from_slice(
                decode_0xhex(self.withdrawal_credentials.as_str()).as_slice(),
            ),
            amount: self.amount,
        };

        let spec = ChainSpec::default();
        let msg = dm.signing_root(spec.get_deposit_domain());
        println!("sig:{}", SignatureBytes::from(sk.sign(msg)).to_string());

        let d = DepositData {
            pubkey: dm.pubkey,
            withdrawal_credentials: dm.withdrawal_credentials,
            amount: dm.amount,
            signature: SignatureBytes::from(sk.sign(msg)),
        };

        println!(
            "deposit_data_root:{}",
            hex::encode(d.tree_hash_root().as_bytes())
        );
    }
}
