use crate::utils::colour_utils::ImageColour;
use crate::utils::image_manager::Args;
use crate::{utils, Generator};
use glam::Vec2;
use image::{DynamicImage, Rgb, RgbImage};
use palette::named::{BLACK};
use palette::LinSrgb;

const SCALE: f32 = 1.0;

#[derive(Default)]
pub(crate) struct DomainWarping {}

impl Generator for DomainWarping {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = RgbImage::new(args.width, args.height);

        let seed = 0;

        let f = utils::noise::BetterFbm::new(seed, 1, 0.0015 / SCALE as f64);
        let g = utils::noise::BetterFbm::new(seed + 1, 1, 0.0015 / SCALE as f64);

        let domain_warp = |v: Vec2, shove: f32| {
            let x = f.get(v.as_dvec2());
            let y = g.get(v.as_dvec2());
            v + Vec2::new(x as f32, y as f32) * shove
        };

        let (w, h) = args.wh();
        let (w_f32, h_f32) = (w as f32, h as f32);

        let colour = |q: Vec2| -> image::Rgb<u8> {
            let spacing = 100. * SCALE;
            let width = 50. * SCALE;
            let border_width = 4. * SCALE;
            let p = q + Vec2::new(spacing / 2., spacing / 2.);

            let band = |p: f32, offset: f32| ((p + offset) % spacing) < width;

            let band_border = |p: f32, offset: f32| {
                let p2 = (p + offset) % spacing;
                p2 < border_width || (p2 > width - border_width && p2 < width)
            };

            let hoz = band(p.x, 0.0);
            let hoz_border = band_border(p.x, 0.0);
            let vert = band(p.y, spacing / 2.);
            let vert_border = band_border(p.y, spacing / 2.);

            let ax = (p.x / (spacing)) as u32 % 2 == 0;
            let ay = (p.y / (spacing)) as u32 % 2 == 0;
            let on_top = ax ^ ay;

            let mut c1 = utils::colour_utils::into_no_alpha(LinSrgb::new(
                p.x / w_f32 * 0.7,
                0.,
                p.y / h_f32,
            ));
            if hoz_border {
                c1 = Rgb::<u8>::from_const(BLACK);
            }

            let mut c2 = utils::colour_utils::into_no_alpha(LinSrgb::new(
                0.,
                p.y / w_f32 * 0.7,
                1.0 - p.x / h_f32,
            ));

            if vert_border {
                c2 = Rgb::<u8>::from_const(BLACK);
            }

            if hoz && vert {
                if on_top {
                    c1
                } else {
                    c2
                }
            } else if hoz {
                c1
            } else if vert {
                c2
            } else {
                Rgb::<u8>::from_const(BLACK)
            }
        };

        for x in 0..w {
            for y in 0..h {
                let pos = Vec2::new(x as f32, y as f32);
                let new_pos = domain_warp(pos, 100.0 * SCALE);

                image.put_pixel(x, y, colour(new_pos));
            }
        }

        image.into()
    }

    fn name() -> &'static str {
        "Domain Warping"
    }
}
