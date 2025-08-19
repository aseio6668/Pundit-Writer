use clap::Parser;

#[derive(Parser)]
#[command(name = "test")]
pub struct TestArgs {
    #[arg(short, long)]
    pub test: bool,
}

fn main() {
    let _args = TestArgs::parse();
    println!("Test successful!");
}