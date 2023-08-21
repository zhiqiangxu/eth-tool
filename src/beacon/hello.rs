use clap::Parser;

#[derive(Parser, Debug)]
pub struct Hello {
    #[arg(long)]
    name: String,
}

impl Hello {
    pub fn run(&self) {
        println!("hello world: {}", self.name);
    }
}
