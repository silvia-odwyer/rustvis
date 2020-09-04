#![allow(dead_code)]

extern crate image;
extern crate imageproc;

use image::{DynamicImage, ImageBuffer};

/// Rgb color type.
#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Create a new Rgb color.
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }
}

pub fn open_image(img_path: &'static str) -> DynamicImage {
    image::open(img_path).unwrap()
}

pub fn save_image(img: DynamicImage, img_path: &str) {
    img.save(img_path).unwrap();
}

pub fn new_with_background(width: u32, height: u32, background_color: &Rgb) -> DynamicImage {
    let pixel = image::Rgba([
        background_color.r,
        background_color.g,
        background_color.b,
        255,
    ]);
    let image_buffer = ImageBuffer::from_pixel(width, height, pixel);
    image::ImageRgba8(image_buffer)
}

pub mod barchart;
pub mod drawing;
pub mod linechart;
pub mod text;
