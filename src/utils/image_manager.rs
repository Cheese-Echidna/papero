use std::path::PathBuf;
use image::ImageResult;
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

    fn set_as_wallpaper(args: &Args, name: &str) {
        let path = Self::get_output_path(args, name);
        wallpaper::set_mode(wallpaper::Mode::Span).unwrap();
        wallpaper::set_from_path(path.to_str().unwrap()).unwrap();
    }

    pub(crate) fn run_all(args: &Args) {
        GeneratorTypes::iter().for_each(|x| {let _ = x.run(args);});
    }
}


#[derive(EnumIter)]
enum GeneratorTypes {
    Mandel(algorithms::complex::mandel::Mandel),
    Voronoi(algorithms::particle::voronoi::Voronoi),
    Spiral(algorithms::pixel::spiral::Spiral),
    Waterfall(algorithms::pixel::waterfall::Waterfall
    ),
}
impl GeneratorTypes {
    fn run(self, args: &Args) -> ImageResult<()> {
        match self {
            GeneratorTypes::Mandel(x) => {
                ImageManager::run::<algorithms::complex::mandel::Mandel>(args)
            }
            GeneratorTypes::Voronoi(x) => {
                ImageManager::run::<algorithms::particle::voronoi::Voronoi>(args)
            }
            GeneratorTypes::Spiral(x) => {
                ImageManager::run::<algorithms::pixel::spiral::Spiral>(args)
            }
            GeneratorTypes::Waterfall(x) => {
                ImageManager::run::<algorithms::pixel::waterfall::Waterfall>(args)
            }
        }
    }
}

// macro_rules! generate_enum {
//     ($($variant:ident => $path:path),*) => {
//         #[derive(Sequence)]
//         enum GeneratorTypes {
//             $(
//                 $variant($path),
//             )*
//         }
//
//         impl GeneratorTypes {
//             fn run(self, args: &Args) -> ImageResult<()> {
//                 match self {
//                     $(
//                         GeneratorTypes::$variant(x) => {
//                             ImageManager::run::<$path>(args)
//                         }
//                     )*
//                 }
//             }
//         }
//     };
// }
//
// generate_enum!(
//     Mandel => algorithms::complex::mandel::Mandel,
//     Voronoi => algorithms::particle::voronoi::Voronoi,
//     Spiral => algorithms::pixel::spiral::Spiral,
//     Waterfall => algorithms::pixel::waterfall::Waterfall
// );


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
}
