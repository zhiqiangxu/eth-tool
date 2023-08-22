use std::time::Duration;

use clap::Parser;
use consensus_types::Config;
use eth2::{BeaconNodeHttpClient, SensitiveUrl, Timeouts};
use std::str::FromStr;
use tokio;

#[derive(Parser, Debug)]
pub enum RPC {
    Genesis(Genesis),
    Spec(Spec),
    Head(Head),
    FC(ForkChoice),
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
        println!("{:?}", result.data);
    }
}

#[derive(Parser, Debug)]
pub struct Spec {
    #[arg(long)]
    url: String,
}

impl Spec {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );
        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime
            .block_on(client.get_config_spec::<Config>())
            .unwrap();
        println!("{:?}", result.data);
    }
}

#[derive(Parser, Debug)]
pub struct Head {
    #[arg(long)]
    url: String,
}

impl Head {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );
        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime.block_on(client.get_debug_beacon_heads()).unwrap();
        println!("{:?}", result.data);
    }
}

#[derive(Parser, Debug)]
pub struct ForkChoice {
    #[arg(long)]
    url: String,
}

impl ForkChoice {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );
        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime.block_on(client.get_debug_fork_choice()).unwrap();
        println!("{:?}", result);
    }
}

impl RPC {
    pub fn run(&self) {
        match &self {
            RPC::Genesis(inner) => inner.run(),
            RPC::Spec(inner) => inner.run(),
            RPC::Head(inner) => inner.run(),
            RPC::FC(inner) => inner.run(),
        }
    }
}
