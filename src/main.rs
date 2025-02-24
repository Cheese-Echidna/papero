mod algorithms;
mod utils;

use crate::utils::colour_utils::into;
use image::{
    DynamicImage, GenericImageView, Rgb, Rgb32FImage, RgbImage, Rgba, Rgba32FImage, RgbaImage,
};
use rand::Rng;
use utils::*;

use image_manager::{Args, ImageManager};

trait Generator: Default {
    fn generate(args: &Args) -> DynamicImage;
    fn name() -> &'static str;
}

fn main() {
    let a = Args::new(1920, 1080, "./out");
    ImageManager::run::<algorithms::maths::mandel::Mandel>(&a).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn all_images() {
        let project_dir = project_root();
        let demo_dir = project_dir.join("demo");
        let args = Args::new(1920, 1080, &demo_dir);
        ImageManager::run_all_fast(&args);
        let prefix = std::fs::read_to_string(project_dir.join("prefix.md")).unwrap();
        let infix = fs::read_dir(demo_dir)
            .unwrap()
            .into_iter()
            .map(|x| x.unwrap().file_name())
            .map(|x| {
                let filename = x.to_str().unwrap();
                let file_path = "demo/".to_owned() + &filename.replace(" ", "%20");
                let name = filename.strip_suffix(".png").unwrap_or(filename);
                format!("\n---\n\n{name}\n\n![{filename}]({file_path})\n")
            })
            .collect::<String>();
        std::fs::write(project_dir.join("readme.md"), prefix + &infix).unwrap();
    }

    fn project_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }
}
