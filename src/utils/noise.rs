use glam::f64::DVec2;
use glam::Vec2;
use noise::{MultiFractal, NoiseFn, OpenSimplex};
use crate::utils::image_manager::Args;

const SCALE_FACTOR: f64 = 150.;
const NOISE_FACTOR: f64 = 400.;


pub fn fbm(seed: u32, h: f64, n: usize, x: DVec2) -> f64 {
    noise::Fbm::<OpenSimplex>::new(seed).set_octaves(n).get((x * h).to_array())
    /*
    old (worse) method vvvv
    let max_min = (2.0_f64.powf(h) - 2.0_f64.powf(h - h * n as f64)) / (2.0_f64.powf(h) - 1.0);
    let max_min = (1. - 2.0_f64.powf(-h*n as f64)) / (h * 2.0_f64.ln());
    let x = x / NOISE_FACTOR;

    let g = f64::exp2(-h);
    let mut f = 1.0;
    let mut a = 1.0;
    let mut t = 0.0;

    let noise = noise::Simplex::new(seed);

    for _ in 0..n {
    t += a * noise.get((x * f).to_array());
    f *= 2.0;
    a *= g;
    }
    t
    */
}

// fn fbm_2d(seed1: u32, seed2: u32, h: f64, n: usize, x: DVec2) -> DVec2 {
//     Position {
//         x: fbm(seed1, h, n, x),
//         y: fbm(seed2, h, n, x),
//     }
// }


// fn domain_warp(position: &mut Vec2, seed_1: u32, seed_2:u32) {
//     let f = |x: Vec2| -> f32 {
//         let perlin = Perlin::new(seed_1);
//         perlin.get(crate::algorithms::shapes::shape::p(x / NOISE_FACTOR)) * SCALE_FACTOR
//     };
//     let g = |x: Vec2| -> f32 {
//         let perlin = Perlin::new(seed_2);
//         perlin.get(crate::algorithms::shapes::shape::p(x / NOISE_FACTOR)) * SCALE_FACTOR
//     };
//
//     let (x, y) = (position.x + f(position.clone()), position.y + g(position.clone()));
//     let new_position = Vec2 { x, y };
//     *position = new_position;
// }
//
// fn domain_warp_2(position: &mut Vec2, seed: u32, h: f32, n: usize) {
//     let f = |x: Vec2| -> f32 { fbm(seed, h, n, x) * SCALE_FACTOR};
//     let g = |x: Vec2| -> f32 { fbm(seed+1, h, n, x) * SCALE_FACTOR };
//
//     let (x, y) = (position.x + f(position.clone()), position.y + g(position.clone()));
//     let new_position = Vec2 { x, y };
//     *position = new_position;
// }
//
// fn render_fbm(args: &Args, seed: u32, h: f32, n: usize) -> RgbaImage {
//     let (width, height) = args.get_wh();
//
//     let mut image = RgbaImage::blank(width, height);
//
//     let mut min = f32::MAX;
//     let mut max = f32::MIN;
//
//     for y in 0..height {
//         for x in 0..width {
//             let p = Vec2 { x: x as f32, y: y as f32 } / 400.;
//             let q = Vec2 { x: fbm(seed, h, n, p), y: fbm(seed + 1, h, n, p) };
//             // let r = Vec2{x: fbm(seed+2, h, n, p + (q*4.0)), y: fbm(seed+3, h, n, p + (q*4.0))};
//
//             let v = fbm(seed + 4, h, n, p + (q * 4.0)) / 4.0;
//             let v = v/5.0 + 0.6;
//             // ang element [0.5, 0.7]
//
//             let c = Hsva::new(v + 1.0, 1.0, 1.0, 1.0).to::<Rgba>();
//             image.set_pixel(x, y, c);
//         }
//     }
//
//     // n = 1 => 1.0
//     // n = 2 => 1.7
//     // n = 3 => 2
//     // n = 5 => 2.3
//     // n = 10 => 2.5
//     // n = 20 => 2.5
//     dbg!(min);
//     dbg!(max);
//
//     image
// }
//
// // http://thingonitsown.blogspot.com/2019/01/general-domain-warping.html
// // height(x,y,strength,size) = noise(x,y,strength*noise(size*x,size*y))
// fn render_double_perlin(args: &Args, seed: u32) -> RgbaImage {
//     let (width, height) = args.get_wh();
//
//     let noise = Perlin::new(seed);
//     let noise2 = Perlin::new(seed + 1);
//
//     let mut image = RgbaImage::blank(width, height);
//
//     for frame in 0..30 {
//         for y in 0..height {
//             for x in 0..width {
//                 // let strength = (1.33_f32).powi(4);
//                 let size = 1. / 400.;
//                 let z = noise.get([x as f32 * size, y as f32 * size]);
//                 let height = noise2.get([x as f32 * size, y as f32 * size, z + (frame as f32 / 10.)]);
//
//                 // dbg!(z);
//                 // dbg!(height);
//
//                 let colour = Hsva::new(height, 0.8, 0.8, 1.).to::<Rgba>();
//                 image.set_pixel(x, y, colour);
//             }
//         }
//         image.save(&format!("out/gif/{}.png", frame));
//         println!("Finished with frame {}", frame);
//     }
//
//     image
// }
