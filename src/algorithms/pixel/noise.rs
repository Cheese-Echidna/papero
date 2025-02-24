use crate::utils::colour_utils::ImageColour;
use crate::utils::image_manager::Args;
use crate::Generator;
use glam::Vec3;
use image::{DynamicImage, Rgb};
use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex};
use palette::named::BLACK;
use palette::Clamp;
use std::f32::consts::*;

#[derive(Default)]
pub(crate) struct NoiseRender;

impl Generator for NoiseRender {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = args.image_f32(Rgb::from_const(BLACK));

        let (width, height) = args.wh();

        let scale_factor = 2.0;
        let noise: Fbm<OpenSimplex> = Fbm::new(0).set_octaves(5);

        for py in 0..height {
            let y = py as f32 / (height as f32) - 0.5;
            for px in 0..width {
                let x = px as f32 / (width as f32) - 0.5;

                let (a, b, c, d) = (cos(x * TAU), sin(x * TAU), cos(y * TAU), sin(y * TAU));

                let v1 = noise.get([
                    a as f64 * scale_factor,
                    b as f64 * scale_factor,
                    c as f64 * scale_factor,
                    d as f64 * scale_factor,
                ]) as f32
                    + 0.5;
                let v2 = noise.get([
                    a as f64 * scale_factor + 10000.2442,
                    b as f64 * scale_factor + 10000.5241,
                    c as f64 * scale_factor + 10000.121343,
                    d as f64 * scale_factor + 10000.4242,
                ]) as f32
                    + 0.5;

                let v1 = multistep(v1, 5);
                let v2 = multistep(v2, 5);

                let c1 = Vec3::new(0.81_f32, 0.56_f32, 0.43_f32) * v1 * 0.5;
                // let c1 = Vec3::new(1.0, 0.0_f32, 0.0_f32) * v1 * 0.5;
                let c2 = Vec3::new(0.22_f32, 0.28_f32, 0.17_f32) * v2;
                // let c2 = Vec3::new(0.0_f32, 1.0_f32, 1.0_f32) * v2 * 0.5;
                // rgb(57, 67, 43)

                let colour = c1 + c2;
                // let colour = c1 * 2.0;

                image.put_pixel(px, py, Rgb(colour.to_array()));
            }
        }

        image.into()
    }

    fn name() -> &'static str {
        "Noisy renderer"
    }
}

fn steep_sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + E.powf((-20.0 * x) + 10.0))
}

// AKA the staircase function
fn multistep(x: f32, num: u32) -> f32 {
    // don't ask
    let x = x.clamp(0.0, 0.99999994);
    let n = num as f32;
    (1.0 / (n - 1.0)) * (x * n).floor()
}

fn step(x: f32, bound: f32) -> f32 {
    if x < bound {
        0.0
    } else {
        1.0
    }
}

fn cos(x: f32) -> f32 {
    x.cos()
}

fn sin(x: f32) -> f32 {
    x.sin()
}
