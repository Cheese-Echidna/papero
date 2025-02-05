use std::path::PathBuf;
use image::{ImageResult, Pixel};
use strum::{EnumIter, IntoEnumIterator};

use crate::*;

pub struct ImageManager {}

impl ImageManager {
    fn save(image: &DynamicImage, args: &Args, name: &str) -> ImageResult<()> {
        let output: RgbaImage = image.to_rgba8();
        let dir = Self::get_output_path(args, name);
        output.save(dir)
    }

    pub(crate) fn run_wallpaper<T: Generator>(args: &Args) {
        Self::run::<T>(args).unwrap();
        Self::set_as_wallpaper(&args, T::name());
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
        let start = std::time::Instant::now();
        let image = T::generate(args);
        println!("Finished generating image in {:?}", start.elapsed());
        let res = ImageManager::save(&image, args, name);
        println!("Saved image to {}\\{}.png", ImageManager::get_output_path(&args, name).to_str().unwrap(), name);
        res
    }

    pub(crate) fn run_silent<T: Generator>(args: &Args) -> (&str, std::time::Duration) {
        let name = T::name();
        let start = std::time::Instant::now();
        let image = T::generate(args);
        let time = start.elapsed();
        let res = ImageManager::save(&image, args, name);
        (name, time)
    }

    fn set_as_wallpaper(args: &Args, name: &str) {
        let path = Self::get_output_path(args, name);
        wallpaper::set_mode(wallpaper::Mode::Span).unwrap();
        wallpaper::set_from_path(path.to_str().unwrap()).unwrap();
    }

    pub(crate) fn run_all(args: &Args) {
        GeneratorTypes::iter().for_each(|x| {
            let (name, time) = x.run(args);
            let secs = time.as_secs_f64();
            let (whole, fract) = (secs as u32, (secs.fract() * 100.) as u32);
            println!("{:<22} {:>3}.{:<2}s", name, whole, fract);
        });

    }

    pub(crate) fn run_and_upscale<T: Generator>(args: &Args, n: u32) -> ImageResult<()> {
        if n == 0 {
            panic!("Cannot downscale by factor 0, how would we get it back again")
        }
        if n == 1 {
            return ImageManager::run::<T>(args)
        }
        assert_eq!(args.width % n, 0, "n must be a factor of the width");
        assert_eq!(args.height % n, 0, "n must be a factor of the height");

        let new_args = Args::new(args.width / n, args.height / n, args.output_dir.clone());
        let name = T::name();
        println!("Generating an image with {}, downscaled by factor {n}", name);
        let start = std::time::Instant::now();
        let image = T::generate(&new_args);
        println!("Finished generating image in {:?}", start.elapsed());
        println!("Upscaling image by factor {n}");
        let new_image = utils::upscale::upscale(image, n);
        let res = ImageManager::save(&new_image, args, name);
        println!("Saved image to {}\\{}.png", ImageManager::get_output_path(&args, name).to_str().unwrap(), name);
        res
    }

    pub(crate) fn run_res_mult<T: Generator>(args: &Args, n: u32) -> ImageResult<()> {
        if n == 0 {
            panic!("Cannot downscale by factor 0, how would we get it back again")
        }

        let new_args = Args::new(args.width * n, args.height * n, args.output_dir.clone());
        Self::run::<T>(&new_args)
    }
}

macro_rules! generator_types {
    ( $( $variant:ident : $path:path ),+ $(,)? ) => {
        #[derive(EnumIter)]
        enum GeneratorTypes {
            $(
                $variant($path),
            )+
        }

        impl GeneratorTypes {
            fn run(self, args: &Args) -> (&str, std::time::Duration) {
                match self {
                    $(
                        GeneratorTypes::$variant(x) => {
                            ImageManager::run_silent::<$path>(args)
                        }
                    ),+
                }
            }
        }
    }
}

generator_types! {
    // Mandel: algorithms::maths::mandel::Mandel,
    Voronoi: algorithms::particle::voronoi::Voronoi,
    Spiral: algorithms::pixel::spiral::Spiral,
    Waterfall: algorithms::pixel::waterfall::Waterfall,
}

pub(crate) struct Args {
    pub(crate) width: u32,
    pub(crate) height: u32,
    output_dir: PathBuf,
}

impl Args {
    pub(crate) fn new(width: u32, height: u32, dir: impl Into<PathBuf>) -> Self {
        Self {
            width,
            height,
            output_dir: dir.into(),
        }
    }
    pub(crate) fn wh(&self) -> (u32, u32) {(self.width, self.height)}

    pub(crate) fn image_u8(&self, colour: Rgb<u8>) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                image.put_pixel(x, y, colour);
            }
        }
        image
    }
    pub(crate) fn image_f32_alpha(&self, colour: Rgba<f32>) -> Rgba32FImage {
        let mut image = Rgba32FImage::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                image.put_pixel(x, y, colour);
            }
        }
        image
    }
}
