use dotenv::dotenv;
use reqwest::{header::*, Client};
use crate::data::{Quotes};
use crate::api::endpoint::{ApiEndpoint, Requestable};

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Clone)]
pub enum ApiError { 
    DeserializationError,
    NetworkError { code: u16, msg: String },
    SetUpError,
    UnknownError
}

pub async fn get_stock_quote(symbols: Vec<String>) -> Result<Quotes> {
    let client = get_client()?;
    let url = ApiEndpoint::Quotes { symbols }.url()?;

    match client.get(url).send().await {
        Ok(res) => parse_quote(res).await,
        Err(err) => Err(parse_network_error(err))
    }
}

async fn parse_quote(resp: reqwest::Response) -> Result<Quotes> {
    let body = resp.bytes()
        .await
        .map_err(|_| ApiError::DeserializationError)?;
    let v = body.to_vec();
    let s = String::from_utf8(v).map_err(|_| ApiError::UnknownError)?;
    serde_json::from_str(&s).map_err(|_| ApiError::DeserializationError)
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