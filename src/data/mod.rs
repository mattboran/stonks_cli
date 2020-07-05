mod quote;
mod symbols;
mod options;
pub mod series;
pub mod watchlist;


pub type Quotes = quote::QuotesDataModel;
pub type Quote = quote::Quote;
pub type Symbol = symbols::Symbol;
pub type Option = options::Option;
pub type TimeSeries = series::TimeSeriesDataModel;