use glam::{U8Vec3, Vec3};
use crate::utils::colour_utils::{convert_from_ok_hsl, ColourF3, ColourU3, ImageColour};
use crate::*;
use palette::named::{BLACK, WHITE};

type U24 = u32;

#[derive(Default)]
pub(crate) struct Pinski {}

impl Generator for Pinski {
    fn generate(args: &Args) -> DynamicImage {
        let black = Rgb::<u8>::from_const(BLACK);
        let white = Rgb::<u8>::from_const(WHITE);
        let mut image = args.image_u8(black);

        image.put_pixel(args.width / 2 - 1, 0, Rgb([1, 3, 5]));
        let m = U8Vec3::new(16, 16, 16);

        for row in 1..args.height {
            for mid in 0..args.width {
                let right = mid + 1;
                let left = mid - 1;
                let left_v = image.get_pixel_checked(left, row - 1).unwrap_or(&black);
                let right_v = image.get_pixel_checked(right, row - 1).unwrap_or(&black);
                let sum = left_v.to_vec3() + right_v.to_vec3();
                let c = sum % m;

                image.put_pixel(mid, row, Rgb::<u8>::from_vec3(c))
            }
        }
        image.pixels_mut().for_each(|x| {
            *x = Rgb::<u8>::from_vec3(x.to_vec3() * (U8Vec3::splat(255) / m));
        });
        image.into()
    }

    fn name() -> &'static str {
        "Sierpiński's Triangle"
    }
}

// pub fn create2(config: Config) -> RgbaImage {
//     let mut image = Hsva01Image::new(config.width, config.height, Hsva01::new(0.0,1.0,1.0,1.0));
//     image.set_pixel(config.width/2-1, 0, Hsva01::new(0.5,1.0,1.0,1.0));
//     for row in 1..image.height {
//         for mid in 0..image.width {
//             let right = mid + 1;
//             let left = mid - 1;
//             let left_v = image.get_pixel(left, row-1).unwrap_or(Hsva01::new(0.0,1.0,0.0,1.0)).h;
//             let right_v = image.get_pixel(right, row-1).unwrap_or(Hsva01::new(0.0,1.0,0.0,1.0)).h;
//             let h_new = angle_mean_01(left_v, right_v);
//             // let c = if sum == 1 {
//             //     Rgb::<u8>::white()
//             // } else {
//             //     Rgb::<u8>::black()
//             // };
//
//             // println!("({}, {}):  {} + {} = {}", mid, row, left_v, right_v, sum);
//             let c = Hsva01::new(h_new, 1.0, 1.0, 1.0);
//             image.set_pixel(mid,row,c).unwrap()
//         }
//     }
//     return image.to_rgba_image()
// }

fn u24_from_colour(c: &Rgb<u8>) -> U24 {
    let r = c.0[0] as u32;
    let g = c.0[1] as u32;
    let b = c.0[2] as u32;
    (r << (2 * 8)) + (g << 8) + b
}

// fn colour_from_u24(c: U24) -> Rgb<u8> {
//     let r = c >> 16;
//     let g = (c >> 8) & 0b1111_1111;
//     let b = c & 0b1111_1111;
//     Rgb::<u8>::from([r, g, b].map(|x| x as u8))
// }
