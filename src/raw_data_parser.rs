use std::io;
use std::io::{Error, ErrorKind};

use serde_json::Value;
use unescape::unescape;

use crate::data_parser::DataParser;
use crate::utilities::raw_data_str_to_json;

pub struct RawDataParser {}

pub struct RawData {
    pub timestamp: i64,
    pub label: String,
    pub author: String,
    pub data: Value,
}

impl DataParser for RawDataParser {
    type Data = RawData;

    fn from_line(mut line: String) -> Result<Self::Data, Error> {
        line.retain(|c| !c.is_whitespace());

        let split_line: Vec<&str> = line.split(',').collect();

        let data_value = raw_data_str_to_json(&split_line)?;

        let raw_data = RawData {
            timestamp: split_line[0]
                .parse::<i64>()
                .map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?,
            label: split_line[1].to_string(),
            author: split_line[2].to_string(),
            data: data_value,
        };
        Ok(raw_data)
    }
}