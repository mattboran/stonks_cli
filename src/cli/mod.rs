mod symbols;
mod options;
use tokio::task;

use std::convert::From;
use std::io;

pub use symbols::{
    Symbol, 
    SymbolLoadingResult,
};

pub use options::{
    Option,
    OptionLoadingResult,
};

pub struct LoadingResult { 
    symbols: SymbolLoadingResult,
    options: OptionLoadingResult
}

#[derive(Debug)]
pub enum CliError {
    InitError { msg: String },
    RefreshSymbolFileError,
}

pub async fn initialize() -> Result<SymbolLoadingResult, CliError> {
    let symbols_result = task::spawn_blocking(move || {
        symbols::read_symbols_file()
            .or_else(|_| retry_loading_symbols())
    })
        .await
        .unwrap()
        .map_err(CliError::from);

    return symbols_result
        .and_then(|result| { 
            println!("Loaded {} symbols.", result.symbols.len());
            let date = result.file_creation_date;
            task::spawn(async move {
                symbols::refresh_symbol_file_if_necessary(date)
                    .map_err(|_| CliError::RefreshSymbolFileError)
            });
            Ok(result) 
        }).map_err(CliError::from);
}

pub fn retry_loading_symbols() -> Result<SymbolLoadingResult, CliError> { 
    symbols::fetch_and_write_symbol_file()?;
    symbols::read_symbols_file().map_err(CliError::from)
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::InitError { msg: err.to_string() }
    }
}