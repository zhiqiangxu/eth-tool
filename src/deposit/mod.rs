use clap::Parser;

mod gen;

#[derive(Parser, Debug)]
pub enum Commands {
    Gen(gen::Gen),
}

impl Commands {
    pub fn run(&self) {
        match &self {
            Commands::Gen(gen) => gen.run(),
        }
    }
}
