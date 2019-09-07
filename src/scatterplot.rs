extern crate image;
extern crate imageproc;
use image::{DynamicImage};
use imageproc::drawing::*;
use crate::barchart::{Chart, draw_labels};
use imageproc::pixelops::interpolate;

pub fn draw_scatterplot(mut img: &mut DynamicImage, chart: &Chart) {
    draw_labels(&mut img, chart);
    let axis_len = *chart.width() as f32 * 0.8;

    let y_origin = 20.0 + axis_len;

    let x_inc = axis_len / chart.data().len() as f32;

    let mut start_x = 20.0;
    let line_pixel = image::Rgba([255, 167, 90, 255]);

    let white = image::Rgba([155, 155, 155, 255]);

    let max_item = chart.data().iter().max().unwrap();

    let mut start_y = y_origin;
    
    for item in chart.data() {
        let div: f32 = *max_item as f32 / *item as f32;

        let end_x: i32 = (start_x + x_inc) as i32;
        let end_y: i32 = (y_origin - (axis_len / div)) as i32;

        draw_filled_circle_mut(img, (end_x, end_y), 4, white);


        start_x += x_inc;
        start_y = end_y as f32;
    }
}