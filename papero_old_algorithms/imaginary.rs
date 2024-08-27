use crate::*;
use crate::util::aa::downscale;

const X_LIM_MIN: f64 = -2.;
const X_LIM_MAX: f64 = 0.74;

const Y_OFFSET: f64 = 0.0;
const Y_LIM_MIN: f64 = -(X_LIM_MAX - X_LIM_MIN + Y_OFFSET) / 2. * HEIGHT as f64 / WIDTH as f64;
const Y_LIM_MAX: f64 = (X_LIM_MAX - X_LIM_MIN - Y_OFFSET) / 2. * HEIGHT as f64 / WIDTH as f64;

pub(crate) struct Imaginary;

impl Plugin for Imaginary {
    fn create() -> Image {
        let mut image = Image::blank(WIDTH, HEIGHT);

        let x_inc = (X_LIM_MAX - X_LIM_MIN) / WIDTH as f64;
        let y_inc = (Y_LIM_MAX - Y_LIM_MIN) / HEIGHT as f64;

        let ln2 = 2f64.ln();

        let max_iteration = 255;

        let bailout = 4f64;

        for Py in 0..HEIGHT {
            for Px in 0..WIDTH {
                let x0 = X_LIM_MIN + Px as f64 * x_inc;
                let y0 = Y_LIM_MIN + Py as f64 * y_inc;

                let mut x:f64 = 0.;
                let mut y:f64 = 0.;

                let mut iteration = 0;

                while x.powi(2) + y.powi(2) <= bailout && iteration < max_iteration {
                    let x_temp:f64 = x.powi(2) - y.powi(2) + x0;
                    y = (x + x) * y + y0;
                    x = x_temp;
                    iteration += 1;
                }

                if iteration < max_iteration {
                    // sqrt of inner term removed using log simplification rules.
                    let log_zn:f64 = ((x.powi(2) + y.powi(2)) as f64).ln() / 2.;
                    let nu = (log_zn / ln2).ln() / ln2;
                    // Rearranging the potential function.
                    // Dividing log_zn by log(2) instead of log(N = 1<<8)
                    // because we want the entire palette to range from the
                    // center to radius 2, NOT our bailout radius.
                    iteration = iteration + 1 - nu as u32;
                }

                let k = 255-((iteration as f64 / max_iteration as f64).sqrt() * 255.) as u8;

                let c = Color::rgba( k, k, k, 255);
                image.set_pixel(Px, Py, c).unwrap();
            }
        }
        image
    }
}

// fn smoothstep(x: f64) -> f64{
//     if x<=0. {
//         return 0.;
//     }
//     if x>=1. {
//         return  1.;
//     }
//     return x * x * (3. - 2. * x)
// }
//
// fn smootherstep(x: f64) -> f64 {
//     if x<=0. {
//         return 0.;
//     }
//     if x>=1. {
//         return 1.;
//     }
//     return x * x * x * (10. + x * (6. * x - 15.))
// }

