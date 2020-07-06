use std::collections::HashMap;

use crate::data;

pub type QuoteCache = HashMap<String, data::Quote>;
pub type GraphCache = HashMap<String, data::TimeSeries>;