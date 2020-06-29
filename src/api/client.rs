use dotenv::dotenv;
use reqwest::{header::*, Client};
use crate::data::quote;
use crate::api::endpoint::{ApiEndpoint, Requestable};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error { 
    DeserializationError,
    NetworkError { code: u16, msg: String },
    SetUpError,
    UnknownError
}

pub async fn get_stock_quote(symbols: Vec<String>) -> Result<quote::QuotesDataModel> {
    let client = get_client()?;
    let url = ApiEndpoint::Quotes {symbols: symbols}.url()?;

    match client.get(url).send().await {
        Ok(res) => parse_quote(res).await,
        Err(err) => Err(parse_network_error(err))
    }
}

async fn parse_quote(resp: reqwest::Response) -> Result<quote::QuotesDataModel> {
    let body = resp.bytes()
        .await
        .map_err(|_| Error::DeserializationError)?;
    let v = body.to_vec();
    let s = String::from_utf8_lossy(&v);
    println!("Parsed response: {}", s);
    Err(Error::UnknownError)
}

fn get_tradier_api_key() -> Result<String> { 
    dotenv().ok();
    dotenv::var("TRADIER_API_KEY")
        .map_err(|_| Error::SetUpError)
}

fn get_client() -> Result<Client> {
    let mut headers = HeaderMap::new();
    let auth = format!("Bearer {key}", key = get_tradier_api_key()?);
    headers.append(AUTHORIZATION, auth.parse().unwrap());
    headers.append(ACCEPT, "application/json".parse().unwrap());
    Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|_| Error::SetUpError)
}

fn parse_network_error(err: reqwest::Error) -> Error { 
    if let Some(status) = err.status() {
        let code = status.as_u16();
        let msg = status.as_str().parse().unwrap();
        return Error::NetworkError { code, msg }
    }
    return Error::UnknownError
}
