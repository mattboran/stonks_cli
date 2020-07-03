use std::collections::HashMap;
use crate::data::Symbol;

static DEFAULT: &'static [&str] = &[
    "SPY",
    "AMD",
    "TWO"
];

pub struct Watchlist { 
    pub watches: Vec<Symbol>,
}

impl Watchlist { 
    fn new(s: Vec<Symbol>) -> Self { 
        // TODO: try_load_custom_watchlist
        let watches: Vec<Symbol> = s
            .into_iter()
            .filter(|s| DEFAULT.into_iter().any(|x| *x == s.symbol))
            .collect();
        Watchlist { watches }
    }
}

