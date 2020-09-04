extern crate image;
extern crate imageproc;
use crate::Rgb;
use image::{DynamicImage, Rgba};
use imageproc::drawing::*;
use rusttype::{FontCollection, Scale};

pub fn draw_text(
    image: &mut DynamicImage,
    text: &str,
    x: u32,
    y: u32,
    font: &str,
    font_size: f32,
    rgb: &Rgb,
) {
    // include_bytes! only takes a string literal
    let font_vec = open_font(font);

    let font = FontCollection::from_bytes(font_vec)
        .unwrap()
        .into_font()
        .unwrap();
    let height = font_size;
    let scale = Scale {
        x: height * 1.0,
        y: height,
    };

    draw_text_mut(
        image,
        Rgba([rgb.r as u8, rgb.g as u8, rgb.b as u8, 255u8]),
        x + 10,
        y - 10,
        scale,
        &font,
        text,
    );
}

fn open_font(font: &str) -> std::vec::Vec<u8> {
    // include_bytes! only takes a string literal
    match font {
        "Roboto-Regular" => Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]),
        "Lato-Regular" => Vec::from(include_bytes!("../fonts/Lato-Regular.ttf") as &[u8]),
        _ => Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]),
    }
}
