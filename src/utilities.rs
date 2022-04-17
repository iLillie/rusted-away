use std::cmp::Ordering;
use std::io;
use std::ops::{Range, RangeFull};
use unescape::unescape;
use std::io::{Error, ErrorKind};


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

pub fn is_within_canvas(pixel_coords: (u16, u16), x_range: Range<u16>, y_range: Range<u16>) -> bool {
    is_within(pixel_coords.0, x_range) && is_within(pixel_coords.1, y_range)
}

fn is_within(value: u16, range: Range<u16>) -> bool {
    value >= range.start && value <= range.end
}
