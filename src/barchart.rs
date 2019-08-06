extern crate image;
extern crate imageproc;
use image::{DynamicImage, Rgba, RgbaImage, GenericImage};
use imageproc::drawing::*;
use crate::{Rgb};
use crate::text::draw_text;
use crate::drawing::*;
use imageproc::pixelops::interpolate;
use imageproc::rect::Rect;
use crate::new_with_background;

/// Chart type, containing data, labels, and other metadata about a chart.
#[derive(Debug)]
pub struct Chart {
    title: String,
    color: Rgb,
    data: Vec<u16>,
    labels: Vec<String>,
    hasGrid: bool,
    width: u32,
    height: u32
}

impl Chart {

    /// Create a new chart.
    pub fn new(title: String, color: Rgb, data: Vec<u16>, labels: Vec<String>, hasGrid: bool, height: u32, width: u32) -> Chart {
        return Chart { title: title, color: color, data: data, labels: labels, hasGrid: hasGrid, width: width, height: height};
    }

    /// Get the chart's title.
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Get the chart's line color.
    pub fn color(&self) -> &Rgb {
        &self.color
    }

    /// Get the chart's data.
    pub fn data(&self) -> &Vec<u16> {
        &self.data
    }

    /// Get the chart's labels.
    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    /// Get the chart's height.
    pub fn height(&self) -> &u32 {
        &self.height
    }

    /// Get the chart's width.
    pub fn width(&self) -> &u32 {
        &self.width
    }
    
    /// Set the chart's title.
    pub fn setTitle(&mut self, title: String) {
        self.title = title;
    }

    /// Set the chart's line color.
    pub fn setColor(&mut self, color: Rgb) {
        self.color = color;
    }

    /// Set the chart's data as a Vec of u16s.
    pub fn setData(&mut self, data: Vec<u16>) {
        self.data = data;
    }

    /// Set the chart's labels.
    pub fn setLabels(&mut self, labels: Vec<String>) {
        self.labels = labels;
    }

    /// Set the chart's height.
    pub fn setHeight(&mut self, height: u32) {
        self.height = height;
    }

    /// Set the chart's width.
    pub fn setWidth(&mut self, width: u32) {
        self.width = width;
    }
}

/// Draw a horizontal barchart, with a specified title and data.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
pub fn draw_horizontal_barchart(img: &mut DynamicImage, barchart: &Chart) {
    draw_horizontal_bars(img, &barchart, "barchart");
}

/// Draw a vertical barchart, with a specified title and data.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
pub fn draw_vertical_barchart(img: &mut DynamicImage, barchart: &Chart) {

    draw_vertical_bars(img, barchart, "barchart");
}

/// Draw a histogram with a specified title, and data.
/// 
/// /// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
pub fn draw_horizontal_histogram(img: &mut DynamicImage, barchart: &Chart) {
    draw_horizontal_bars(img, &barchart, "histogram");

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
    draw_axes(img, barchart);
    let mut start_x: u32 = 20;
    let start_y: u32 = barchart.height - 40;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_height: f32 = barchart.height as f32 - 2.0 * (barchart.height as f32 / 10.0);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_width: u32 = ((barchart.width / num_bars) as f32 * 0.8) as u32;
    let space_between_bars: u32 = (bar_width as f32 * 0.1) as u32;

    let yellow = Rgb{ r: 255, g: 226, b: 98};
    let white = Rgb { r: 255, g: 255, b: 255};

    for item in &barchart.data {
        let div: f32 =  *max_item as f32 / *item as f32;
        let bar_height: f32 = max_bar_height / div as f32;
        draw_preset_rect_gradient(img, bar_width as u32, bar_height as u32, start_x, start_y - bar_height as u32, preset);

        start_x += bar_width + space_between_bars;    
    }    

    draw_text(img, &barchart.title, barchart.height, start_y as u32, "Lato-Regular", 50.0, &white);
}

