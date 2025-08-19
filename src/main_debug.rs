use clap::Parser;

mod cli;

#[derive(Parser)]
struct SimpleArgs {
    #[arg(short, long)]
    test: bool,
}

fn main() {
    let _args = SimpleArgs::parse();
    println!("Test successful!");
}