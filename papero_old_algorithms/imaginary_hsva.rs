use num::PrimInt;
use crate::*;
use crate::util::hsva::{HSVA, HSVAImage};

// Code stolen from: https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set

const X_LIM_MIN: f64 = -2.;
const X_LIM_MAX: f64 = 0.47;

const Y_OFFSET: f64 = 0.0;
const Y_LIM_MIN: f64 = -(X_LIM_MAX - X_LIM_MIN + Y_OFFSET) / 2. * HEIGHT as f64 / WIDTH as f64;
const Y_LIM_MAX: f64 = (X_LIM_MAX - X_LIM_MIN - Y_OFFSET) / 2. * HEIGHT as f64 / WIDTH as f64;

pub(crate) struct Imaginary;

impl Plugin for Imaginary {
    fn create() -> Image {
        let mut image = HSVAImage::new(WIDTH, HEIGHT, HSVA::new(0., 0., 0., 0.));

        let x_inc = (X_LIM_MAX - X_LIM_MIN) / WIDTH as f64;
        let y_inc = (Y_LIM_MAX - Y_LIM_MIN) / HEIGHT as f64;

        let ln2 = 2f64.ln();

        let max_iteration = 1500;

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

                let smooth_iteration = iteration as f64 + 1.0 - ((x * x + y * y).sqrt().ln() / (2.0 as f64).sqrt().ln()).ln() / (2.0 as f64).sqrt().ln();
                let hue = 360.0 * smooth_iteration / max_iteration as f64;
                let value = if iteration < max_iteration {
                    100.0 * (1.0 / (1.0 + (-10.0 * (iteration as f64 / max_iteration as f64 - 0.1)).exp()))
                } else {
                    0.0
                };
                let c = HSVA::new(hue % 360.0, 100.0, value, 100.0);


                // let c = HSVA::new(((iteration as f64 / max_iteration as f64) * 360.).powf(1.5) % 360., 100., (iteration as f64 / max_iteration as f64).powf(0.5) * 100., 100.);
                // let c = HSVA::new(((iteration / max_iteration) as f64 * 360.).powf(1.5) % 360., 100., (iteration / max_iteration) as f64 * 100., 100.);
                // println!("{:?}", c);
                image.set_pixel(Px, Py, c).unwrap();
            }
        }
        image.to_rgb_image()
    }
}

/*fn OLD() {
    // Fist pass, iteration_counts[y][x] = iteration count
    let mut iteration_counts:Vec<Vec<u32>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
    let multiplier = 2;
    let max_iteration = 255/multiplier;
    let mut iteration = 0;

    let cutoff:f64 = 4.;

    for i in 0..WIDTH as usize {
        for j in 0..HEIGHT as usize {
            let x0 = X_LIM_MIN + i as f64 * x_inc;
            let y0 = Y_LIM_MIN + j as f64 * y_inc;

            let mut x = 0.;
            let mut y = 0.;

            let mut x2 = 0.;
            let mut y2 = 0.;


            while (x2+y2) <= cutoff && iteration < max_iteration {
                y= (x + x) * y + y0;
                x= x2 - y2 + x0;
                x2= x * x;
                y2= y * y;
                iteration += 1;
            }

            iteration_counts[j][i] = iteration;
        }
    }


    // Second pass, making histogram
    let mut iterations_per_pixel:Vec<u32> = vec![0; max_iteration as usize];
    for x in 0..WIDTH as usize {
        for y in 0..HEIGHT as usize {
            let i = iteration_counts[x][y];
            iterations_per_pixel[i] += 1;
        }
    }

    // Third pass
    let mut total = 0;
    for i in 0..max_iteration as usize {
        total += iterations_per_pixel[i];
    }


}*/