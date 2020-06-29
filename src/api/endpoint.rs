use crate::api::client::{Result, Error};
use reqwest::Url;

const BASE_URL: &str = "https://sandbox.tradier.com/v1";

#[derive(Debug, Clone)]
pub enum ApiEndpoint { 
    Quotes { symbols: Vec<String> }
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
                    .map_err(|_| Error::SetUpError)
            }
        }
    }
}