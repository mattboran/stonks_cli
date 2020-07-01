use std::io::{self, Write};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use chrono::{Date, FixedOffset, TimeZone};

use ftp::{FtpStream};

const SYMBOLS_DIRECTORY: &str = "SymbolDirectory";
const SYMBOLS_FILENAME: &str = "nasdaqlisted.txt";
/// Symbol represents the symbol for a single NASDAQ security
pub struct Symbol { 
    symbol: String,
    security_name: String,
    market_category: MarketCategory,
    test_issue: bool,
    financial_status: FinancialStatus,
    round_lot_size: u16,
    etf: bool,
    next_shares: bool
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

pub enum MarketCategory { 
    GlobalSelectMarketSM,
    GlobalMarketSM,
    CapitalMarket
}

impl FromStr for MarketCategory { 
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Q" => Ok(Self::GlobalSelectMarketSM),
            "G" => Ok(Self::GlobalMarketSM),
            "S" => Ok(Self::CapitalMarket),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Error parsing market category."))
        }
    }
}

pub enum FinancialStatus {
    Deficient, 
    Delinquent,
    Bankrupt,
    Normal,
    DeficientAndBankrupt,
    DeficientAndDelinquent,
    DelinquentAndBankrupt,
    DeficientDelinquentAndBankrupt
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
            _ => Err(io::Error::new(io::ErrorKind::Other, "Error parsing financial status.")),
            
        }
    }
}

pub struct SymbolLoadingResult { 
    pub symbols: Vec<Symbol>,
    pub file_creation_date: Date<FixedOffset>,
}

pub fn read_symbols_file() -> Result<SymbolLoadingResult, io::Error> {
    let contents = fs::read_to_string(symbols_file_path())?;
    let lines: Vec<&str> = contents.split("\n").collect();
    // Remove the first line (header) and last line (empty newline)
    let mut lines = lines[1..lines.len() - 1].to_vec();
    let last_line = lines.pop().unwrap();
    let file_creation_date = get_file_creation_date(last_line);
    let symbols = lines
                .into_iter()
                .map(|line| line.parse::<Symbol>())
                .filter_map(Result::ok)
                .collect();
    Ok(SymbolLoadingResult { symbols, file_creation_date })
}

// These functions correspond to fetching and refreshing the symbols file

pub fn create_dir_if_necessary() -> Result<(), io::Error> {
    std::fs::create_dir_all(SYMBOLS_DIRECTORY)
}

pub fn refresh_symbol_file_if_necessary(creation_date: Date<FixedOffset>) -> Result<(), io::Error> { 
    if is_symbol_file_outdated(creation_date) {
        println!("Symbol file is stale - refreshing.");
        fetch_and_write_symbol_file()
    } else {
        Ok(())
    }
}

pub fn fetch_and_write_symbol_file() -> Result<(), io::Error> {
    create_dir_if_necessary()?;
    let bytes = fetch_symbol_file()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
    write_symbol_file(&bytes[..])
}

fn fetch_symbol_file() -> Result<Vec<u8>, ftp::FtpError> {
    // Fetch a fresh copy of the file from the Nasdaqtrader FTP server
    let mut ftp_stream =  FtpStream::connect("206.200.251.105:21")?;
    ftp_stream.login("anonymous", "anonymous")?;
    ftp_stream.cwd(SYMBOLS_DIRECTORY)?;
    let remote_file = ftp_stream.simple_retr(SYMBOLS_FILENAME)?;
    let bytes = remote_file.into_inner();
    ftp_stream.quit()?;
    Ok(bytes)
}      

fn write_symbol_file(bytes: &[u8]) -> Result<(), std::io::Error> {
    // Write to filesystem
    let mut buffer = File::create(symbols_file_path())?;
    buffer.write_all(&bytes)?;
    println!("Saved to {:?}", symbols_file_path());
    Ok(())
}

fn get_file_creation_date(line: &str) -> Date<FixedOffset> { 

    let start_index = "File Creation Time: ".len();
    let end_index = line.find("|");
    let hour = 3600;
    let est = chrono::FixedOffset::west(5 * hour);

    // If we can't parse the file creation date, just assume it's today's date.symbols
    // That means the file won't be refreshed. Is this bad?
    if let None = end_index {
        let local_date = chrono::offset::Local::today().naive_local();
        est.from_local_date(&local_date).unwrap()
    } else {
        let segment = line[start_index..end_index.unwrap()].to_string();
        let month_component = &segment[..2];
        let day_component = &segment[2..4];
        let year_component = &segment[4..8];
        est.ymd(year_component.parse().unwrap(),
                month_component.parse().unwrap(), 
                day_component.parse().unwrap())
    }  
}

fn is_symbol_file_outdated(creation_date: Date<FixedOffset>) -> bool { 
    creation_date != chrono::offset::Local::today()
}

fn symbols_file_path() -> PathBuf { 
    Path::new(SYMBOLS_DIRECTORY).join(SYMBOLS_FILENAME)
}
