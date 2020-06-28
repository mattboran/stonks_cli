use dotenv::dotenv;
use reqwest::{Client, Url};
use reqwest::header::*;
use serde_json::Result as JsonResult;
use serde::{Serialize, Deserialize};

type Result<T> = std::result::Result<T, ApiError>;

const BASE_URL: &str = "https://sandbox.tradier.com/v1/";

#[derive(Debug, Clone)]
enum ApiError { 
    SetUpError,
    NetworkError { code: i32, msg: String }
}

trait Requestable {
    fn url(&self) -> Result<Url>;
}

#[derive(Debug, Clone)]
enum ApiEndpoint { 
    Market { symbols: Vec<String> }
}

impl Requestable for ApiEndpoint { 
    fn url(&self) -> Result<Url> { 
        match self {
            ApiEndpoint::Market { symbols } => {
                let symbols = symbols.join(",");
                Url::parse_with_params(BASE_URL, &[("symbols", symbols)])
                    .map_err(|_| ApiError::SetUpError)
            }
        }
    }
}

fn get_tradier_api_key() -> Result<String> { 
    dotenv().ok();
    dotenv::var("TRADIER_ACCESS_TOKEN")
        .map_err(|_| ApiError::SetUpError)
}

fn get_client() -> Result<Client> {
    let mut headers = HeaderMap::new();
    let auth = format!("Bearer {key}", key = get_tradier_api_key()?);
    headers.append(AUTHORIZATION, auth.parse().unwrap());
    headers.append(CONTENT_TYPE, "appplication/json".parse().unwrap());
    Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|_| ApiError::SetUpError)
}

#[derive(Serialize, Deserialize, Debug)]
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

