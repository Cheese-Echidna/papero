use crate::utils::colour_utils::ColourU3;
use crate::*;
use glam::U8Vec3;

type U24 = u32;

#[derive(Default)]
pub(crate) struct Pinski {}

impl Generator for Pinski {
    fn generate(args: &Args) -> DynamicImage {
        Self::real_generate(args, U8Vec3::ZERO, U8Vec3::ONE, U8Vec3::new(2, 2, 2))
    }

    fn name() -> &'static str {
        "Sierpiński's Triangle"
    }
}

impl Pinski {
    pub fn real_generate(args: &Args, bg: U8Vec3, starting_colour: U8Vec3, modulus: U8Vec3) -> DynamicImage {
        let mut image = args.image_u8(Rgb::<u8>::from_vec3(bg));

        image.put_pixel(args.width / 2 - 1, 0, Rgb::<u8>::from_vec3(starting_colour));

        for row in 1..args.height {
            for mid in 0..args.width {
                let right = mid + 1;
                let left = mid - 1;
                let lr = [(left, row - 1), (right, row - 1)].map(|(x, y)| image.get_pixel_checked(x, y).map(|x| x.to_vec3()).unwrap_or(U8Vec3::ZERO));
                let sum = lr[0] + lr[1];
                let c = sum % modulus;

                image.put_pixel(mid, row, Rgb::<u8>::from_vec3(c))
            }
        }
        image.pixels_mut().for_each(|x| {
            *x = Rgb::<u8>::from_vec3(x.to_vec3() * (U8Vec3::splat(255) / modulus));
        });
        image.into()
    }
}
