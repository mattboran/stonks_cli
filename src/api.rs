extern crate dotenv;

use dotenv::dotenv;

const ENDPOINT: &str = "https://sandbox.tradier.com/v1/";

pub fn get_api_key() -> Option<String> { 
    dotenv().ok();
    let result = dotenv::var("TRADIER_ACCESS_TOKEN");
    match result {
        Ok(result) => Some(result),
        Err(_) => None
    }
}

// pub fn get_stock_quote() -> Quote { 

// }

pub struct Quote { 
    pub ask: f32,
    pub bid: f32,
    pub ask_size: i32,
    pub bid_size: i32,
    pub symbol: String,
    pub volume: i32, 
    pub week_52_high: f32,
    pub week_52_low: f32,
    pub last_open: f32,
    pub last_close: f32,
    pub description: String,
    pub change_val: f32,
    pub change_percentage: f32,
}

