mod api;
mod cli;
mod data;

use tokio::task;
use std::io::{Error, ErrorKind};

#[tokio::main]
async fn main() -> Result<(), cli::CliError> {
    let result = cli::initialize().await;
    match result {
        Ok(result) => {
            println!("Got successful {} symbols", result.symbols.len());
            Ok(())
        },
        Err(err) => { 
            println!("Got cli error: {:?}", err);
            Err(err)
        },
    }
}