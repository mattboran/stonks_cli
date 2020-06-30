use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{Write, Error, ErrorKind};
use chrono::{Date, FixedOffset, TimeZone};

use ftp::FtpStream;

/// Symbol represents the symbol for a single NASDAQ security
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

impl Symbol { 

    fn new(line: &str) -> Symbol { 
    
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
}

pub struct SymbolLoadingResult { 
    pub symbols: Vec<Symbol>,
    file_creation_date: Date<FixedOffset>,
}

/// This method is blocking.
pub fn load_symbols_from_file() -> Result<SymbolLoadingResult, std::io::Error> { 

    let contents = fs::read_to_string(symbols_file_path())?;
    let mut lines: Vec<&str> = contents.split("\n").collect();
    let num_lines = lines.len();
    lines.truncate(num_lines - 1);

    if let Some((last_line, elements)) = lines.split_last() {
        let file_creation_date = get_file_creation_date(last_line);
        let symbols: Vec<Symbol> = elements
            .into_iter()
            .map(|line| Symbol::new(*line))
            .collect();
        return Ok(SymbolLoadingResult { symbols, file_creation_date })
    }
    return Err(Error::new(ErrorKind::Other, "Error parsing symbol file"))
}

pub fn refresh_symbol_file_if_necessary(result: &SymbolLoadingResult) -> Result<(), Box<dyn std::error::Error>> { 
    if !is_symbol_file_outdated(result.file_creation_date) {
        println!("No need to refresh symbol file");
        return Ok(())
    } 
    println!("Refreshing file");
    let mut ftp_stream =  FtpStream::connect("206.200.251.105:21")?;
    ftp_stream.login("anonymous", "anonymous")?;
    ftp_stream.cwd("/SymbolDirectory")?;
    let remote_file = ftp_stream.simple_retr("nasdaqlisted.txt")?;
    let bytes = &remote_file.into_inner()[..];
    ftp_stream.quit()?;
            
    println!("Got FTP file from ftp.nasdaqtrader.com/nasdaqlisted.txt");
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
    Path::new("./data").join("nasdaqlisted.txt")
}
