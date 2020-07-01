use std::io;
use std::str::FromStr;
use chrono::{Date, FixedOffset, TimeZone};

pub enum OptionType { 
    Call,
    Put
}

type Dollars = f32;

// Root Symbol|Options Closing Type|Options Type|Expiration Date|Explicit Strike Price|Underlying Symbol|Underlying Issue Name|Pending
pub struct Option { 
    pub closing_type: String,
    pub options_type: OptionType,
    pub expiration_date: String,
    pub strike_price: Dollars,
    pub underlying_symbol: String,
    pub underlying_name: String,
    pub pending: bool
}

impl FromStr for Option { 
    //Root Symbol|Options Closing Type|Options Type|Expiration Date|Explicit Strike Price|Underlying Symbol|Underlying Issue Name|Pending
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split("|").collect();
        let option = Option { 
            closing_type: components[1].parse().unwrap(),
            options_type: components[2].parse().unwrap(),
            expiration_date: components[3].parse().unwrap(),
            strike_price: components[4].parse().unwrap(),
            underlying_symbol: components[5].parse().unwrap(),
            underlying_name: components[6].parse().unwrap(),
            pending: components[7] == "Y"
        };
        Ok(option)
    }
}

impl FromStr for OptionType { 
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Self::Call),
            "P" => Ok(Self::Put),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Error parsing option type."))
        }
    }
}