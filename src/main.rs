use std::env;
use std::path::Path;

use image::DynamicImage;

use crate::image_converter::ImageConverter;
use crate::max_character_set::MaxCharacterSet;
use crate::ten_character_set::TenCharacterSet;

mod image_converter;
mod max_character_set;
mod ten_character_set;

extern crate image;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let img = image::open(&Path::new(&config.filename))
        .unwrap()
        .into_rgba8();

    let mut gray_image = DynamicImage::ImageRgba8(img).into_luma8();
    let character_set = MaxCharacterSet::new();

    let mut image_converter = ImageConverter::new(character_set);
    let result = image_converter.convert(&mut gray_image);

    println!("{}", result);
    let character_set_two = TenCharacterSet::new();
    let mut image_converter_two = ImageConverter::new(character_set_two);
    let result_two = image_converter_two.convert(&mut gray_image);

    println!("{}", result_two);
}

struct Config {
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    Config {
        filename: args[1].clone(),
    }
}
