mod algorithms;
mod utils;

use image::{DynamicImage, RgbaImage, Rgba32FImage, Rgba, GenericImageView, Rgb, Rgb32FImage, RgbImage};
use rand;
use rand::Rng;
use utils::*;
use crate::utils::colour_utils::into;

use image_manager::{ImageManager, Args};

trait Generator : Default {
    fn generate(args: &Args) -> DynamicImage;
    fn name() -> &'static str;
}

fn main() {

    let a = Args::new(1920, 1080, r"C:\Users\Gabriel\OneDrive\Coding\Projects\Paperos\papero\out\");

    ImageManager::run::<algorithms::particle::domain::DomainWarping>(&a).unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_all() {
//
//     }
// }