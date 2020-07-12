use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::util;

// API Representation 

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
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub price: f64, 
    #[serde(with = "date_format")]
    pub time: DateTime<FixedOffset>,
    pub timestamp: u32,
    pub volume: u32, 
    pub vwap: f64
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
        let s = format!("{}-05:00", s);
        
        DateTime::<FixedOffset>::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)
    }
}

// Graph Representation

pub const TIME_MARKERS: &[&str] = &["9:30", "11:00", "1:00", "2:30", "4:00"]; 

impl TimeSeriesDataModel {
    pub fn to_graph_data(&self, width: u16) -> Vec<(f64, f64)> {
        let data = &self.data();
        let num_points = data.len();
        let mut result = Vec::with_capacity(width as usize);
        for i in 0..width {
            let pos = (i as f64) / (width as f64);
            let x = pos * (num_points as f64);
            let idx = x as usize;
            let y = data[idx].vwap;
            result.push((i as f64, y));
        }
        result
    }

    pub fn min_max(&self) -> (f64, f64) {
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for point in self.data() {
            if point.vwap > max { 
                max = point.vwap;
            }
            if point.vwap < min { 
                min = point.vwap;
            }
        }
        (min, max)
    }

    pub fn went_up(&self) -> bool { 
        let data = self.data();
        return data[0 as usize].vwap <= data.last().unwrap().vwap;
    }
}