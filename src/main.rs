mod algorithms;
mod utils;

use image::{DynamicImage, RgbaImage, Rgba32FImage, Rgba, GenericImageView, Rgb, Rgb32FImage, RgbImage};
use rand;
use rand::Rng;
use utils::*;
use crate::utils::colour_utils::into;

use image_manager::{ImageManager, Args};

trait Generator: Default {
    fn generate(args: &Args) -> DynamicImage;
    fn name() -> &'static str;
}

fn main() {
    let a = Args::new(1920, 1080, "out/");
    ImageManager::run::<algorithms::pixel::noise::NoiseRender>(&a).unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn all_images() {
        let demo_dir = r"C:\Users\Gabriel\OneDrive\Coding\Projects\Paperos\papero\demo";
        let args = Args::new(1920, 1080, demo_dir);
        ImageManager::run_all_fast(&args);
        let prefix = std::fs::read_to_string(r"C:\Users\Gabriel\OneDrive\Coding\Projects\Paperos\papero\prefix.md").unwrap();
        let infix = fs::read_dir(demo_dir).unwrap().into_iter().map(|x| x.unwrap().file_name()).map(|x| {
            let filename = x.to_str().unwrap();
            let file_path = "demo/".to_owned() + &filename.replace(" ", "%20");
            let name = filename.strip_suffix(".png").unwrap_or(filename);
            format!("\n---\n\n{name}\n\n![{filename}]({file_path})\n")
        }).collect::<String>();
        std::fs::write(r"C:\Users\Gabriel\OneDrive\Coding\Projects\Paperos\papero\readme.md", prefix + &infix).unwrap();
    }
}