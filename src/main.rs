mod algorithms;
mod image_manager;

use image::{DynamicImage};
use image_manager::{ImageManager, Args};
use algorithms::spiral::Spiral;

trait Generator {
    fn gen_image(args: &Args) -> DynamicImage;
    fn name() -> &'static str;
}

fn main() {
    ImageManager::run::<Spiral>(&Args::new(100, 100, r"C:\Users\Gabriel\OneDrive\Coding\Projects\Paperos\papero\out\")).unwrap();
}

