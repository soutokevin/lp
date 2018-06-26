#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate exif;
extern crate wasm_bindgen;

use exif::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn read_metadata(vec: &[u8]) -> Vec<f64> {
    let reader = exif::Reader::new(&mut std::io::BufReader::new(vec)).unwrap();

    let degrees_lati;
    let min_lati;
    let sec_lati;

    let degrees_long;
    let min_long;
    let sec_long;

    match reader
        .get_field(exif::Tag::GPSLatitude, false)
        .unwrap()
        .value
    {
        Value::Rational(ref lati_cord) => {
            degrees_lati = lati_cord[0].to_f64();
            min_lati = lati_cord[1].to_f64();
            sec_lati = lati_cord[2].to_f64();
        }
        _ => panic!(),
    }

    match reader
        .get_field(exif::Tag::GPSLongitude, false)
        .unwrap()
        .value
    {
        Value::Rational(ref long_cord) => {
            degrees_long = long_cord[0].to_f64();
            min_long = long_cord[1].to_f64();
            sec_long = long_cord[2].to_f64();
        }
        _ => panic!(),
    }

    let mut latitude = degrees_lati + (min_lati / 60.0) + (sec_lati / 3600.0);
    let mut longitude = degrees_long + (min_long / 60.0) + (sec_long / 3600.0);

    match reader
        .get_field(exif::Tag::GPSLatitudeRef, false)
        .unwrap()
        .value
    {
        Value::Ascii(ref referencia) => {
            if referencia[0][0] == b'S' {
                latitude *= -1.0;
            }
        }
        _ => panic!(),
    }

    match reader
        .get_field(exif::Tag::GPSLongitudeRef, false)
        .unwrap()
        .value
    {
        Value::Ascii(ref referencia) => {
            if referencia[0][0] == b'W' {
                longitude *= -1.0;
            }
        }
        _ => panic!(),
    }

    println!("latitude {:?}", latitude);
    println!("Longitude {:?}", longitude);

    vec![latitude, longitude]
}
