use std::io;
use std::io::{Error, ErrorKind};

use serde_json::{Map, Value};

use crate::data_parser::DataParser;
use crate::utilities::{detail_key_to_coords, raw_data_str_to_json};

pub struct DetailDataParser {}

#[derive(Debug)]
pub struct DetailData {
    pub timestamp: i64,
    pub label: String,
    pub author: String,
    pub data: Vec<PixelDetail>,
}

#[derive(Debug)]
pub struct PixelDetail {
    pub id: String,
    pub last_modified_timestamp: i64,
    pub user_id: String,
    pub user_name: String,
    pub coords: (u16, u16),
}

impl DataParser for DetailDataParser {
    type Data = DetailData;

    fn from_line(mut line: String) -> Result<Self::Data, Error> {
        line.retain(|c| !c.is_whitespace());

        let split_line: Vec<&str> = line.split(',').collect();

        let data_value = raw_data_str_to_json(&split_line)?;
        let raw_data = DetailData {
            timestamp: split_line[0]
                .parse::<i64>()
                .map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?,
            label: split_line[1].to_string(),
            author: split_line[2].to_string(),
            data: json_to_details(data_value)?,
        };

        Ok(raw_data)
    }
}

fn json_to_details(json: Value) -> Result<Vec<PixelDetail>, Error> {
    let mut details = Vec::new();

    for (key, json_value) in json["data"]
        .as_object()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not find data object in json"))?
    {
        let coords = detail_key_to_coords(key)?;
        let id = to_id(json_value)?.to_string();
        let last_modified_timestamp = to_timestamp(json_value)? as i64;
        let user_info = to_user_info(json_value)?;
        let user_id = to_user_id(user_info)?.to_string();
        let user_name = to_user_name(user_info)?.to_string();

        let pixel_detail = PixelDetail {
            id,
            last_modified_timestamp,
            user_id,
            user_name,
            coords,
        };

        details.push(pixel_detail);
    }

    Ok(details)
}

fn to_user_name(user_info: &Map<String, Value>) -> Result<&str, Error> {
    user_info
        .get("username")
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not find username"))?
        .as_str()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Not valid string"))
}

fn to_user_id(user_info: &Map<String, Value>) -> Result<&str, Error> {
    user_info
        .get("userID")
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not find userID"))?
        .as_str()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Not valid string"))
}

fn to_user_info(json_value: &Value) -> Result<&Map<String, Value>, Error> {
    json_value["data"][0]["data"]["userInfo"]
        .as_object()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not user info"))
}

fn to_timestamp(json_value: &Value) -> Result<f64, Error> {
    json_value["data"][0]["data"]["lastModifiedTimestamp"]
        .as_f64()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not find last modified timestamp"))
}

fn to_id(json_value: &Value) -> Result<&str, io::Error> {
    json_value["data"][0]["id"]
        .as_str()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not find id"))
}
