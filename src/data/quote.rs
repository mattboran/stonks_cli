use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QuotesDataModel { 
    pub quotes: Quotes
}

#[derive(Debug, Deserialize)]
pub struct Quotes {
    pub quote: Vec<Quote>
}

#[derive(Debug, Deserialize)]
pub struct Quote { 
    pub ask: f32,
    pub bid: f32,
    #[serde(rename(deserialize = "asksize"))]
    pub ask_size: u32,
    #[serde(rename(deserialize = "bidsize"))]
    pub bid_size: u32,
    pub symbol: String,
    pub volume: u32, 
    pub week_52_high: f32,
    pub week_52_low: f32,
    pub open: f32,
    pub last: f32,
    pub description: String,
    #[serde(rename(deserialize = "change"))]
    pub change_points: f32,
    pub change_percentage: f32,
}

