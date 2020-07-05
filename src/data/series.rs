use std::io;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::util;

#[derive(Debug, Deserialize)]
pub struct TimeSeriesDataModel {
    series: TimeSeriesContainer
}

impl TimeSeriesDataModel {
    pub fn data(&self) -> &Vec<TimeSeriesPoint> {
        &self.series.data
    }
}

#[derive(Debug, Deserialize)]
struct  TimeSeriesContainer {
    data: Vec<TimeSeriesPoint>
}

#[derive(Debug, Deserialize)]
pub struct TimeSeriesPoint {
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub price: f32, 
    #[serde(with = "date_format")]
    pub time: DateTime<FixedOffset>,
    pub timestamp: u32,
    pub volume: u32, 
    pub vwap: f32
}

mod date_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::<FixedOffset>::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)
    }
}
