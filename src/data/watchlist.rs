use crate::data::Symbol;

static DEFAULT: &'static [&str] = &[
    "SPY",
    "TSLA",
    "DIS",
    "LUV",
    "AMD",
    "NVDA",
    "AAPL",
    "MSFT",
    "FB",
    "GOOG",
    "AMZN",
    "TWO"
];

pub fn get_watch_list(s: &[Symbol]) -> Vec<Symbol> { 
    s.to_vec()
        .into_iter()
        .filter(|s| DEFAULT.into_iter().any(|x| *x == s.symbol))
        .collect()
} 