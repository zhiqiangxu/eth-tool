use clap::Parser;
mod hello;

#[derive(Parser, Debug)]
pub enum Commands {
    Hello(hello::Hello),
}
impl Commands {
    pub fn run(&self) {
        match &self {
            Commands::Hello(hello) => hello.run(),
        }
    }
}
