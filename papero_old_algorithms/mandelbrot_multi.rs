use std::thread::JoinHandle;
use crate::*;

// Time to beat: ~88 per image in 69 image test
// With i_max=30

const X_LIM_MIN: f64 = -2.;
const X_LIM_MAX: f64 = 0.74;

const Y_OFFSET: f64 = 0.0;
const Y_LIM_MIN: f64 = -(X_LIM_MAX - X_LIM_MIN + Y_OFFSET) / 2. * HEIGHT as f64 / WIDTH as f64;
const Y_LIM_MAX: f64 = (X_LIM_MAX - X_LIM_MIN - Y_OFFSET) / 2. * HEIGHT as f64 / WIDTH as f64;

pub(crate) struct Mandelbrot;

impl Plugin for Mandelbrot {
    fn create() -> Image {
        let mut image = Image::blank(WIDTH, HEIGHT);

        let x_inc = (X_LIM_MAX - X_LIM_MIN) / WIDTH as f64;
        let y_inc = (Y_LIM_MAX - Y_LIM_MIN) / HEIGHT as f64;

        let ln2 = 2f64.ln();
        let max_iteration = 255;
        let bailout = 4f64;
        let mut handles = vec![];

        let i_max:i32 = 30;
        for i in 0..i_max {
            let handle: JoinHandle<Vec<((i32, i32), u32)>> = thread::spawn(move ||{
                let mut v:Vec<((i32, i32), u32)> = vec![];
                let start_y = (i*HEIGHT)/i_max;
                let end_y = ((i+1)*HEIGHT)/i_max;
                for py in start_y..end_y {
                    for px in 0..WIDTH {
                        v.push(((px, py), mandelbrot(px, py, x_inc, y_inc, ln2, max_iteration, bailout)));
                    }
                }
                v
            });

            handles.push(handle);

        }


        for handle in handles {
            let v = handle.join().unwrap();
            for ((px, py), iteration) in v {
                image.set_pixel(px, py, iteration_to_colour(iteration, max_iteration)).unwrap();
            }
        }

        image
    }
}

fn mandelbrot(px:i32, py:i32, x_inc:f64, y_inc:f64, ln2:f64, max_iteration:u32, bailout:f64) -> u32 {
    let x0 = X_LIM_MIN + px as f64 * x_inc;
    let y0 = Y_LIM_MIN + py as f64 * y_inc;

    let mut x:f64 = 0.;
    let mut y:f64 = 0.;

    let mut iteration = 0;

    while x.powi(2) + y.powi(2) <= bailout && iteration < max_iteration {
        let x_temp:f64 = x.powi(2) - y.powi(2) + x0;
        y = (x + x) * y + y0;
        x = x_temp;
        iteration += 1;
    }

    // if iteration < max_iteration {
    //     // sqrt of inner term removed using log simplification rules.
    //     let log_zn:f64 = ((x.powi(2) + y.powi(2)) as f64).ln() / 2.;
    //     let nu = (log_zn / ln2).ln() / ln2;
    //     // Rearranging the potential function.
    //     // Dividing log_zn by log(2) instead of log(N = 1<<8)
    //     // because we want the entire palette to range from the
    //     // center to radius 2, NOT our bailout radius.
    //     iteration = iteration + 1 - nu as u32;
    // }

    iteration
}

fn iteration_to_colour(iteration: u32, max_iteration: u32) -> Color {
    let k = 255-((iteration as f64 / max_iteration as f64).sqrt() * 255.) as u8;

    Color::rgba( k, k, k, 255)
}