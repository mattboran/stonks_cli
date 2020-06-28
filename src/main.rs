mod cli;
use cli::Cli;

mod api;
use api::get_api_key;

fn main() {
    let args = Cli::from_args();
    println!("Symbol: {}", args.symbol);
    match get_api_key() {
        Some(s) => println!("Tradier Key: {}", s),
        None => println!("Can't get API key")
    }
}