// Draw vertical bars, either as a histogram or bar chart. 
// This is a private function, but may become public in the future.
fn draw_vertical_bars(img: &mut DynamicImage, barchart: &Chart, chart_type: &str) {

    let mut start_x: u32 = 20;
    let start_y: u32 = barchart.height - 40;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_height: f32 = barchart.height as f32 - 2.0 * (barchart.height as f32 / 10.0);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_width: u32 = ((barchart.width / num_bars) as f32 * 0.8) as u32;


    let bar_gap = match chart_type {
        "barchart" => (bar_width as f32 * 0.1) as u32,
        "histogram" => 0,
        _ => (bar_width as f32 * 0.1) as u32,
    };

    let border_thickness: i32 = match chart_type {
        "barchart" => 0, 
        "histogram" => 2,
        _ => 0
    };

    for item in &barchart.data {
        let mut bar_height: f32 = 0.0;

        if *item > 0 as u16 {
            let div: f32 =  *max_item as f32 / *item as f32;
            bar_height = max_bar_height / div;

        }

        draw_solid_rect(img, &barchart.color, bar_width as u32, bar_height as u32, start_x as i32, (start_y as f32 - bar_height) as i32);
        start_x += bar_width + bar_gap;    
    }

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

/// Draw a vertical barchart, where the bars are filled with a gradient.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `barchart` - Barchart struct, which contains all data & meta-data about the barchart.
/// * `preset` - Preset name for the gradient. Can be: "pinkblue", "pastel_pink", "pastel_mauve", "lemongrass"
pub fn draw_horizontal_gradient_barchart(img: &mut DynamicImage, barchart: &Chart, preset: &str) {

    let start_x: u32 = 20;
    let mut start_y: u32 = 20;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_width: u32 = barchart.width - 2 * (barchart.width / 10);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_height: u32 = ((barchart.height / num_bars) as f32 * 0.8) as u32;
    let space_between_bars: u32 = (bar_height as f32 * 0.1) as u32;

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for item in &barchart.data {
        let div: f32 =  *max_item as f32 / *item as f32;
        let bar_width: f32 = max_bar_width as f32 / div;
        draw_preset_rect_gradient(img, bar_width as u32, bar_height as u32, start_x, start_y , preset);
        start_y += bar_height + space_between_bars;
    }    

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

// Draw a horizontal chart, either as a histogram or as a barchart,
// with horizontal bars.
fn draw_horizontal_bars(img: &mut DynamicImage, barchart: &Chart, chart_type: &str) {

    let start_x: i32 = 20;
    let mut start_y: i32 = 20;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_width: f32 = barchart.width as f32 - 2.0 * (barchart.width as f32 / 10.0);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_height: f32 = ((barchart.height / num_bars) as f32 * 0.8);

    let space_between_bars = match chart_type {
        "barchart" => (bar_height as f32 * 0.1) as u32,
        "histogram" => 0,
        _ => 30
    };

    let yellow = Rgb{ r: 255, g: 226, b: 98};
    let line_pixel = Rgba([240, 240, 255, 255]);

    let border_thickness: i32 = match chart_type {
        "barchart" => 0, 
        "histogram" => 2,
        _ => 0
    };

    for item in &barchart.data {
        let div: f32 =  *max_item as f32 / *item as f32;
        let bar_width: f32 = max_bar_width / div;
        
        draw_solid_rect(img, &barchart.color, bar_width as u32, bar_height as u32, start_x, start_y + border_thickness);
        start_y += (bar_height as u32 + space_between_bars) as i32;    

        if chart_type == "histogram" {
            draw_filled_rect_mut(img, Rect::at(start_x, start_y).of_size(bar_width as u32, border_thickness as u32), line_pixel);
        }
    }    
    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

/// Draw a vertical barchart, where each bar is denoted by an image.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `bar_img` - Image the bars should contain.
/// * `barchart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn draw_vertical_image_barchart(img: &mut DynamicImage, bar_img: &DynamicImage, barchart: &Chart) {
    draw_axes(img, barchart);
    let mut start_x: u32 = 20;
    let start_y = barchart.height - 40;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_height: f32 = barchart.height as f32 - 2.0 * (barchart.height as f32 / 10.0);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_width: u32 = ((barchart.width / num_bars) as f32 * 0.8) as u32;
    let space_between_bars: u32 = (bar_width as f32 * 0.1) as u32;

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for item in &barchart.data {
        let div: f32 =  *max_item as f32 / *item as f32;
        let bar_height: f32 = max_bar_height / div;

        let sampling_filter = image::FilterType::Nearest;
  
        let resized_img = image::ImageRgba8(image::imageops::resize(bar_img, bar_width as u32, bar_height as u32, sampling_filter));

        image::imageops::overlay(img, &resized_img, start_x, start_y - bar_height as u32);        

        start_x += bar_width + space_between_bars;    
    }    

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

/// Draw a vertical barchart onto a source image, with a specified title and data.
/// 
/// ### Arguments
/// * `img` - Image to draw the barchart onto.
/// * `bar_img` - Image the bars should contain.
/// * `barchart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn draw_horizontal_image_barchart(img: &mut DynamicImage, bar_img: &DynamicImage, barchart: &Chart) {

    let start_x: u32 = 20;
    let mut start_y: u32 = 20;

    let max_item = barchart.data.iter().max().unwrap();
    let max_bar_width: f32 = barchart.width as f32 - 2.0 * (barchart.width as f32 / 10.0);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_height: f32 = ((barchart.height / num_bars) as f32 * 0.8);
    let space_between_bars: u32 = (bar_height as f32 * 0.1) as u32;

    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for item in &barchart.data {
        let div: f32 =  *max_item as f32 / *item as f32;
        let bar_width: f32 = max_bar_width / div;

        draw_image_as_bar(img, bar_img, bar_width as u32, bar_height as u32, start_x, start_y);
        
        start_y += bar_height as u32 + space_between_bars;
    }    
    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}


/// Create a new vertical barchart image, and return this image.
/// 
/// ### Arguments
/// * `bar_img` - Image the bars should contain.
/// * `barchart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn create_vertical_image_barchart(bar_img: &DynamicImage, barchart: &Chart) -> DynamicImage {
    let slate_grey = Rgb { r: 33, g: 33, b: 36};

    let mut img = new_with_background(barchart.width, barchart.height, &slate_grey);
    draw_vertical_image_barchart(&mut img, bar_img, &barchart);
    return img;
}

/// Create a new horizontal barchart image, and return this image.
/// 
/// ### Arguments
/// * `bar_img` - Image the bars should contain.
/// * `barchart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn create_horizontal_image_barchart(bar_img: &DynamicImage, barchart: &Chart) -> DynamicImage {
    let slate_grey = Rgb { r: 33, g: 33, b: 36};

    let mut img = new_with_background(barchart.width, barchart.height, &slate_grey);
    draw_horizontal_image_barchart(&mut img, bar_img, &barchart);
    return img;
}

