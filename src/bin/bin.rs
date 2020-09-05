extern crate rustvis;
extern crate time;

use rustvis::barchart::*;
use rustvis::Rgb;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();

    let white = Rgb::new(255, 255, 255);
    let _black = Rgb::new(0, 0, 0);
    let slate_grey = Rgb::new(33, 33, 33);
    let _orange = Rgb::new(239, 132, 62);
    let blue_g = Rgb::new(67, 145, 200);
    let mut img = rustvis::new_with_background(1000, 700, &slate_grey);

    // Insert the data into a vec
    let data = vec![25_u16, 30, 50, 10, 14, 56, 54, 12, 34, 54];

    // Create labels for the barchart
    let labels = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];

    // Barchart bar color
    let _blue = Rgb::new(40, 50, 200);

    // Create a barchart struct
    let barchart = Chart::new(
        "Earnings for 2019/2020".to_string(),
        blue_g,
        white,
        data,
        labels,
        "".to_string(),
        "".to_string(),
        1000,
        700,
    );

    draw_vertical_gradient_barchart(&mut img, &barchart, "lemongrass");
    rustvis::save_image(img, "barchart.png");

    let end = PreciseTime::now();
    println!("Took {} seconds to create image.", start.to(end));
    println!("You'll find the output image titled barchart.png in the root of this dir");
}
