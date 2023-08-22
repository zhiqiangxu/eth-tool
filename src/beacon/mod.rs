use clap::Parser;
mod hello;
mod rpc;

#[derive(Parser, Debug)]
pub enum Commands {
    Hello(hello::Hello),
    #[clap(subcommand)]
    RPC(rpc::RPC),
}
impl Commands {
    pub fn run(&self) {
        match &self {
            Commands::Hello(hello) => hello.run(),
            Commands::RPC(rpc) => rpc.run(),
        }
    }
}
