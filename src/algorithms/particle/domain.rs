use glam::Vec2;
use image::{DynamicImage, RgbImage};
use crate::{utils, Generator};
use crate::utils::image_manager::Args;
use palette::LinSrgb;

#[derive(Default)]
pub(crate) struct DomainWarping {}

impl Generator for DomainWarping {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = RgbImage::new(args.width, args.height);

        let seed = 0;

        let f = |x, scale| utils::noise::fbm(seed,          scale, 5, x);
        let g = |x, scale| utils::noise::fbm(seed + 1, scale, 5, x);



        let domain_warp = |v: Vec2, scale: f32, shove: f32| {
            let x = f(v.as_dvec2(), scale as f64);
            let y = g(v.as_dvec2(), scale as f64);
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