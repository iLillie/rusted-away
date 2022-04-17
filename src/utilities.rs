use serde_json::Value;
use std::cmp::Ordering;
use std::io;
use std::io::{Error, ErrorKind};
use std::ops::{Range, RangeFull};
use unescape::unescape;

const CANVAS_SIZE: u16 = 1000;

pub fn to_local_canvas_coords(mut pixel_coords: (u16, u16)) -> (u16, u16) {
    pixel_coords.0 = to_local_canvas_coord(pixel_coords.0);
    pixel_coords.1 = to_local_canvas_coord(pixel_coords.1);

    pixel_coords
}

fn to_local_canvas_coord(mut pixel_coord: u16) -> u16 {
    pixel_coord = match CANVAS_SIZE.cmp(&pixel_coord) {
        Ordering::Greater | Ordering::Equal => pixel_coord - CANVAS_SIZE,
        Ordering::Less => pixel_coord,
    };

    pixel_coord
}

pub fn is_within_canvas(
    pixel_coords: (u16, u16),
    x_range: Range<u16>,
    y_range: Range<u16>,
) -> bool {
    is_within(pixel_coords.0, x_range) && is_within(pixel_coords.1, y_range)
}

fn is_within(value: u16, range: Range<u16>) -> bool {
    value >= range.start && value <= range.end
}

pub fn raw_data_str_to_json(split_line: &[&str]) -> Result<Value, io::Error> {
    let mut data = split_line[3..].join(",");
    data.pop();
    data.remove(0);

    let unescaped_data =
        unescape(&data).ok_or_else(|| Error::new(ErrorKind::Other, "Could not unescape string"))?;
    serde_json::from_str(&unescaped_data)
        .map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))
}

pub fn detail_key_to_coords(detail_key: &str) -> Result<(u16, u16), io::Error> {
    let index = detail_key
        .find("x")
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not find index"))?;
    let x_coord = detail_key[1..index]
        .parse::<u16>()
        .map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?;
    let y_coord = detail_key[index + 1..]
        .parse::<u16>()
        .map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?;
    Ok((x_coord, y_coord))
}
