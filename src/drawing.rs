extern crate image;
extern crate imageproc;
use image::{DynamicImage, Rgba, RgbaImage, GenericImage, GenericImageView};
use imageproc::drawing::*;
use imageproc::rect::Rect;
use palette::{LinSrgba, Gradient, Lch, Srgba};
use palette::encoding::pixel::Pixel;
use crate::{Rgb};


/// Create a gradient element in the shape of a Rect.
/// 
/// Returns a DynamicImage.
/// 
/// ### Arguments
/// * `width` - u32 - Desired width of gradient.
/// * `height` - u32 - Desired height of gradient.
pub fn create_gradient(width: u32, height: u32) -> DynamicImage {
    let mut image = RgbaImage::new(width, height);

    // Create a gradient.
    let grad1 = Gradient::new(vec![
        LinSrgba::new(1.0, 0.1, 0.1, 1.0),
        LinSrgba::new(0.1, 0.1, 1.0, 1.0),
        LinSrgba::new(0.1, 1.0, 0.1, 1.0),
    ]);

    let _grad3 = Gradient::new(vec![
        Lch::from(LinSrgba::new(1.0, 0.1, 0.1, 1.0)),
        Lch::from(LinSrgba::new(0.1, 0.1, 1.0, 1.0)),
        Lch::from(LinSrgba::new(0.1, 1.0, 0.1, 1.0)),
    ]);

    for (i, c1) in grad1
        .take(width as usize)
        .enumerate()
    {
        let c1 = Srgba::from_linear(c1).into_format().into_raw();
        {
            let mut sub_image = image.sub_image(i as u32, 0, 1, height);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgba {
                        data: c1
                    });
                }
            }
        }
    }
    let rgba_img = image::ImageRgba8(image);
    return rgba_img;
}

/// Apply a preset gradient by passing in a name. 
///
/// ### Arguments
/// * `width` - u32 - Desired width of rectangle.
/// * `height` - u32 - Desired height of rectangle.
/// * `name` - The preset to be used. Presets available include: pinkblue, lemongrass
pub fn create_gradient_preset(width: u32, height: u32, name: &str) -> DynamicImage {
    let mut image = RgbaImage::new(width, height);

    let gradient = match name {
        "pinkblue" => Gradient::new(vec![
            LinSrgba::new(0.2039, 0.5803, 0.90196, 1.0),
            LinSrgba::new(0.92549, 0.431372549, 0.6784313 , 1.0),
        ]),
        "lemongrass" => Gradient::new(vec![
            LinSrgba::new(0.2039, 0.5803, 0.90196, 1.0),
            LinSrgba::new(0.40392156, 0.69803921, 0.43529 , 1.0),
        ]),
        "pink_pastel" => Gradient::new(vec![
            LinSrgba::new(0.93725, 0.19607, 0.85098, 1.0),
            LinSrgba::new(0.537254, 1.0, 0.9921568, 1.0),
        ]),
        "mauve_pastel" => Gradient::new(vec![
            LinSrgba::new(0.498039, 0.498039, 0.835294, 1.0),
            LinSrgba::new(0.537254, 0.6588235, 0.905882, 1.0),
        ]),
        _ => Gradient::new(vec![
            LinSrgba::new(0.2039, 0.5803, 0.90196, 1.0),
            LinSrgba::new(0.52549, 0.69803921, 0.43529 , 1.0),
        ])

    };

    for (i, c1) in gradient
        .take(width as usize)
        .enumerate()
    {
        let c1 = Srgba::from_linear(c1).into_format().into_raw();
        {
            let mut sub_image = image.sub_image(i as u32, 0, 1, height);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgba {
                        data: c1
                    });
                }
            }
        }
    }
    let rgba_img = image::ImageRgba8(image);
    return rgba_img;
}

/// Draw a solid rectangle with a given background colour. 
/// 
/// # Arguments
/// * `img` - A mutable ref to a DynamicImage.
/// * `background_color` - Rgb color of rectangle.
/// * `width` - u32 - Desired width of rectangle.
/// * `height` - u32 - Desired height of rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
pub fn draw_solid_rect(img: &mut DynamicImage, background_color: &Rgb, width: u32, height: u32, x_pos: i32, y_pos: i32) {    
    draw_filled_rect_mut(img, 
                        Rect::at(x_pos, y_pos).of_size(width, height), 
                        Rgba([background_color.r, background_color.g, 
                        background_color.b, 255u8]));

}


/// Preset: Draw a gradient rectangle filled with a gradient.
/// 
/// ### Arguments
/// * `img` - A mutable ref to a DynamicImage.
/// * `width` - u32 - Desired width of gradient rectangle.
/// * `height` - u32 - Desired height of gradient rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
/// * `preset_name` - Name of the preset. Examples include "lemongrass", "pink_blue", "pastel_pink", "pastel_mauve"
pub fn draw_preset_rect_gradient(img: &mut DynamicImage, width: u32, height: u32, x_pos: u32, y_pos: u32, preset_name: &str) {
    let rect = create_gradient_preset(width, height, preset_name);
        
    image::imageops::overlay(img, &rect, x_pos, y_pos);
}