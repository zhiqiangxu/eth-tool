use std::time::Duration;

use clap::Parser;
use consensus_types::{Config, MainnetEthSpec};
use eth2::{
    types::{BlockId, ValidatorId},
    BeaconNodeHttpClient, SensitiveUrl, Timeouts,
};
use std::str::FromStr;
use tokio;

#[derive(Parser, Debug)]
pub enum RPC {
    Genesis(Genesis),
    Spec(Spec),
    SlotHeader(SlotHeader),
    BlockHeader(BlockHeader),
    ELBlockInfo(ELBlockInfo),
    DebugHead(DebugHead),
    FC(ForkChoice),
    ProtoArray(ProtoArray),
    DepositSnapshot(DepositSnapshot),
    BlockRewards(BlockRewards),
    Validator(Validator),
    Database(Database),
    Staking(Staking),
    StateBalances(StateBalances),
    NodeSyncing(NodeSyncing),
    NodeHealth(NodeHealth),
    Eth1Syncing(Eth1Syncing),
    NodeVersion(NodeVersion),
    PeerCount(PeerCount),
    Blobs(Blobs),
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
pub struct DebugHead {
    #[arg(long)]
    url: String,
}

impl DebugHead {
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
pub struct SlotHeader {
    #[arg(long)]
    url: String,
    #[arg(long)]
    slot: u64,
}

impl SlotHeader {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime
            .block_on(client.get_beacon_headers(Some(self.slot.into()), None))
            .unwrap();
        println!("{:?}", result);
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

#[derive(Parser, Debug)]
pub struct ProtoArray {
    #[arg(long)]
    url: String,
}

impl ProtoArray {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime
            .block_on(client.get_lighthouse_proto_array())
            .unwrap();
        println!("{:?}", result.data);
    }
}

#[derive(Parser, Debug)]
pub struct DepositSnapshot {
    #[arg(long)]
    url: String,
}

impl DepositSnapshot {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime.block_on(client.get_deposit_snapshot()).unwrap();
        println!("{:?}", result);
    }
}

#[derive(Parser, Debug)]
pub struct BlockRewards {
    #[arg(long)]
    url: String,
    #[arg(long)]
    start: u64,
    #[arg(long)]
    end: u64,
}

impl BlockRewards {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime
            .block_on(
                client.get_lighthouse_analysis_block_rewards(self.start.into(), self.end.into()),
            )
            .unwrap();
        println!("{:?}", result);
    }
}

#[derive(Parser, Debug)]
pub struct Validator {
    #[arg(long)]
    url: String,
    #[arg(long)]
    state: String,
    #[arg(long, num_args(1..))]
    ids: Vec<String>,
}

impl Validator {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let state_id = FromStr::from_str(&self.state).unwrap();
        let ids: Vec<_> = self
            .ids
            .iter()
            .map(|id| ValidatorId::from_str(&id).unwrap())
            .collect();
        let result = runtime
            .block_on(client.get_beacon_states_validators(state_id, Some(ids.as_ref()), None))
            .unwrap();
        println!("{:?}", result);
    }
}

#[derive(Parser, Debug)]
pub struct BlockHeader {
    #[arg(long)]
    url: String,
    #[arg(long)]
    block_id: String,
}

impl BlockHeader {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime
            .block_on(
                client.get_beacon_headers_block_id(BlockId::from_str(&self.block_id).unwrap()),
            )
            .unwrap();
        println!("{:?}", result);
    }
}

#[derive(Parser, Debug)]
pub struct ELBlockInfo {
    #[arg(long)]
    url: String,
    #[arg(long)]
    block_id: String,
}

impl ELBlockInfo {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result =
            runtime
                .block_on(client.get_beacon_blocks::<MainnetEthSpec>(
                    BlockId::from_str(&self.block_id).unwrap(),
                ))
                .unwrap()
                .unwrap();
        let payload = result
            .data
            .message()
            .execution_payload()
            .unwrap()
            .execution_payload_capella()
            .unwrap();

        println!(
            "hash:\t{:?}\nnum:\t{:?}\nstate_root:\t{:?}\nparent_hash:\t{:?}\nwithdrawals:\t{:?}",
            payload.block_hash,
            payload.block_number,
            payload.state_root,
            payload.parent_hash,
            payload.withdrawals
        );
    }
}

#[derive(Parser, Debug)]
pub struct Database {
    #[arg(long)]
    url: String,
}

