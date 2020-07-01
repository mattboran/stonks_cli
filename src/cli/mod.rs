mod symbols;
use tokio::task;

pub use symbols::{
    Symbol, 
    SymbolLoadingResult,
};

#[derive(Debug)]
pub enum CliError {
    InitError { msg: String },
    RefreshSymbolFileError,
}

pub async fn initialize() -> Result<SymbolLoadingResult, CliError> {
    let symbols_result = task::spawn_blocking(move || {
        symbols::read_symbols_file().or_else(|_| {
            retry_loading_symbols()
        })
    }).await
      .map_err(|_| CliError::InitError { msg: "Error loading symbols file".to_string() })?;

    return symbols_result
        .and_then(|result| { 
            println!("Loaded {} symbols.", result.symbols.len());
            let date = result.file_creation_date;
            task::spawn(async move {
                symbols::refresh_symbol_file_if_necessary(date)
                    .map_err(|_| CliError::RefreshSymbolFileError)
            });
            Ok(result) 
        }).map_err(|err| CliError::InitError { msg: err.to_string() });
}

pub fn retry_loading_symbols() -> Result<SymbolLoadingResult, std::io::Error> { 
    symbols::create_data_dir_if_necessary().and_then(|_| {
        // Swallow error from the next call.
        symbols::fetch_symbol_file();
        symbols::read_symbols_file()
    })
}