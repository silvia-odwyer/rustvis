extern crate image;
extern crate imageproc;
use image::{DynamicImage, Rgba, RgbaImage, GenericImage};
use imageproc::drawing::*;
use crate::{Rgb};
use crate::text::draw_text;
use crate::drawing::*;

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


/// Draw a vertical barchart, with a specified title and data.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
pub fn draw_vertical_barchart(img: &mut DynamicImage, barchart: &Chart) {

    draw_vertical_bars(img, barchart, "barchart");
}




/// Draw a vertical barchart, with a specified title and data.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `histogram` - Chart struct, which contains all data & meta-data about the barchart.
pub fn draw_vertical_histogram(img: &mut DynamicImage, histogram: &Chart) {

    draw_vertical_bars(img, histogram, "histogram");
}


/// Draw a vertical barchart, where the bars are filled with a gradient.
///
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
/// * `preset` - Preset name for the gradient. Can be: "pinkblue", "pastel_pink", "pastel_mauve", "lemongrass"
pub fn draw_vertical_gradient_barchart(img: &mut DynamicImage, barchart: &Chart, preset: &str) {

    let mut start_x: u32 = 20;
    let start_y: u32 = barchart.height - 40;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_height: u32 = barchart.height - 2 * (barchart.height / 10);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_width: u32 = ((barchart.height / num_bars) as f32 * 0.8) as u32;

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for item in &barchart.data {
        let div =  max_item / item;
        let bar_height = max_bar_height / div as u32;
        draw_preset_rect_gradient(img, bar_width as u32, bar_height as u32, start_x, start_y - bar_height, preset);

        start_x += bar_width + 30;    
    }    

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

// Draw vertical bars, either as a histogram or bar chart. 
// This is a private function, but may become public in the future.
fn draw_vertical_bars(img: &mut DynamicImage, barchart: &Chart, chart_type: &str) {

    let bar_gap = match chart_type {
        "barchart" => 30,
        "histogram" => 0,
        _ => 30
    };

    let mut start_x: u32 = 20;
    let start_y: u32 = barchart.height - 40;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_height: u32 = barchart.height - 2 * (barchart.height / 10);
    let num_bars: u32 = barchart.data.len() as u32;
    println!("num_bars {}", barchart.data.len());
    let bar_width: u32 = ((barchart.height / num_bars) as f32 * 0.8) as u32;
    
    for item in &barchart.data {
        let mut bar_height = 0;

        if *item > 0 as u16 {
            let div =  max_item / item;
            bar_height = max_bar_height / div as u32;

        }

        draw_solid_rect(img, &barchart.color, bar_width as u32, bar_height as u32, start_x as i32, (start_y - bar_height) as i32);
        start_x += bar_width + bar_gap;    
    }

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}



// Draw labels onto the axes of a chart, typically for bar or line charts.
fn draw_labels(img: &mut DynamicImage, chart: &Chart) {
    draw_axes(img, chart);
    let axis_len = chart.width as f32 * 0.8;
    let x_inc = axis_len / chart.data.len() as f32;
    let yellow = Rgb { r: 150, g: 150, b: 30};
    let mut start_x = 20.0;

    let start_y = 20.0 + (chart.width as f32 * 0.8);
    
    for label in &chart.labels {
        draw_text(img, label, start_x as u32, start_y as u32, "Roboto-Regular", 30.0, &yellow);

        start_x += x_inc;
    }    
}

// Draw x and y-axes to the image, mainly for bar charts and line charts.
fn draw_axes(img: &mut DynamicImage, chart: &Chart) {
    let line_pixel = image::Rgba([255, 167, 90, 255]);

    let axis_len = chart.width as f32 * 0.8;

    // Origin point
    let start_x = 20.0;
    let start_y = 20.0 + axis_len;

    // End point on y-axis 
    let end_y_yaxis: f32 = start_y - axis_len;

    // End point on x-axis
    let end_x_xaxis: f32 = start_x + axis_len;
    
    // Draw y-axis
    draw_line_segment_mut(img, (start_x as f32, start_y as f32), (start_x, end_y_yaxis), line_pixel);

    // Draw x-axis 
    draw_line_segment_mut(img, (start_x as f32, start_y as f32), (end_x_xaxis, start_y), line_pixel);
}
