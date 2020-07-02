mod symbols;
mod options;
mod loader;
pub mod ui;

use tokio::sync::Mutex;

use std::convert::From;
use std::io;
use std::sync::Arc;

pub use symbols::Symbol;
pub use options::Option;

#[derive(Debug)]
pub enum CliError {
    InitError { msg: String },
}

pub struct App { 
    pub title: String,
    pub symbols: Vec<Symbol>,
    pub options: Vec<Option>
}

pub async fn initialize(app: Arc<Mutex<App>>) -> Result<(), CliError> {
    let symbols = loader::load::<Symbol>()?;
    {
        let mut app = app.lock().await;
        app.symbols = symbols;
    }

    tokio::spawn(async move {
        if let Ok(options) = loader::load::<Option>() {
            let mut app = app.lock().await;
            app.options = options;
        }
    });
    Ok(())
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::InitError { msg: err.to_string() }
    }
}