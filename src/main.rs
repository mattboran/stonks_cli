mod api;
mod cli;
mod data;

#[tokio::main]
async fn main() -> Result<(), cli::CliError> {
    let result = cli::initialize().await;
    let mut i = 0;
    loop {
        println!("Iteration {}", i);
        tokio::time::delay_for(tokio::time::Duration::new(1, 0)).await;
        i = i + 1;
        if i == 10 { 
            break;
        }
    }
    match result {
        Ok(_) => {
            println!("Initialize complete");
            Ok(())
        },
        Err(err) => { 
            println!("Got cli error: {:?}", err);
            Err(err)
        },
    }
}