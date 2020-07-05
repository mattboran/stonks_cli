use std::io;
use std::str::FromStr;

/// Symbol represents the symbol for a single NASDAQ security
#[derive(Debug, Clone)]
pub struct Symbol { 
    pub symbol: String,
    pub security_name: String,
    pub market_category: MarketCategory,
    pub test_issue: bool,
    pub financial_status: FinancialStatus,
    pub round_lot_size: u16,
    pub etf: bool,
    pub next_shares: bool
}

impl FromStr for Symbol { 
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split("|").collect();
        let round_lot_size: u16;
        if let Ok(result) = components[5].parse::<u16>() {
            round_lot_size = result;
        } else { 
            round_lot_size = 100;
        }
        let symbol = Symbol { 
            symbol: components[0].parse().unwrap(),
            security_name: components[1].parse().unwrap(),
            market_category: components[2].parse().unwrap(),
            test_issue: components[3] == "Y",
            financial_status: components[4].parse().unwrap(),
            round_lot_size,
            etf: components[6] == "Y",
            next_shares: components[7] == "Y"
        };
        Ok(symbol)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MarketCategory { 
    GlobalSelectMarketSM,
    GlobalMarketSM,
    CapitalMarket,
    Unknown
}

impl FromStr for MarketCategory { 
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Q" => Ok(Self::GlobalSelectMarketSM),
            "G" => Ok(Self::GlobalMarketSM),
            "S" => Ok(Self::CapitalMarket),
            _ => Ok(Self::Unknown)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FinancialStatus {
    Deficient, 
    Delinquent,
    Bankrupt,
    Normal,
    DeficientAndBankrupt,
    DeficientAndDelinquent,
    DelinquentAndBankrupt,
    DeficientDelinquentAndBankrupt,
    Unknown
}

impl FromStr for FinancialStatus { 
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D" => Ok(Self::Deficient),
            "E" => Ok(Self::Delinquent),
            "Q" => Ok(Self::Bankrupt),
            "N" => Ok(Self::Normal),
            "G" => Ok(Self::DeficientAndBankrupt),
            "H" => Ok(Self::DeficientAndDelinquent),
            "J" => Ok(Self::DelinquentAndBankrupt),
            "K" => Ok(Self::DeficientDelinquentAndBankrupt),
            _ => Ok(Self::Unknown)
            
        }
    }
}