mod api;
mod cli;
mod data;

use cli::arg_parser::Cli;
use cli::symbols;

use tokio::task;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Cli::from_args();
    let action = api::client::get_stock_quote(args.symbols);

    let symbols_result = task::spawn_blocking(move || {
        symbols::load_symbols_from_file()
    }).await?;
    
    match symbols_result {
        Ok(symbols) => { println!("Loaded {} symbols.", symbols.len()); Ok(()) }, 
        Err(err) => Err(err)
    }
}