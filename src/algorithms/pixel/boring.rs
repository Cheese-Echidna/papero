use crate::*;
use crate::algorithms::particle::particle::Point;
use rayon::prelude::*;
use num_utils::*;

const BLACK: Rgb<f32> = Rgb([0.0, 0.0, 0.0]);

#[derive(Default)]
pub struct Boring;

impl Generator for Boring {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = Rgb32FImage::new(args.width, args.height);

        let (w, h) = (args.width as f32, args.height as f32);

        for py in 0..args.height {
            for px in 0..args.width {
                let (x, y) = (px as f32 / w, py as f32 / h);
                let c = colour_utils::sick_gradient(x, y);
                image.put_pixel(px, py, c);
            }
        }
        image.into()
    }

    fn name() -> &'static str {
        "Boring gradient"
    }
}

