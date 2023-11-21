use base64::{engine::general_purpose, Engine};
use image::{load_from_memory, ImageOutputFormat::Png};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_string: &str) -> String {
    log(&"Grayscaled called".into());

    let base64_to_vector = general_purpose::STANDARD.decode(&encoded_string).unwrap();
    log(&"Decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image Loaded".into());

    img = img.grayscale();
    log(&"Image Grayscaled".into());

    let mut grayscaled_image_buffer = vec![];
    img.write_to(&mut grayscaled_image_buffer, Png).unwrap();
    log(&"Image Written".into());

    let grayscaled_image_base64 = general_purpose::STANDARD.encode(&grayscaled_image_buffer);

    let data_url = format!("data:image/png;base64,{}", grayscaled_image_base64);
    return data_url;
}
