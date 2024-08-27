use std::f32::consts::*;
use palette::{IntoColor, Clamp};
use image::{Rgb32FImage, RgbaImage, ImageResult};
use noise::{OpenSimplex, NoiseFn, Fbm, MultiFractal};

const WIDTH:usize = 1000;
const WIDTH_F32:f32 = WIDTH as f32;
const WIDTH_U32:u32 = WIDTH as u32;
const HEIGHT:usize = 1000;
const HEIGHT_F32:f32 = HEIGHT as f32;
const HEIGHT_U32:u32 = HEIGHT as u32;

type ImageColour = image::Rgb<f32>;
type Colour = palette::LinSrgb<f32>;

fn main() {
    let mut image = Rgb32FImage::new(WIDTH_U32, HEIGHT_U32);

    let scale_factor = 2.0;
    let noise: Fbm<OpenSimplex> = Fbm::new(0).set_octaves(5);

    for py in 0..HEIGHT_U32 {
        let y = py as f32 / HEIGHT_F32 - 0.5;
        for px in 0..WIDTH_U32 {
            let x = px as f32 / WIDTH_F32 - 0.5;

            let (a,b,c,d) = (cos(x*TAU), sin(x*TAU), cos(y*TAU), sin(y*TAU));

            let v1 = noise.get([a as f64 * scale_factor, b as f64 * scale_factor, c as f64 * scale_factor, d as f64 * scale_factor]) as f32 + 0.5;
            let v2 = noise.get([a as f64 * scale_factor + 10000.2442, b as f64 * scale_factor + 10000.5241, c as f64 * scale_factor + 10000.121343, d as f64 * scale_factor + 10000.4242]) as f32 + 0.5;

            let v1 = multistep(v1, 5);
            let v2 = multistep(v2, 5);

            let c1 = palette::Srgb::new(0.6_f32, 0.6_f32, 0.45_f32) * v1;
            let c2 = palette::Srgb::new(0.24_f32, 0.23_f32, 0.21_f32) * v2;

            let colour = c1 + c2;

            *image.get_pixel_mut(px,py) = convert_palette_to_image_rgba(colour.into());
        }
    }

    save_image(image, "output.png").unwrap();

}

fn steep_sigmoid(x: f32) -> f32 { 1.0 / (1.0 + E.powf((-20.0*x) + 10.0)) }

// fn camo() {
// let (t1, t2) = (cos(t), sin(t));
//     let (a,b,c,d) = (cos(x*TAU), sin(x*TAU), cos(y*TAU), sin(y*TAU));
//
//     let v1 = OpenSimplex::new(0).get([a as f64 * scale_factor, b as f64 * scale_factor, c as f64 * scale_factor, d as f64 * scale_factor]) as f32 + 0.5;
//     let v2 = OpenSimplex::new(0).get([a as f64 * scale_factor + 10000.2442, b as f64 * scale_factor + 10000.5241, c as f64 * scale_factor + 10000.121343, d as f64 * scale_factor + 10000.4242]) as f32 + 0.5;
//
//     let v1 = multistep(v1, 4);
//     let v2 = multistep(v2, 4);
//
//     let c1 = palette::Srgb::new(0.81_f32, 0.56_f32, 0.43_f32) * v1 * 0.5;
//     let c2 = palette::Srgb::new(0.34_f32, 0.67_f32, 0.97_f32) * v2 * 0.5;
//
//     let colour = c1 + c2;
//
//     *image.get_pixel_mut(px,py) = convert_palette_to_image_rgba(colour.into());
// }


// AKA the staircase function
fn multistep(x: f32, num: u32) -> f32 {
    // don't ask
    let x = x.clamp(0.0, 0.99999994);
    let n = num as f32;
    (1.0/(n - 1.0)) * (x * n).floor()
}

fn step(x: f32, bound: f32) -> f32 {
    return if x < bound {
        0.0
    } else {
        1.0
    }
}

fn save_image(image: Rgb32FImage, path: &str) -> ImageResult<()> {
    let mut image2 = RgbaImage::new(WIDTH_U32, HEIGHT_U32);
    image.enumerate_pixels().for_each(|(x, y, colour) | {
        let new_colour:[u8;4] = {
            let mut c = colour.0.map(|v| (v * 255.) as u8).to_vec();
            c.push(255);
            c.try_into().unwrap()
        };
        *image2.get_pixel_mut(x,y) = image::Rgba::from(new_colour);
    });

    image2.save(path)
}


fn convert_palette_to_image_rgba(c: Colour) -> ImageColour {
    let c2: palette::Srgb<f32> = c.clamp().into_color();
    ImageColour::from([c2.red, c2.green, c2.blue])
}

fn cos(x: f32) -> f32 {
    x.cos()
}

fn sin(x: f32) -> f32 {
    x.sin()
}