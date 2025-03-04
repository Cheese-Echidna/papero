use std::f32::consts::{PI, TAU};
use crate::*;
use num::complex::{Complex64, ComplexFloat};
use crate::utils::num_utils::lerp;

#[derive(Default)]
pub(crate) struct Julia;

impl Generator for Julia {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = Rgb32FImage::new(args.width, args.height);
        let (width, height) = (args.width as f64, args.height as f64);

        for py in 0..args.height {
            for px in 0..args.width {
                // This makes it so that the centre of the image is (0, 0)
                // And up in the image is +y
                let (x, y) = (
                    (px as f64 / width - 0.5),
                    -((py as f64 / height - 0.5) * height / width),
                );
                // Adjust the julia starting point for different values
                let c = Complex64::new(0.355, 0.355);

                // Centre of the complex plane that we're focusing on
                let centre = Complex64::new(0.0, -0.4);

                // Zoom level: Bigger = number bigger
                let zoom = 1.25;

                let z = Complex64::new(x, y) / zoom + centre;

                // Changes the "resolution" of the julia set iteration
                let max = 320;

                // How long does it take for the point to diverge
                // 0 = straight away; 1 = it never does
                let i = quadratic_iteration(c, z, max) as f32 / max as f32;

                // let h_start = 300.0 / 360.0;
                // let h_end = 190.0 / 360.0;
                // let hue = (h_start + i * (h_end - h_start));
                // Above and below are equivalent
                let hue = lerp(i, 310.0, 190.0) / 360.0;

                // let s_start = 0.8;
                // let s_end = 1.0;
                // let saturation = s_start + i * (s_end - s_start);
                let saturation = lerp(i, 0.8, 1.0);

                // let l_start = 0.3;
                // let l_end = 0.8;
                // let lightness = l_start + i * (l_end - l_start);

                let lightness = lerp(i, 0.3, 0.8);

                let colour = colour_utils::convert_from_ok_hsl(hue, saturation, lightness);

                image.put_pixel(px, py, colour);
            }
        }
        image.into()
    }
    fn name() -> &'static str {
        "Julia (Jazzmine)"
    }
}

fn quadratic_iteration(c: Complex64, mut z: Complex64, max: u32) -> u32 {
    for i in 0..=max {
        if z.abs() > 2.0 {
            return i;
        }
        z = z.powu(2) + c;
    }
    max
}
