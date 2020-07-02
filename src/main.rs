mod api;
mod cli;
mod data;

use std::sync::Arc;
use tokio::sync::Mutex;
use cli::App;

#[tokio::main]
async fn main() -> Result<(), cli::CliError> {
    let app = Arc::new(Mutex::new(App {
        title: format!("StonksCLI"),
        options: vec![],
        symbols: vec![],
    }));
    let result = cli::initialize(Arc::clone(&app)).await;
    let mut i = 0;
    while i < 300 {
        let cloned_app = Arc::clone(&app);
        let app = cloned_app.lock().await;
        println!("{}: {} symbols, {} options", i, app.symbols.len(), app.options.len());
        tokio::time::delay_for(tokio::time::Duration::new(0, 50000000)).await;
        i = i + 1;
    }
    result
}