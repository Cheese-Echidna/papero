use crate::utils::num_utils::lerp;
use crate::*;
use num::complex::{Complex64, ComplexFloat};
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct Mandel;

impl Generator for Mandel {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = Rgb32FImage::new(args.width, args.height);
        let (width, height) = (args.width as f64, args.height as f64);

        for py in 0..args.height {
            for px in 0..args.width {
                let centre = Complex64::new(1.4, 0.0);
                let mandel_width = 6.0;

                let (x, y) = (
                    (px as f64 / width - 0.5),
                    -((py as f64 / height - 0.5) * height / width),
                );
                let c = Complex64::new(x, y).scale(mandel_width) + centre;

                let (dist_bound, iter_bound) = (2.0, 200);

                let colour = escape_rgb(c.recip(), dist_bound, iter_bound);

                image.put_pixel(px, py, colour);
            }
        }

        image.into()
    }

    fn name() -> &'static str {
        "Mandelbrot"
    }
}

fn escape(c: Complex64, escaped: f64, limit: u32) -> (Complex64, u32) {
    let mut set = HashSet::new();

    let mut num = 0_u32;
    let mut z = c;
    set.insert(bits(z));

    let escaped_sqr = escaped.powi(2);

    while z.norm_sqr() < escaped_sqr && num < limit {
        if set.contains(&bits(z)) {
            return (z, limit);
        }
        z = z.powu(2) + c;
        num += 1;
        set.insert(bits(z));
    }

    (c, num)
}

fn escape_rgb(c: Complex64, escaped: f64, limit: u32) -> Rgb<f32> {
    let mut set = HashSet::new();

    let mut num = 0_u32;
    let mut z = c;
    set.insert(bits(z));

    let escaped_sqr = escaped.powi(2);

    while z.norm_sqr() < escaped_sqr && num < limit {
        z = z.powu(2) + c;
        num += 1;
        if set.contains(&bits(z)) {
            let limit_prop = num as f32 / limit as f32;

            return colour_utils::convert_from_ok_hsl(lerp(limit_prop, 1.0, 0.75), 1.0, 0.5);
        }
        set.insert(bits(z));
    }

    let limit_prop = num as f32 / limit as f32;

    if z.norm_sqr() >= escaped_sqr {
        return colour_utils::convert_from_ok_hsl(0.8, 1.0, 1.0 - limit_prop);
    }

    Rgb([1.0, 1.0, 1.0])
}

fn bits(z: Complex64) -> u128 {
    let b1 = z.re.to_bits();
    let b2 = z.im.to_bits();
    ((b1 as u128) << 64) | (b2 as u128)
}

fn smoothstep(x: f32) -> f32 {
    3.0 * x.powi(2) - 2.0 * x.powi(3)
}

fn steepstep(x: f32) -> f32 {
    let semi = |a: f32| (a * (1.0 - a)).sqrt();
    if x <= 0.5 {
        semi(x)
    } else {
        1.0 - semi(1.0 - x)
    }
}
