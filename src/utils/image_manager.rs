#![allow(dead_code)]

use std::path::PathBuf;
use std::time::Duration;
use image::{ImageResult};
use strum::{EnumIter, IntoEnumIterator};
use rayon::prelude::*;
use crate::*;

/// A struct that manages the running of generators, and the saving of images
pub struct ImageManager {}

impl ImageManager {
    /// Saves the image to the path specified in args
    fn save(image: &DynamicImage, args: &Args, name: &str) -> ImageResult<()> {
        let output: RgbaImage = image.to_rgba8();
        let dir = Self::get_output_path(args, name);
        output.save(dir)
    }

    /// Gets the name of the generator
    fn name<T: Generator>() -> &'static str {
        T::name()
    }

    /// Runs the generator and sets the image as the wallpaper
    pub(crate) fn run_wallpaper<T: Generator>(args: &Args) {
        Self::run::<T>(args).unwrap();
        Self::set_as_wallpaper(args, T::name());
        println!("Set image as wallpaper");
    }

    /// Gets the path to the output image
    /// args.output_dir/name.png
    fn get_output_path(args: &Args, name: &str) -> PathBuf {
        args.output_dir.clone().join(format!("{}.png", name))
    }

    /// Runs the generator and saves the image to the path specified in args
    pub(crate) fn run<T: Generator>(args: &Args) -> ImageResult<()> {
        let name = T::name();
        println!("Generating an image with {}", name);
        let start = std::time::Instant::now();
        let image = T::generate(args);
        println!("Finished generating image in {:?}", start.elapsed());
        let res = ImageManager::save(&image, args, name);
        println!("Saved image to {}", ImageManager::get_output_path(args, name).to_str().unwrap());
        res
    }

    /// Runs without print statements helpful for benchmarking and when running all
    pub(crate) fn run_silent<T: Generator>(args: &Args) -> (String, std::time::Duration) {
        let name = T::name();
        if std::fs::exists(Self::get_output_path(args, name)).unwrap_or(false) {
            return (format!("Skipped {name}"), Duration::from_micros(0))
        }
        let start = std::time::Instant::now();
        let image = T::generate(args);
        let time = start.elapsed();
        let _res = ImageManager::save(&image, args, name);
        (name.to_string(), time)
    }

    /// Sets the image at the given path as the wallpaper
    fn set_as_wallpaper(args: &Args, name: &str) {
        let path = Self::get_output_path(args, name);
        wallpaper::set_mode(wallpaper::Mode::Span).unwrap();
        wallpaper::set_from_path(path.to_str().unwrap()).unwrap();
    }

    /// Run all generators at the same resolution in series
    pub(crate) fn run_all(args: &Args) {
        GeneratorTypes::iter().for_each(|x| {
            print!("{:<22}", x.name());
            let (_name, time) = x.run(args);
            let secs = time.as_secs_f64();
            let (whole, fract) = (secs as u32, (secs.fract() * 100.) as u32);
            println!(" {:>3}.{:<2}s", whole, fract);
        });
    }

    /// Run all generators at the same resolution in parallel
    pub(crate) fn run_all_fast(args: &Args) {
        let types = GeneratorTypes::iter().collect::<Vec<_>>();
        types.into_par_iter().for_each(|x| {
            let (name, time) = x.run(args);
            let secs = time.as_secs_f64();
            let (whole, fract) = (secs as u32, (secs.fract() * 100.) as u32);
            println!("{:<22} {:>3}.{:<2}s", name, whole, fract);
        });
    }

    /// Run the generator at a higher resolution and downscale it
    /// Thus the output image will be args.width x args.height
    /// Does nice anti-aliasing
    pub(crate) fn run_at_higher_res_and_downscale<T: Generator>(args: &Args, n: u32) -> ImageResult<()> {
        if n == 0 {
            panic!("Cannot downscale by factor 0, how would we get it back again")
        }

        let args = Args::new(args.width * n, args.height * n, args.output_dir.clone());
        let name = T::name();
        println!("Generating an image with {}, upscaled by factor {n}", name);
        let start = std::time::Instant::now();
        let image = T::generate(&args);
        println!("Finished generating image in {:?}", start.elapsed());
        let new_image = utils::upscale::downscale(image, n);
        let res = ImageManager::save(&new_image, &args, name);
        println!("Saved image to {}", ImageManager::get_output_path(&args, name).to_str().unwrap());
        res
    }

    /// Runs the generator at a higher resolution
    ///
    /// It's nice when you want to specify one resolution, but receive another; or when you are testing several versions of an algorithm
    pub(crate) fn run_res_mult<T: Generator>(args: &Args, n: u32) -> ImageResult<()> {
        if n == 0 {
            panic!("Cannot downscale by factor 0, how would we get it back again")
        }

        let new_args = Args::new(args.width * n, args.height * n, args.output_dir.clone());
        Self::run::<T>(&new_args)
    }
}

// This trait is crazy
// It generates a list of all the generators
// See below
macro_rules! generator_types {
    ( $( $variant:ident : $path:path ),+ $(,)? ) => {
        #[derive(EnumIter)]
        enum GeneratorTypes {
            $(
                $variant,
            )+
        }

        impl GeneratorTypes {
            fn run(self, args: &Args) -> (String, std::time::Duration) {
                match self {
                    $(
                        GeneratorTypes::$variant => {
                            ImageManager::run_silent::<$path>(args)
                        }
                    ),+
                }
            }
            fn name(&self) -> &str {
                match self {
                    $(
                        GeneratorTypes::$variant => {
                            ImageManager::name::<$path>()
                        }
                    ),+
                }
            }
        }
    }
}

/// Add your type and its path below
generator_types! {
    Mandel:     algorithms::maths::mandel::Mandel,
    Hilbert:    algorithms::maths::hilbert::Hilbert,
    Pinski:     algorithms::maths::pinski::Pinski,
    //
    Domain:     algorithms::particle::domain::DomainWarping,
    Flow:       algorithms::particle::flow::Flow,
    Voronoi:    algorithms::particle::voronoi::Voronoi,
    //
    Bitwise:    algorithms::pixel::bitwise::Bitwise,
    Gradient:   algorithms::pixel::gradient_test::Boring,
    Noise:      algorithms::pixel::noise::NoiseRender,
    Spiral:     algorithms::pixel::spiral::Spiral,
    Waterfall:  algorithms::pixel::waterfall::Waterfall,
    //
    Hex:        algorithms::shapes::hex::Hex
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
    pub(crate) fn image_f32(&self, colour: Rgb<f32>) -> Rgb32FImage {
        let mut image = Rgb32FImage::new(self.width, self.height);
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
