use std::borrow::Borrow;
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind};
use serde_json::Value;
use unescape::unescape;

const HISTORY_JSON_CSV_INDEX:usize = 37;

pub fn read_pixel_history(filePath: &str) {
    let file = File::open(filePath).unwrap();
    let reader = BufReader::new(file);
    let value = reader.lines();
    for line in value {
        match line {
            Ok(line_str) => {
                parse_pixel_history_line(&line_str);
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }


}

fn parse_escaped_json(escaped_json: &str) -> Result<Value, io::Error> {
    let json_str = unescape(&escaped_json[1..escaped_json.len()-1]).unwrap();
    let json: Value = serde_json::from_str(&json_str)?;
    Ok(json)
}

fn parse_pixel_history_line(line: &str)  {
    let json_str = &line[HISTORY_JSON_CSV_INDEX..];
    match parse_escaped_json(json_str) {
        Ok(json) => {
            let pixel_history = json_to_pixel_detail_vec(json);
            for pixel_history in pixel_history {
                println!("{},{},{},{},{}", pixel_history.last_modified, pixel_history.user_id, pixel_history.user_name, pixel_history.x, pixel_history.y);
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn json_to_pixel_detail_vec(json: Value) -> Vec<PixelHistory> {
    let pixel_data = json["data"].as_object();
    let mut pixel_history: Vec<PixelHistory> = vec![];
    match pixel_data {
        Some(pixel_keys) => {
            let default_pixel = PixelHistory {
                last_modified: 0,
                user_id: "undefined".to_string(),
                user_name: "undefined".to_string(),
                x: 0,
                y: 0
            };
            pixel_history = pixel_keys.iter().map(|(key, value)| parse_pixel_history(key, value).unwrap_or(default_pixel.clone())).collect();
        },
        _ => {

        }
    }
    pixel_history
}

fn parse_pixel_history(key: &String, pixel_value: &Value) -> Option<PixelHistory> {
    let position = key_to_pixel_position(key);
    if position.is_err() {
        return None;
    }
    let position = position.unwrap();
    let details_data_obj = pixel_value["data"][0]["data"].as_object();
    if details_data_obj.is_none() {
        return None;
    }
    let detail_data = details_data_obj.unwrap();
    let user_info_wrapped = detail_data["userInfo"].as_object();
    if user_info_wrapped.is_none() {
        return None;
    }
    let user_info = user_info_wrapped.unwrap();
    let user_id = user_info["userID"].to_string();
    let user_name = user_info["username"].to_string();
    let timestamp = detail_data["lastModifiedTimestamp"]
        .as_f64()
        .unwrap()
        .round() as i64;
    let pixel_history = PixelHistory {
        last_modified: timestamp,
        user_id,
        user_name,
        x: position.0,
        y: position.1,
    };
    Some(pixel_history)
}

fn key_to_pixel_position(key: &str) -> Result<(u16, u16), &'static str> {
    let index = key.find("x").ok_or("Failed to find X")?;
    let x = key[1..index].parse::<u16>().unwrap();
    let y =  key[index + 1..].parse::<u16>().unwrap();
    Ok((x, y))
}


#[derive(Debug, Clone)]
struct PixelHistory {
    last_modified: i64,
    user_id: String,
    user_name: String,
    x: u16,
    y: u16
}



