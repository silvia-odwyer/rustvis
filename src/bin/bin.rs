extern crate rustvis;
extern crate time;
use rustvis::{Rgb, barchart, new_with_background};
use rustvis::scatterplot::{draw_scatterplot};
use rustvis::linechart::*;
use time::PreciseTime;
use rustvis::barchart::*;

fn main() {
    let start = PreciseTime::now();

    let white = Rgb { r: 255, g: 255, b: 255};
    let black = Rgb { r: 0, g: 0, b: 0};
    let slate_grey = Rgb { r: 33, g: 33, b: 36};
    let orange = Rgb {r: 239, g: 132, b: 62};
    let blue_g = Rgb {r: 67, g: 145, b: 200};
    let mut img = rustvis::new_with_background(1000, 700, &slate_grey);

    // Insert the data into a vec
    let data: Vec<u16> = vec![25, 30, 50, 10, 14, 56, 54, 12, 34, 54];

    // Create labels for the barchart
    let labels: Vec<&'static str> = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
    
    // Barchart bar color
    let blue = Rgb { r: 40, g: 50, b: 200};

    // Create a barchart struct
    let barchart = Chart::new("Earnings for 2019/2020".to_string(), blue_g, white, data, labels, "".to_string(), "".to_string(), 1000, 700);

    draw_horizontal_histogram(&mut img, &barchart);
    rustvis::save_image(img, "barchart.png");

    let end = PreciseTime::now();
    println!("Took {} seconds to create image.", start.to(end));
    println!("You'll find the output image in examples/example_output");
}