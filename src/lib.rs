extern crate image;
extern crate imageproc;
use image::{DynamicImage, Rgba, RgbaImage, GenericImage, GenericImageView};

/// Rgb color type.
#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Rgb {
    /// Create a new Rgb color.
    pub fn new(&mut self, r: u8, g: u8, b: u8) -> Rgb {
        return Rgb {r: r, g: g, b: b}
    }
}

// STRUCTS 

/// Chart type, containing data, labels, and other metadata about a chart.
#[derive(Debug)]
pub struct Chart {
    pub title: String,
    pub color: Rgb,
    pub data: Vec<u16>,
    pub labels: Vec<String>,
    pub height: u32, 
    pub width: u32
}

impl Chart {

    /// Create a new chart.
    pub fn new(title: String, color: Rgb, data: Vec<u16>, labels: Vec<String>, height: u32, width: u32) -> Chart {
        return Chart { title: title, color: color, data: data, labels: labels, width: width, height: height};
    }
}

pub mod text;
pub mod drawing;
pub mod barchart;