impl Database {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime
            .block_on(client.get_lighthouse_database_info())
            .unwrap();

        println!("{:?}", result);
    }
}

#[derive(Parser, Debug)]
pub struct Staking {
    #[arg(long)]
    url: String,
}

impl Staking {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime.block_on(client.get_lighthouse_staking()).unwrap();
        println!("{:?}", result);
    }
}

#[derive(Parser, Debug)]
pub struct StateBalances {
    #[arg(long)]
    url: String,
    #[arg(long)]
    state: String,
    #[arg(long, num_args(1..))]
    ids: Vec<String>,
}

impl StateBalances {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let state_id = FromStr::from_str(&self.state).unwrap();
        let ids: Vec<_> = self
            .ids
            .iter()
            .map(|id| ValidatorId::from_str(&id).unwrap())
            .collect();
        let result = runtime
            .block_on(client.get_beacon_states_validator_balances(state_id, Some(&ids)))
            .unwrap();
        println!("{:?}", result);
    }
}

#[derive(Parser, Debug)]
pub struct NodeSyncing {
    #[arg(long)]
    url: String,
}

impl NodeSyncing {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");

        let result = runtime.block_on(client.get_node_syncing()).unwrap();
        println!("{:?}", result.data);
    }
}

#[derive(Parser, Debug)]
pub struct NodeHealth {
    #[arg(long)]
    url: String,
}

impl NodeHealth {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");

        let result = runtime.block_on(client.get_node_health()).unwrap();
        println!("{:?}", result);
    }
}

#[derive(Parser, Debug)]
pub struct Eth1Syncing {
    #[arg(long)]
    url: String,
}

impl Eth1Syncing {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");

        let result = runtime
            .block_on(client.get_lighthouse_eth1_syncing())
            .unwrap();
        println!("{:?}", result.data);
    }
}

#[derive(Parser, Debug)]
pub struct NodeVersion {
    #[arg(long)]
    url: String,
}

impl NodeVersion {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");

        let result = runtime.block_on(client.get_node_version()).unwrap();
        println!("{:?}", result.data);
    }
}

#[derive(Parser, Debug)]
pub struct PeerCount {
    #[arg(long)]
    url: String,
}

impl PeerCount {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");

        let result = runtime.block_on(client.get_node_peer_count()).unwrap();
        println!("{:?}", result.data);
    }
}

#[derive(Parser, Debug)]
pub struct Blobs {
    #[arg(long)]
    url: String,
    #[arg(long)]
    slot: String,
    #[arg(long)]
    ids: Option<Vec<u64>>,
}

impl Blobs {
    pub fn run(&self) {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::from_str(self.url.as_str()).unwrap(),
            Timeouts::set_all(Duration::new(1, 0)),
        );

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let result = runtime
            .block_on(client.get_blobs::<MainnetEthSpec>(
                BlockId::from_str(self.slot.as_str()).unwrap(),
                self.ids.as_deref(),
            ))
            .unwrap()
            .unwrap();
        if result.data.len() == 1 {
            print!("0x{}", hex::encode(result.data[0].blob.to_vec()));
        } else {
            for sidecar in result.data {
                println!("0x{}", hex::encode(sidecar.blob.to_vec()));
            }
        }
    }
}

impl RPC {
    pub fn run(&self) {
        match &self {
            RPC::Genesis(inner) => inner.run(),
            RPC::Spec(inner) => inner.run(),
            RPC::DebugHead(inner) => inner.run(),
            RPC::SlotHeader(inner) => inner.run(),
            RPC::BlockHeader(inner) => inner.run(),
            RPC::ELBlockInfo(inner) => inner.run(),
            RPC::FC(inner) => inner.run(),
            RPC::ProtoArray(inner) => inner.run(),
            RPC::DepositSnapshot(inner) => inner.run(),
            RPC::BlockRewards(inner) => inner.run(),
            RPC::Validator(inner) => inner.run(),
            RPC::Database(inner) => inner.run(),
            RPC::Staking(inner) => inner.run(),
            RPC::StateBalances(inner) => inner.run(),
            RPC::NodeSyncing(inner) => inner.run(),
            RPC::NodeHealth(inner) => inner.run(),
            RPC::Eth1Syncing(inner) => inner.run(),
            RPC::NodeVersion(inner) => inner.run(),
            RPC::PeerCount(inner) => inner.run(),
            RPC::Blobs(inner) => inner.run(),
        }
    }
}
