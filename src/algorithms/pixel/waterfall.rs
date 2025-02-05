use std::f32::consts::{E, PI};
use crate::*;

#[derive(Default)]
pub(crate) struct Waterfall;

// todo: this is yet to work

impl Generator for Waterfall {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = Rgb32FImage::new(args.width, args.height);

        for y in 0..args.height {
            for x in 0..args.width {
                let x_prop = (x as f32) / (args.width as f32) * PI;

                let color = if y == 0 {
                    Rgb::<f32>([x_prop.sin().abs(), x_prop.cos().abs(), x_prop.tan().abs().clamp(0.0, 1.0)])
                } else {
                    Self::iteration(&image, y, x, args.width)
                };
                image.put_pixel(x, y, color);
            }
        }
        image.into()
    }

    fn name() -> &'static str {
        "Waterfall"
    }
}

impl Waterfall {
    fn iteration(image: &Rgb32FImage, y: u32, x: u32, width: u32) -> Rgb<f32> {
        let x = x as i32;
        let width = width as i32;
        let mut output: [Vec<f32>; 3] = Default::default();
        ((-1_i32)..=1)
            .map(|i| (x - 1).rem_euclid(width)) // rem_euclid means it doesn't stay negative
            .for_each(|new_x| {
                let (px, py) = (new_x as u32, y-1);
                let colour = image.get_pixel(px, py).0.iter().enumerate().for_each(|(i, value)| {
                    output[i].push(*value);
                });
            });

        let (r,g,b) = (&output[0], &output[1], &output[2]);


        // The integral of these functions needs to be ~= 0.5 so that the drift is negligible.
        let red = (r[0] * 0.5 + r[1] * 0.3 + r[2] * 0.2).sin().abs().clamp(0.0, 1.0);
        let green = (g[0] * 0.5 + g[1] * 0.3 + g[2] * 0.2).cos().abs().clamp(0.0, 1.0);
        let blue = ((b[0] + b[1] + b[2]) / 5.0).sqrt().abs().clamp(0.0, 1.0);
        Rgb::<f32>([red, green, blue])
    }
}