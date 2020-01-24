// https://github.com/image-rs/image
extern crate image;

use image::imageops;
use image::GenericImageView;

fn scale_down(dimensions: (u32, u32)) -> (u32, u32) {
    let ratio = dimensions.0 as f32 / dimensions.1 as f32;
    let height = dimensions.1 as f32 / ratio;
    (600, height as u32)
}

#[test]
fn test_scale_down() {
    let dimensions = scale_down((1424, 751));
    assert_eq!(dimensions, (600, 396));
}

pub fn run(mut img: image::DynamicImage) -> image::DynamicImage {
    let dimensions = img.dimensions();
    if dimensions.0 > 600 {
        let dimensions = scale_down(dimensions);
        img = img.resize(dimensions.0, dimensions.1, imageops::Nearest);
    }
    img.grayscale()
}

#[test]
fn test_run() {
    let img = image::open("static/croatia.png").unwrap();
    let img = run(img);
    let dimensions = img.dimensions();
    println!("{:?}", dimensions);
    assert_eq!(dimensions.0, 600);
    let color = img.color();
    assert_eq!(color, image::ColorType::Gray(8));
}
