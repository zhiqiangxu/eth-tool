use clap::Parser;

mod beacon;
mod deposit;
mod util;

#[derive(Parser)]
enum Cli {
    #[clap(subcommand)]
    Beacon(beacon::Commands),
    #[clap(subcommand)]
    Deposit(deposit::Commands),
}

fn main() {
    let matches = Cli::parse();

    match matches {
        Cli::Beacon(inner) => inner.run(),
        Cli::Deposit(inner) => inner.run(),
    }
}
