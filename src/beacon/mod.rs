use clap::Parser;
mod hello;
mod keygen;
mod keyinfo;
mod rpc;

#[derive(Parser, Debug)]
pub enum Commands {
    Hello(hello::Hello),
    #[clap(subcommand)]
    RPC(rpc::RPC),
    Keygen(keygen::Keygen),
    Keyinfo(keyinfo::Keyinfo),
}
impl Commands {
    pub fn run(&self) {
        match &self {
            Commands::Hello(inner) => inner.run(),
            Commands::RPC(inner) => inner.run(),
            Commands::Keygen(inner) => inner.run(),
            Commands::Keyinfo(inner) => inner.run(),
        }
    }
}
