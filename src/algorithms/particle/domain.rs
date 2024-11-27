use glam::Vec2;
use image::{DynamicImage, RgbImage, RgbaImage};
use crate::algorithms::particle::particle::Particle;
use crate::{utils, Generator};
use crate::utils::image_manager::Args;
use noise::{NoiseFn, OpenSimplex};
use palette::LinSrgb;

#[derive(Default)]
pub(crate) struct DomainWarping {}

impl Generator for DomainWarping {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = RgbImage::new(args.width, args.height);

        let seed = 0;

        let f = FBM::new(seed, 5);
        let g = FBM::new(seed + 1, 5);

        let domain_warp = |v: Vec2, scale: f32, shove: f32| {
            let x = f.get(v * scale + Vec2::new(0.1122, 0.6995));
            let y = g.get(v * scale + Vec2::new(0.5577, 0.1295));
            v + Vec2::new(x as f32, y as f32) * shove
        };

        let (w, h) = args.wh();
        let (w_f32, h_f32) = (w as f32, h as f32);

        let colour = |q: Vec2| -> image::Rgb<u8> {
            let spacing = 80.0;
            let width = 20.;
            let p = q + Vec2::new(spacing / 2., spacing / 2.);
            let rx = (p.x % spacing) < width;
            let ry = (p.y % spacing) < width;
            if rx || ry {
                utils::colour_utils::into_no_alpha(LinSrgb::new(p.x / w_f32, 0., p.y / h_f32,))
            } else {
                utils::colour_utils::into_no_alpha(LinSrgb::new(0., 0., 0.,))
            }
        };

        for x in 0..w {
            for y in 0..h {
                let pos = Vec2::new(x as f32, y as f32);
                let new_pos = domain_warp(pos, 0.0025, 20.0);

                image.put_pixel(
                    x,
                    y,
                    colour(new_pos)
                );
            }
        }

        image.into()
    }

    fn name() -> &'static str {
        "Domain Warping"
    }
}

struct FBM {
    n: u32,
    seed: u32,
    noise_fn: OpenSimplex
}

impl FBM {
    fn new(seed: u32, n: u32) -> Self {
        FBM { seed, n, noise_fn: OpenSimplex::new(seed) }
    }
}

impl FBM {
    fn get(&self, point: Vec2) -> f64 {
        let mut v = 0.;
        for i in 0..self.n {
            v += self.noise_fn.get([point.x as f64, point.y as f64]) * 2_f64.powi(-(i as i32));
        }
        v
    }
}