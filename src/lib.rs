#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn read_file(vec: &[u8]) -> String {
    let mut text = String::new();

    for (i, el) in vec.iter().enumerate() {
        if i % 80 == 0 {
            text.push('\n');
        }

        match el {
            b'a'...b'z'
            | b'A'...b'Z'
            | b'1'...b'9'
            | b'['
            | b']'
            | b'@'
            | b'='
            | b'"'
            | b'-'
            | b'{'
            | b'}'
            | b' ' => text.push(*el as char),
            b'\n' => text.push_str("\\n"),
            _ => text.push('.'),
        }
    }

    text
}
