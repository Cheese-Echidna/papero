use image::Rgba;
use rand::Rng;

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