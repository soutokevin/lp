#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate exif;
extern crate wasm_bindgen;

use exif::{Reader, Tag, Value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn read_metadata(vec: &[u8]) -> Vec<f64> {
    match metadata(vec) {
        Some((a, b)) => vec![a, b],
        None => vec![],
    }
}

fn get_cord(reader: &Reader, tag: Tag) -> Option<f64> {
    match reader.get_field(tag, false)?.value {
        Value::Rational(ref cord) => {
            let degrees = cord[0].to_f64();
            let min = cord[1].to_f64();
            let sec = cord[2].to_f64();
            Some(degrees + (min / 60.0) + (sec / 3600.0))
        }
        _ => None,
    }
}

fn get_ref(reader: &Reader, tag: Tag) -> Option<u8> {
    match reader.get_field(tag, false)?.value {
        Value::Ascii(ref chars) => Some(chars[0][0]),
        _ => None,
    }
}

fn metadata(vec: &[u8]) -> Option<(f64, f64)> {
    let reader = Reader::new(&mut std::io::BufReader::new(vec)).unwrap();
    let mut latitude = get_cord(&reader, Tag::GPSLatitude)?;
    let mut longitude = get_cord(&reader, Tag::GPSLongitude)?;

    if get_ref(&reader, Tag::GPSLatitudeRef)? == b'S' {
        latitude *= -1.0;
    }

    if get_ref(&reader, Tag::GPSLongitudeRef)? == b'W' {
        longitude *= -1.0;
    }

    Some((latitude, longitude))
}
