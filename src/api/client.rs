use std::fmt;
use std::error::Error;

use dotenv::dotenv;
use reqwest::{header::*, Client};
use chrono::{DateTime, FixedOffset};
use crate::data::{Quotes, TimeSeries};
use crate::api::endpoint::{ApiEndpoint, Requestable};

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Clone)]
pub enum ApiError { 
    DeserializationError,
    NetworkError { code: u16, msg: String },
    ParseError,
    SetUpError,
    UnknownError
}


impl Error for ApiError { }

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

pub async fn get_stock_quotes(symbols: Vec<String>) -> Result<Quotes> {
    let client = get_client()?;
    let url = ApiEndpoint::Quotes { symbols }.url()?;

    match client.get(url).send().await {
        Ok(res) => {
            let body = res.bytes().await
                .map_err(|_| ApiError::DeserializationError)?;
            let v = body.to_vec();
            let s = String::from_utf8(v).map_err(|_| ApiError::UnknownError)?;
            serde_json::from_str(&s).map_err(|_| ApiError::DeserializationError)
        },
        Err(err) => Err(parse_network_error(err))
    }
}

pub async fn get_time_series_data(symbol: String, start_date: DateTime<FixedOffset>, end_date: DateTime<FixedOffset>, interval: u8) -> Result<TimeSeries> {
    let client = get_client()?;
    let endpoint = ApiEndpoint::TimeSeries {symbol, start_date, end_date, interval};
    let url = endpoint.url()?;

    match client.get(url).send().await {
        Ok(res) => {
            if res.status() == 400 {
                Err(ApiError::NetworkError{ code: 400, msg: "".to_string()})
            } else {
                let body = res.bytes().await
                    .map_err(|_| ApiError::DeserializationError)?;
                let v = body.to_vec();
                let s = String::from_utf8(v).map_err(|_| ApiError::UnknownError)?;
                serde_json::from_str(&s).map_err(|_| ApiError::DeserializationError)
            }
        },
        Err(err) => Err(parse_network_error(err))
    }
}

fn get_tradier_api_key() -> Result<String> { 
    dotenv().ok();
    dotenv::var("TRADIER_API_KEY")
        .map_err(|_| ApiError::SetUpError)
}

fn get_client() -> Result<Client> {
    let mut headers = HeaderMap::new();
    let auth = format!("Bearer {key}", key = get_tradier_api_key()?);
    headers.append(AUTHORIZATION, auth.parse().unwrap());
    headers.append(ACCEPT, "application/json".parse().unwrap());
    Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|_| ApiError::SetUpError)
}

fn parse_network_error(err: reqwest::Error) -> ApiError { 
    if let Some(status) = err.status() {
        let code = status.as_u16();
        let msg = status.as_str().parse().unwrap();
        return ApiError::NetworkError { code, msg }
    }
    return ApiError::UnknownError
}