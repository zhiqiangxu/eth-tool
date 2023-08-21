use clap::Parser;

mod beacon;

#[derive(Parser)]
enum Cli {
    #[clap(subcommand)]
    Beacon(beacon::Commands),
}

fn main() {
    let matches = Cli::parse();

    match matches {
        Cli::Beacon(beacon) => beacon.run(),
    }
}
