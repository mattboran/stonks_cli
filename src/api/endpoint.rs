use chrono::{Datelike, DateTime, Timelike, FixedOffset};
use reqwest::Url;

use crate::api::client::{Result, ApiError};

const BASE_URL: &str = "https://sandbox.tradier.com/v1";

#[derive(Debug, Clone)]
pub enum ApiEndpoint { 
    Quotes { symbols: Vec<String> },
    TimeSeries { symbol: String, start_date: DateTime<FixedOffset>, end_date: DateTime<FixedOffset>, interval: u8 }
}
    
pub trait Requestable {
    fn url(&self) -> Result<Url>;
}

impl Requestable for ApiEndpoint { 
    fn url(&self) -> Result<Url> { 
        match self {
            ApiEndpoint::Quotes { symbols } => {
                let symbols = symbols.join(",");
                let url_str = format!("{}/{}", BASE_URL, "markets/quotes");
                Url::parse_with_params(&url_str, &[("symbols", symbols)])
                    .map_err(|_| ApiError::ParseError)
            },
            ApiEndpoint::TimeSeries { symbol, start_date, end_date, interval } => {
                let url_str = format!("{}/{}", BASE_URL, "markets/timesales");
                let symbol = format!("symbol={}", symbol);
                let interval = format!("interval={}min", interval);
                let start = format!("start={}", date_to_api_string(start_date));
                let end = format!("start={}", date_to_api_string(end_date));
                let url = format!("{}?{}&{}&{}&{}", url_str, symbol, interval, start, end);
                Url::parse(&url).map_err(|_| ApiError::ParseError)
            }
        }
    }
}

fn date_to_api_string(date: &DateTime<FixedOffset>) -> String { 
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}", 
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute()
    )
}