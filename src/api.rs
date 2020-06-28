extern crate dotenv;

use dotenv::dotenv;

pub fn get_api_key() -> Option<String> { 
    dotenv().ok();
    let result = dotenv::var("TRADIER_ACCESS_TOKEN");
    match result {
        Ok(result) => Some(result),
        Err(_) => None
    }
}