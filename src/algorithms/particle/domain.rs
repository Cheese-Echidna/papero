use image::{DynamicImage, RgbaImage};
use crate::algorithms::particle::particle::Particle;
use crate::Generator;
use crate::utils::image_manager::Args;
use noise::OpenSimplex;
use crate::utils::colour_utils::random_colour;

#[derive(Default)]
struct DomainWarping {
    particles: Vec<Particle>,
    seed: u32
}

impl Generator for DomainWarping {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = RgbaImage::new(args.width, args.height);

        let mut warping = DomainWarping {
            particles: (0..1000).map(|_| Particle::new_random(args.width, args.height)).collect(),
            seed: 0,
        };


        image.into()
    }

    fn name() -> &'static str {
        "Domain Warping"
    }
}