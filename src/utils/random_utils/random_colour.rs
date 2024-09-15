use crate::*;

pub fn random_rgb() -> Rgba<u8> {
    let mut rng = rand::thread_rng();

    let red = rng.gen_range(0..=255);
    let green = rng.gen_range(0..=255);
    let blue = rng.gen_range(0..=255);

    Rgba([red, green, blue, 255])
}


pub fn random_rgb_f32() -> Rgba<f32> {
    let mut rng = rand::thread_rng();

    let mut rand = || rng.gen_range((0.0)..(1.0));

    Rgba([rand(), rand(), rand(), 1.0])
}

pub fn random_ok_rgb_f32() -> Rgb<f32> {
    let mut rng = rand::thread_rng();

    let mut rand = || rng.gen_range((0.0)..(1.0));

    colour_utils::convert_from_ok_hsl(rand(), rand(), rand())
}

pub fn random_pretty_ok() -> Rgb<f32> {
    let mut rng = rand::thread_rng();

    colour_utils::convert_from_ok_hsl(rng.gen_range((0.0)..(1.0)), rng.gen_range((0.2)..(0.8)), rng.gen_range((0.2)..(0.8)))
}