use crate::data::Symbol;
// use crate::cli::Listable;

const DEFAULT: [&str; 12] = [
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
        .filter(|s| DEFAULT.iter().any(|x| *x == s.symbol))
        .collect()
} 