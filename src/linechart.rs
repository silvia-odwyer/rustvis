extern crate image;
extern crate imageproc;
use image::{DynamicImage, Rgba, RgbaImage, GenericImage};
use imageproc::drawing::*;
use crate::{Rgb};
use crate::text::draw_text;
use crate::drawing::*;

/// Draw a linechart, with a specified title and data.
///
/// #### Arguments
/// * `img` - Image to draw the linechart onto.
/// * `chart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn draw_linechart(mut img: &mut DynamicImage, chart: &Chart) {
    draw_labels(&mut img, chart);
    let axis_len = chart.width as f32 * 0.8;
    let y_origin = 20.0 + axis_len;

    let x_inc = axis_len / chart.data.len() as f32;

    let mut start_x = 20.0;
    let line_pixel = image::Rgba([255, 167, 90, 255]);
    let max_item = chart.data.iter().max().unwrap();

    let mut start_y = y_origin;
    
    for item in &chart.data {
        let div: f32 = *max_item as f32 / *item as f32;

        let y_dist = y_origin - (axis_len / div);
        draw_line_segment_mut(img, (start_x as f32, start_y as f32), (start_x + x_inc, y_dist), line_pixel);
        start_x += x_inc;
        start_y = y_dist;
    }
}