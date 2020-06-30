use std::fs;
use std::path::Path;

pub struct Symbol { 
    symbol: String,
    security_name: String,
    market_category: String,
    test_issue: String,
    financial_status: String,
    round_lot_size: u16,
    etf: bool,
    next_shares: bool
}

pub fn load_symbols_from_file() -> Result<Vec<Symbol>, std::io::Error> { 
    let path = Path::new("./data").join("nasdaqlisted.txt");
    let contents = fs::read_to_string(path)?;
    let lines: Vec<&str> = contents.split("\n").collect();
    if let Some((_, elements)) = lines.split_last() {
        let symbols: Vec<Symbol> = elements
            .into_iter()
            .map(|line| symbol_from_line(*line))
            .collect();
        return Ok(symbols)
    }
    return Ok(vec![])
}

fn symbol_from_line(line: &str) -> Symbol { 
    let components: Vec<&str> = line.split("|").collect();
    let round_lot_size: u16;
    if let Ok(result) = components[5].parse::<u16>() {
        round_lot_size = result;
    } else { 
        round_lot_size = 100;
    }
    Symbol { 
        symbol: components[0].parse().unwrap(),
        security_name: components[1].parse().unwrap(),
        market_category: components[2].parse().unwrap(),
        test_issue: components[3].parse().unwrap(),
        financial_status: components[4].parse().unwrap(),
        round_lot_size,
        etf: components[6] == "Y",
        next_shares: components[7] == "Y"
    }
}

