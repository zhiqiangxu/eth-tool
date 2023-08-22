use std::time::Duration;

use clap::Parser;
use eth2::{BeaconNodeHttpClient, SensitiveUrl, Timeouts};

use std::str::FromStr;
use tokio;

#[derive(Parser, Debug)]
pub enum RPC {
    Genesis(Genesis),
}

#[derive(Parser, Debug)]
pub struct Genesis {
    #[arg(long)]
    url: String,
}

impl Genesis {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );
        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime.block_on(client.get_beacon_genesis()).unwrap();
        println!("{:?}", result);
    }
}

impl RPC {
    pub fn run(&self) {
        match &self {
            RPC::Genesis(g) => g.run(),
        }
    }
}
