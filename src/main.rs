mod api;
mod cli;
mod data;

#[tokio::main]
async fn main() {
    let args = cli::Cli::from_args();
    let action = api::client::get_stock_quote(args.symbols);

    match action.await {
        Ok(result) => { println!("Result {:?}", result)}
        Err(error) => { println!("Error {:?}", error) }
    }
}