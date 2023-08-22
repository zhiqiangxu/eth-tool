use clap::Parser;

#[derive(Parser, Debug)]
pub struct Gen {
    #[arg(long)]
    name: String,
    #[arg(long)]
    blskey: String,
    #[arg(long)]
    amount: u32,
}

impl Gen {
    pub fn run(&self) {
        println!("hello world: {}", self.name);
    }
}
