use std::env;
use structopt::StructOpt;
// mod cli;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short)]
    symbol: String,
}
fn main() {
    let args = Cli::from_args();
    println!("{:?}", args);
}