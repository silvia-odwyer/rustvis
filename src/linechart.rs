extern crate image;
extern crate imageproc;
use image::{DynamicImage, Rgba, RgbaImage, GenericImage};
use imageproc::drawing::*;
use crate::{Rgb};
use crate::text::draw_text;
use crate::barchart::{Chart, draw_labels, draw_y_axial_notches};
use crate::drawing::*;
use imageproc::rect::Rect;
use imageproc::pixelops::interpolate;
use crate::new_with_background;

/// Draw a linechart, with a specified title and data.
///
/// #### Arguments
/// * `img` - Image to draw the linechart onto.
/// * `chart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn draw_linechart(mut img: &mut DynamicImage, chart: &Chart) {
    draw_lines(img, chart, false);
}

/// Draw a linechart with points, containing a specified title and data.
///
/// #### Arguments
/// * `img` - Image to draw the linechart onto.
/// * `chart` - Chart struct, which contains all data & metadata about the barchart.
pub fn draw_linechart_points(mut img: &mut DynamicImage, chart: &Chart) {
    draw_lines(img, chart, true);
}

/// Create a linechart and return this as an image.
///
/// #### Arguments
/// * `chart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn create_linechart(chart: &Chart) -> DynamicImage {
    let slate_grey = Rgb { r: 33, g: 33, b: 36};
    let mut img = new_with_background(*chart.width(), *chart.height(), &slate_grey);
    
    draw_lines(&mut img, chart, false);
    return img;
}

/// Create a linechart with points and return this as an image.
///
/// #### Arguments
/// * `chart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn create_linechart_points(chart: &Chart) -> DynamicImage {
    let slate_grey = Rgb { r: 33, g: 33, b: 36};
    let mut img = new_with_background(*chart.width(), *chart.height(), &slate_grey);
    
    draw_lines(&mut img, chart, true);
    return img;
}

// Draw linechart onto the image.
fn draw_lines(mut img: &mut DynamicImage, chart: &Chart, points: bool) {
    draw_labels(&mut img, chart);
    let axis_len_width = *chart.width() as f32 * 0.8;
    let axis_len_height = *chart.height() as f32 * 0.8;
    
    let y_origin = *chart.height() as f32 - (*chart.height() as f32 * 0.1);

    let x_inc = axis_len_width / chart.data().len() as f32;

    let mut start_x = 20.0;
    let line_pixel = image::Rgba([chart.color().r, chart.color().g, chart.color().b, 255]);

    let white = image::Rgba([155, 155, 155, 255]);
    let white_rgb = Rgb {r: 255, g: 255, b:255};

    let chart_data = chart.data();
    let max_item = chart_data.iter().max().unwrap();

    let mut start_y = y_origin;
    let start_y_meta: u32 = chart.height() - ((*chart.height() as f32 * 0.1) as u32);

    let div: f32 = *max_item as f32 / chart_data[0] as f32;
    let end_y: i32 = (y_origin - (axis_len_height / div)) as i32;
    start_y = end_y as f32;

    for i  in 1..chart_data.len() {
        let item = chart_data[i];
        let div: f32 = *max_item as f32 / item as f32;

        let end_x: i32 = (start_x + x_inc) as i32;
        let end_y: i32 = (y_origin - (axis_len_height / div)) as i32;
        
        for i in 0..3 {
            draw_antialiased_line_segment_mut(img, (start_x as i32 + i, start_y as i32), (end_x + i, end_y), line_pixel, interpolate);
        }

        if (points) { 
            draw_filled_circle_mut(img, (end_x, end_y), 4, white);
        }

        // Draw x axial notch for that bar
        draw_solid_rect(img, &white_rgb, 1, 10, start_x as i32, start_y_meta as i32);

        start_x += x_inc;
        start_y = end_y as f32;
    }
    draw_solid_rect(img, &white_rgb, 1, 10, start_x as i32, start_y_meta as i32);

    draw_y_axial_notches(img, chart);
}