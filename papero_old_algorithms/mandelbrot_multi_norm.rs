use std::thread::JoinHandle;
use crate::*;
use crate::util::aa::downscale;
use crate::util::hsva::HSVA;

const CENTRE: (f64, f64) = (-0.4, 0.65);
const Z_WIDTH: f64 = 1.0/20.0;

const PI:f64 = std::f64::consts::PI;

pub(crate) struct Mandelbrot;

impl Plugin for Mandelbrot {
    fn create() -> Image {
        let mut image = Image::blank(WIDTH, HEIGHT);

        let x_lim_min = CENTRE.0 - Z_WIDTH / 2.;
        let x_lim_max = CENTRE.0 + Z_WIDTH / 2.;
        let y_lim_min = CENTRE.1 - Z_WIDTH / 2. * HEIGHT as f64 / WIDTH as f64;
        let y_lim_max = CENTRE.1 + Z_WIDTH / 2. * HEIGHT as f64 / WIDTH as f64;

        let x_inc = (x_lim_max - x_lim_min) / WIDTH as f64;
        let y_inc = (y_lim_max - y_lim_min) / HEIGHT as f64;

        let ln2 = 2f64.ln();
        let max_iteration = 360;
        let bailout = 4f64;
        let mut handles = vec![];

        let i_max:i32 = 30;
        for i in 0..i_max {
            let handle: JoinHandle<Vec<((i32, i32), f64)>> = thread::spawn(move ||{
                let mut v:Vec<((i32, i32), f64)> = vec![];
                let start_y = (i*HEIGHT)/i_max;
                let end_y = ((i+1)*HEIGHT)/i_max;
                for py in start_y..end_y {
                    for px in 0..WIDTH {
                        v.push(((px, py), mandelbrot(px as f64, py as f64, x_inc, y_inc, ln2, max_iteration, bailout, x_lim_min, y_lim_min)));
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
                // iter_map[py as usize][px as usize] = iteration as i32;
            }
        }

        // for i in 0..HEIGHT {
        //     for j in 0..WIDTH {
        //         print!("{}", iter_map[i as usize][j as usize]);
        //         print!(" ");
        //     }
        //     println!();
        // }


        downscale(image, MULTIPLIER)
    }
}

fn mandelbrot(px:f64, py:f64, x_inc:f64, y_inc:f64, ln2:f64, max_iteration:u32, bailout:f64, x_lim_min:f64, y_lim_min:f64) -> f64 {
    let x0 = x_lim_min + px * x_inc;
    let y0 = x_lim_min + py * y_inc;

    let mut x:f64 = 0.;
    let mut y:f64 = 0.;

    let mut iteration = 0;

    while x.powi(2) + y.powi(2) <= bailout && iteration < max_iteration {
        let x_temp:f64 = x.powi(2) - y.powi(2) + x0;
        y = (x + x) * y + y0;
        x = x_temp;
        iteration += 1;
    }

    let d = (x.powi(2) + y.powi(2)).sqrt();
    let smooth_iter = iteration as f64 - max_f64(1.0, d.log2()).log2();

    smooth_iter
}

#[allow(non_snake_case)]
fn iteration_to_colour(i: f64, i_max: u32) -> Color {

    HSVA::new(i, 100.0, 100.0, 100.0).to_rgb()
}

fn iteration_to_colour2(i: f64, i_max: u32) -> Color {
    let S = 1.0;
    let N = 360.;

    let h = ((i / i_max as f64).powf(S) * N).powf(1.5) % N ;
    let s = 100.0;
    let v = 100.0;
    let a = 100.0;
    // let k = 255-((i as f64 / i_max as f64).sqrt() * 255.) as u8;

    HSVA::new(h, s, v, a).to_rgb()
}

fn iteration_to_bw(i: f64, i_max: u32) -> Color {
    let i_max = i_max as f64;
    let r = (i / i_max * 255.) as u8;
    let g = r;
    let b = r;

    Color::rgb(r, g, b)
}

fn max_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

fn get_rand(max:f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..max)
}