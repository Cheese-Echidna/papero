mod algorithms;
mod utils;

use image::{DynamicImage, RgbaImage, Rgba32FImage, Rgba, GenericImageView, Rgb, Rgb32FImage, RgbImage};
use rand;
use rand::Rng;
use utils::*;

use image_manager::{ImageManager, Args};

trait Generator : Default {
    fn generate(args: &Args) -> DynamicImage;
    fn name() -> &'static str;
}

fn main() {
    // 1792, 828
    ImageManager::run::<algorithms::pixel::boring::Boring>
        (&Args::new(1920, 1080, r"C:\Users\Gabriel\OneDrive\Coding\Projects\Paperos\papero\out\")).unwrap();
}

