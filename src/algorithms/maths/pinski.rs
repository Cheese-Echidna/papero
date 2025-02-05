use palette::named::{BLACK, WHITE};
use crate::*;
use crate::utils::colour_utils::ImageColour;

type U24 = u32;


#[derive(Default)]
pub(crate) struct Pinski {}

impl Generator for Pinski {
    fn generate(args: &Args) -> DynamicImage {
        let black = Rgb::<u8>::from_const(BLACK);
        let white = Rgb::<u8>::from_const(WHITE);
        let mut image = args.image_u8(black);


        // for i in 0..(args.width/3) {
        //     image.put_pixel(i*3,0,white);
        // }

        image.put_pixel(args.width/2-1, 0, white);

        for row in 1..args.height {
            for mid in 0..args.width {
                let right = mid + 1;
                let left = mid - 1;
                let left_v = u24_from_colour(&image.get_pixel_checked(left, row-1).unwrap_or(&black));
                let right_v = u24_from_colour(&image.get_pixel_checked(right, row-1).unwrap_or(&black));
                let sum = left_v + right_v;

                let c = if sum%2 == 1 {
                    white
                } else {
                    black
                };

                // let c = colour_from_u24(sum);
                image.put_pixel(mid,row,c)
            }
        }

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
    (r << 2*8) + (g << 1*8) + b
}

fn colour_from_u24(x:U24) -> Rgb<u8> {
    let r = ((x & 0xff0000) >> 2*8) as u8;
    let g = ((x & 0x00ff00) >> 1*8) as u8;
    let b = (x & 0x0000ff) as u8;

    return Rgb::<u8>::from([r, g, b]);
}

fn angle_mean_01(a:f32, b:f32) -> f32 {
    let x = zero_one_to_rad(a).cos() + zero_one_to_rad(b).cos() / 2.0;
    let y = zero_one_to_rad(a).sin() + zero_one_to_rad(b).sin() / 2.0;
    let ang = y.atan2(x);
    return rad_to_01(ang)
}

fn rad_to_01(rad:f32) -> f32 {
    return rad / (2.0 * std::f32::consts::PI)
}

fn zero_one_to_rad(zero_one:f32) -> f32 {
    return zero_one * (2.0 * std::f32::consts::PI)
}

