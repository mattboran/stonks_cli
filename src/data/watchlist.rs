use crate::data::Symbol;
// use crate::cli::Listable;

const DEFAULT: [&str; 10] = [
    "SPY",
    "TSLA",
    "DIS",
    "AMD",
    "NVDA",
    "AAPL",
    "MSFT",
    "FB",
    "GOOG",
    "AMZN",
];

pub fn get_watch_list(s: &[Symbol]) -> Vec<Symbol> { 
    s.to_vec()
        .into_iter()
        .filter(|s| DEFAULT.iter().any(|x| *x == s.symbol.to_uppercase()))
        .collect()
} 