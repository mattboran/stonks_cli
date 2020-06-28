mod cli;
use cli::Cli;

mod api;

fn main() {
    let args = Cli::from_args();
    println!("Symbols: {:?}", args.symbols);
}