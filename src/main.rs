mod algorithms;
mod utils;

use image::{DynamicImage, RgbaImage, Rgba32FImage, Rgba, GenericImageView, Rgb, Rgb32FImage};
use rand;
use rand::Rng;
use utils::*;

use image_manager::{ImageManager, Args};

trait Generator {
    fn gen_image(args: &Args) -> DynamicImage;
    fn name() -> &'static str;
}

fn main() {
    ImageManager::run_wallpaper::<algorithms::complex::mandel::Mandel>(&Args::new(1920, 1080, r"C:\Users\Gabriel\OneDrive\Coding\Projects\Paperos\papero\out\"));
}

