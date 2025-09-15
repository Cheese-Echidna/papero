mod algorithms;
mod utils;

use std::io::{Cursor};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use glam::U8Vec3;
use crate::utils::colour_utils::into;
use image::{
    DynamicImage, GenericImageView, Rgb, Rgb32FImage, RgbImage, Rgba, Rgba32FImage, RgbaImage,
};
use rand::Rng;
use utils::*;
use wasm_bindgen::prelude::*;
use image_manager::{Args, ImageManager};

trait Generator: Default {
    fn generate(args: &Args) -> DynamicImage;
    fn name() -> &'static str;
}

#[wasm_bindgen]
pub fn generate_pinski() -> String {
    let a = Args::new(1400, 1000, "./out");
    let image = algorithms::maths::pinski::Pinski::real_generate(&a, U8Vec3::ZERO, U8Vec3::ONE, U8Vec3::new(2, 2, 2));
    base64_png(image)
}

pub fn base64_png(img: DynamicImage) -> String {
    let mut buf = Cursor::new(vec![]);
    img.write_to(&mut buf, image::ImageFormat::Png).unwrap();
    BASE64_STANDARD.encode(buf.into_inner())
}