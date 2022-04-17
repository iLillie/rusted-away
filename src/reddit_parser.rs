use std::io;
use std::io::{Error, ErrorKind};
use unescape::unescape;
use serde_json::{Map, Value};

use crate::data_parser::DataParser;
use crate::utilities::{detail_key_to_coords, raw_data_str_to_json};

pub struct RedditDataParser {}

#[derive(Debug)]
pub struct RedditData {
    pub timestamp: String,
    pub hashed_user: String,
    pub hex_code: String,
    pub coords: String
}


impl DataParser for RedditDataParser {
    type Data = RedditData;

    fn from_line(mut line: String) -> Result<Self::Data, Error> {
        line.retain(|c| !c.is_whitespace());

        if line.len() < 50 {
             return Err(Error::new( ErrorKind::Other,"CSV Line hit"))
        }

        let split_line: Vec<&str> = line.split(',').collect();
        let reddit_data = RedditData {
            timestamp: split_line[0].to_string(),
            hashed_user: split_line[1].to_string(),
            hex_code: split_line[2].to_string(),
            coords: split_line[3..].join(",")
        };

        Ok(reddit_data)
    }
}
