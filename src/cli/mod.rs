mod symbols;
mod options;
mod loader;

use std::convert::From;
use std::io;

pub use symbols::Symbol;
pub use options::Option;

#[derive(Debug)]
pub enum CliError {
    InitError { msg: String },
    RefreshSymbolFileError,
    TaskError,
}

pub async fn initialize() -> Result<(), CliError> {
    let symbols = loader::load::<Symbol>()?;
    println!("Loaded {} symbols", symbols.len());

    let option_box = Box::new(|opt: Result<Vec<Option>, CliError>| {
        if let Ok(options) = opt {
            println!("Loaded {} options", options.len());
        }
    });
    loader::load_with_callback::<Option>(option_box).await;
    Ok(())
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::InitError { msg: err.to_string() }
    }
}