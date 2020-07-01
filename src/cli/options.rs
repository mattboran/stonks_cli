use chrono::{Date, FixedOffset, TimeZone};

pub enum OptionsType { 
    Call,
    Put
}

type Dollars = f32;

// Root Symbol|Options Closing Type|Options Type|Expiration Date|Explicit Strike Price|Underlying Symbol|Underlying Issue Name|Pending
pub struct Option { 
    pub closing_type: String,
    pub options_type: OptionsType,
    pub expiration_date: String,
    pub strike_price: Dollars,
    pub underlying_symbol: String,
    pub underlying_name: String,
    pub pending: bool
}


pub struct OptionLoadingResult { 
    pub symbols: Vec<Option>,
    pub file_creation_date: Date<FixedOffset>,
}