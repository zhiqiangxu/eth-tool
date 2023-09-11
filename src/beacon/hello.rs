use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
pub struct Hello {
    #[arg(long)]
    name: String,
}

impl Hello {
    pub fn run(&self) {
        println!("hello world: {}", self.name);
        println!(
            "current_dir: {}",
            env::current_dir().unwrap().to_str().unwrap()
        );
    }
}
