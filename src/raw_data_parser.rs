use std::io::{Error, ErrorKind};

use serde_json::Value;
use unescape::unescape;

use crate::data_parser::DataParser;

pub struct RawDataParser {}

pub struct RawData {
    pub timestamp: i64,
    pub label: String,
    pub author: String,
    pub data: Value,
}

impl DataParser for RawDataParser {
    type Data = RawData;

    fn parse_data(mut line: String) -> Result<Self::Data, Error> {
        line.retain(|c| !c.is_whitespace());
        let split_line: Vec<&str> = line.split(',').collect();
        let mut data = split_line[3..].join(",");
        data.pop();
        data.remove(0);

        let unescaped_data =
            unescape(&data).ok_or_else(|| Error::new(ErrorKind::Other, "Could not unescape string"))?;
        let data_value = serde_json::from_str(&unescaped_data)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Error: {}", e)))?;
        let raw_data = RawData {
            timestamp: split_line[0]
                .parse::<i64>()
                .map_err(|e| Error::new(ErrorKind::Other, format!("Error: {}", e)))?,
            label: split_line[1].to_string(),
            author: split_line[2].to_string(),
            data: data_value,
        };
        Ok(raw_data)
    }
}
