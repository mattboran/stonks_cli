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
    }).await.expect("Symbol file should be present.");
    
    let symbols = symbols_result.expect("Unable to parse symbols file.");
    println!("Loaded {} symbols.", symbols.symbols.len());

    let bg_fetch = task::spawn(async move {
        symbols::refresh_symbol_file_if_necessary(&symbols)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))
    });
    return bg_fetch.await?
}