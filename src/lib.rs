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

fn metadata(vec: &[u8]) -> Option<(f64, f64)> {
    let reader = Reader::new(&mut std::io::BufReader::new(vec)).unwrap();

    let mut latitude = match reader.get_field(Tag::GPSLatitude, false)?.value {
        Value::Rational(ref cord) => {
            let degrees = cord[0].to_f64();
            let min = cord[1].to_f64();
            let sec = cord[2].to_f64();
            degrees + (min / 60.0) + (sec / 3600.0)
        }
        _ => panic!(),
    };

    let mut longitude = match reader.get_field(Tag::GPSLongitude, false)?.value {
        Value::Rational(ref cord) => {
            let degrees = cord[0].to_f64();
            let min = cord[1].to_f64();
            let sec = cord[2].to_f64();
            degrees + (min / 60.0) + (sec / 3600.0)
        }
        _ => panic!(),
    };

    match reader.get_field(Tag::GPSLatitudeRef, false)?.value {
        Value::Ascii(ref chars) if chars[0][0] == b'S' => latitude *= -1.0,
        _ => panic!(),
    }

    match reader.get_field(Tag::GPSLongitudeRef, false)?.value {
        Value::Ascii(ref chars) if chars[0][0] == b'W' => longitude *= -1.0,
        _ => panic!(),
    }

    Some((latitude, longitude))
}
