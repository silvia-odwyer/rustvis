extern crate image;
extern crate imageproc;
use image::{DynamicImage, Rgba};
use imageproc::drawing::*;
use rusttype::{FontCollection, Scale};
use crate::{Rgb};

pub fn draw_text(image: &mut DynamicImage, text: &str, x: u32, y:u32, font: &str, font_size: f32, rgb: &Rgb) {

    // include_bytes! only takes a string literal
    let font_vec = open_font(font);

    let font = FontCollection::from_bytes(font_vec).unwrap().into_font().unwrap();
    let height = font_size;
    let scale = Scale { x: height * 1.0, y: height };

    draw_text_mut(image, Rgba([rgb.r as u8, rgb.g as u8, rgb.b as u8, 255u8]), x + 10, y - 10, scale, &font, text);
}

fn open_font(font: &str) -> std::vec::Vec<u8> {
    // include_bytes! only takes a string literal
    let font_vec = match font {
        "Roboto-Regular" => Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]),
        "Lato-Regular" => Vec::from(include_bytes!("../fonts/Lato-Regular.ttf") as &[u8]),
        "Lato-Bold" => Vec::from(include_bytes!("../fonts/Lato-Bold.ttf") as &[u8]),
        "BebasKai" => Vec::from(include_bytes!("../fonts/BebasKai.ttf") as &[u8]),
        "Oswald-Regular" => Vec::from(include_bytes!("../fonts/Oswald-Regular.ttf") as &[u8]),
        "MrDafoe-Regular" => Vec::from(include_bytes!("../fonts/MrDafoe-Regular.ttf") as &[u8]),
        "Norwester" => Vec::from(include_bytes!("../fonts/Norwester.ttf") as &[u8]),
        "Montserrat-Regular" => Vec::from(include_bytes!("../fonts/Montserrat-Regular.ttf") as &[u8]),
        "Roboto-Light" => Vec::from(include_bytes!("../fonts/Roboto-Light.ttf") as &[u8]),
        "Roboto-Bold" => Vec::from(include_bytes!("../fonts/Roboto-Bold.ttf") as &[u8]),
        "Roboto-Black" => Vec::from(include_bytes!("../fonts/Roboto-Black.ttf") as &[u8]),
        "Roboto-Thin" => Vec::from(include_bytes!("../fonts/Roboto-Thin.ttf") as &[u8]),
        _ => Vec::from(include_bytes!("../fonts/Roboto-Bold.ttf") as &[u8])
    };

    return font_vec;
}
