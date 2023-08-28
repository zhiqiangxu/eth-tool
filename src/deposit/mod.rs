use clap::Parser;

mod data_root;
mod sign;

#[derive(Parser, Debug)]
pub enum Commands {
    Sign(sign::Sign),
    DataRoot(data_root::DataRoot),
}

impl Commands {
    pub fn run(&self) {
        match &self {
            Commands::Sign(inner) => inner.run(),
            Commands::DataRoot(inner) => inner.run(),
        }
    }
}
