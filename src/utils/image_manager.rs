use std::path::PathBuf;
use image::ImageResult;
use crate::*;

pub struct ImageManager {}

impl ImageManager {
    fn save(image: &DynamicImage, args: &Args, name: &str) -> ImageResult<()> {
        let output: RgbaImage = image.to_rgba8();
        let dir = Self::get_output_path(args, name);
        output.save(dir)
    }

    pub(crate) fn run_wallpaper<T: Generator>(args: &Args) {
        let name = T::name();
        println!("Generating an image with {}", name);
        let image = T::gen_image(args);
        println!("Finished generating image");
        let _ = ImageManager::save(&image, args, name).unwrap();
        println!("Saved image to {}\\{}.png", ImageManager::get_output_path(&args, name).to_str().unwrap(), name);
        Self::set_as_wallpaper(&args, name);
        println!("Set image as wallpaper");
    }

    fn get_output_path(args: &Args, name: &str) -> PathBuf {
        let mut dir = args.output_dir.clone();
        dir.push(format!("{}.png", name));
        dir
    }

    pub(crate) fn run<T: Generator>(args: &Args) -> ImageResult<()> {
        let name = T::name();
        println!("Generating an image with {}", name);
        let image = T::gen_image(args);
        println!("Finished generating image");
        let res = ImageManager::save(&image, args, name);
        println!("Saved image to {}\\{}.png", ImageManager::get_output_path(&args, name).to_str().unwrap(), name);
        res
    }

    fn set_as_wallpaper(args: &Args, name: &str) {
        let path = Self::get_output_path(args, name);
        wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
        wallpaper::set_from_path(path.to_str().unwrap()).unwrap();
    }
}

pub(crate) struct Args {
    pub(crate) width: u32,
    pub(crate) height: u32,
    output_dir: std::path::PathBuf,
}

impl Args {
    pub(crate) fn new(width: u32, height: u32, dir: impl Into<std::path::PathBuf>) -> Self {
        Self {
            width,
            height,
            output_dir: dir.into(),
        }
    }
}