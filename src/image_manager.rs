use image::ImageResult;
use crate::*;

pub(crate) struct ImageManager {}

impl ImageManager {
    fn save(image: &DynamicImage, args: &Args, name: &str) -> ImageResult<()> {
        let output: image::RgbaImage = image.to_rgba8();
        let mut dir = args.output_dir.clone();
        dir.push(format!("{}.png", name));
        output.save(dir)
    }

    pub(crate) fn run<T: Generator>(args: &Args) -> ImageResult<()> {
        let name = T::name();
        println!("Generating an image with {}", name);
        let image = T::gen_image(args);
        println!("Finished generating image");
        let res = ImageManager::save(&image, args, name);
        println!("Saved image to {}\\{}.png", args.output_dir.to_str().unwrap(), name);
        res
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