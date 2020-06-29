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
    pub asksize: i32,
    pub bidsize: i32,
    pub symbol: String,
    pub volume: i32, 
    pub week_52_high: f32,
    pub week_52_low: f32,
    pub open: f32,
    pub last_close: f32,
    pub description: String,
    pub change: f32,
    pub change_percentage: f32,
}