/// Create a new horizontal histogram chart, and return this image.
/// 
/// ### Arguments
/// * `barchart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn create_horizontal_histogram(barchart: &Chart) -> DynamicImage {
    let slate_grey = Rgb { r: 33, g: 33, b: 36};

    let mut img = new_with_background(barchart.width, barchart.height, &slate_grey);
    draw_horizontal_histogram(&mut img, &barchart);
    return img;
}

///  Create a new vertical histogram chart, and return this image.
/// 
/// ### Arguments
/// * `barchart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn create_vertical_histogram(barchart: &Chart) -> DynamicImage {
    let slate_grey = Rgb { r: 33, g: 33, b: 36};
    let mut img = new_with_background(barchart.width, barchart.height, &slate_grey);
    draw_vertical_histogram(&mut img, &barchart);
    return img;
}

///  Create a new horizontal barchart, and return this image.
/// 
/// ### Arguments
/// * `barchart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn create_horizontal_barchart(barchart: &Chart) -> DynamicImage {
    let slate_grey = Rgb { r: 33, g: 33, b: 36};
    let mut img = new_with_background(barchart.width, barchart.height, &slate_grey);
    draw_horizontal_barchart(&mut img, &barchart);
    return img;
}

///  Create a new vertical barchart, and return this image.
/// 
/// ### Arguments
/// * `barchart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn create_vertical_barchart(barchart: &Chart) -> DynamicImage {
    let slate_grey = Rgb { r: 33, g: 33, b: 36};
    let mut img = new_with_background(barchart.width, barchart.height, &slate_grey);
    draw_vertical_barchart(&mut img, &barchart);
    return img;
}

// Draw labels onto the axes of a chart, typically for bar or line charts.
pub fn draw_labels(img: &mut DynamicImage, chart: &Chart) {
    draw_axes(img, chart);
    let axis_len = chart.width as f32 * 0.8;
    let x_inc = axis_len / chart.data.len() as f32 * 1.05;

    let num_bars: u32 = chart.data.len() as u32;
    let bar_width: f32 = ((chart.width / num_bars) as f32 * 0.8);

    let white = Rgb { r: 255, g: 255, b: 255};
    let mut start_x = bar_width / 2.0;

    let y_pos_label = chart.height - 30;
    
    for label in &chart.labels {
        draw_text(img, label, start_x as u32, y_pos_label as u32, "Roboto-Regular", 30.0, &white);

        start_x += bar_width;
    }    
}

// Draw x and y-axes to the image, mainly for bar charts and line charts.
fn draw_axes(img: &mut DynamicImage, chart: &Chart) {
    let line_pixel = image::Rgba([255, 167, 90, 255]);

    let axis_len = chart.width as f32 * 0.8;

    // Origin point
    let start_x = 20.0;
    let start_y: f32 = chart.height as f32 - 40.0;

    // End point on y-axis 
    let end_y_yaxis: f32 = start_y - axis_len;

    // End point on x-axis
    let end_x_xaxis: f32 = start_x + axis_len;
    
    // Draw y-axis
    draw_line_segment_mut(img, (start_x as f32, start_y as f32), (start_x, end_y_yaxis), line_pixel);

    // Draw x-axis 
    draw_line_segment_mut(img, (start_x as f32, start_y as f32), (end_x_xaxis, start_y), line_pixel);
}

// Draw an image as a bar component of a bar chart. 
// This can be used for custom bar colours or custom bar images within bar charts.
fn draw_image_as_bar(img: &mut DynamicImage, bar_img: &DynamicImage, bar_width: u32, bar_height: u32, start_x: u32, start_y: u32) {
    let sampling_filter = image::FilterType::Nearest;
    let resized_img = image::ImageRgba8(image::imageops::resize(bar_img, bar_width as u32, bar_height as u32, sampling_filter));
    image::imageops::overlay(img, &resized_img, start_x, start_y);        
}