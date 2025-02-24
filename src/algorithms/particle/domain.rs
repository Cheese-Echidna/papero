use crate::utils::image_manager::Args;
use crate::{utils, Generator};
use glam::Vec2;
use image::{DynamicImage, Rgb, RgbImage};
use palette::LinSrgb;
use palette::named::{BLACK, WHITE};
use crate::utils::colour_utils::ImageColour;

#[derive(Default)]
pub(crate) struct DomainWarping {}

impl Generator for DomainWarping {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = RgbImage::new(args.width, args.height);

        let seed = 0;

        let f = utils::noise::BetterFbm::new(seed, 1, 0.0015);
        let g = utils::noise::BetterFbm::new(seed + 1, 1, 0.0015);

        let domain_warp = |v: Vec2, shove: f32| {
            let x = f.get(v.as_dvec2());
            let y = g.get(v.as_dvec2());
            v + Vec2::new(x as f32, y as f32) * shove
        };

        let (w, h) = args.wh();
        let (w_f32, h_f32) = (w as f32, h as f32);

        let colour = |q: Vec2| -> image::Rgb<u8> {
            let spacing = 100.;
            let width = 15.;
            let border_width = 2.;
            let p = q + Vec2::new(spacing / 2., spacing / 2.);

            let band = |p: Vec2, offset: f32| {
                ((p.x + offset) % spacing) < width || ((p.y + offset) % spacing) < width
            };

            let band_border = |p: Vec2, offset: f32| {
                let x = (p.x + offset) % spacing;
                let y = (p.y + offset) % spacing;

                let bx = x < border_width || (x > width - border_width && x < width);
                let by = y < border_width || (y > width - border_width && y < width);

                bx || by
            };

            let primary = band(p, 0.0);
            let primary_border = band_border(p, 0.0);
            let secondary = band(p, spacing / 2.);
            let secondary_border = band_border(p, spacing / 2.);

            let ax = (p.x / spacing) as u32 % 2 == 0;
            let ay = (p.y / spacing) as u32 % 2 == 0;
            let on_top = ax || ay;

            let c1 = if primary_border {
                Rgb::<u8>::from_const(WHITE)
            } else {
                utils::colour_utils::into_no_alpha(LinSrgb::new(p.x / w_f32 * 0.7, 0., p.y / h_f32))
            };

            let c2 = if secondary_border {
                Rgb::<u8>::from_const(WHITE)
            } else {
                utils::colour_utils::into_no_alpha(LinSrgb::new(0., p.y / w_f32 * 0.7, 1.0 - p.x / h_f32))
            };

            if primary && secondary {
                if on_top {
                    c1
                } else {
                    c2
                }
            } else if primary {
                c1
            } else if secondary {
                c2
            } else {
                Rgb::<u8>::from_const(BLACK)
            }
        };

        for x in 0..w {
            for y in 0..h {
                let pos = Vec2::new(x as f32, y as f32);
                let new_pos = domain_warp(pos, 100.0);

                image.put_pixel(x, y, colour(new_pos));
            }
        }

        image.into()
    }

    fn name() -> &'static str {
        "Domain Warping"
    }
}
