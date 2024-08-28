mod algorithms;
mod utils;

use image::{DynamicImage, RgbaImage, Rgba32FImage, Rgba, GenericImageView};
use rand;
use rand::Rng;
use utils::*;

use image_manager::{ImageManager, Args};

trait Generator {
    fn gen_image(args: &Args) -> DynamicImage;
    fn name() -> &'static str;
}

fn main() {
    ImageManager::run_wallpaper::<algorithms::particle::voronoi::Voronoi>(&Args::new(1920, 1080, r"C:\Users\Gabriel\OneDrive\Coding\Projects\Paperos\papero\out\"));
}

