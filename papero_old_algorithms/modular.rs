// tau (t) element C, Im(t) > 0
// k element Z, k >= 2
// Sum (m,n) of (1/(m+nt)^2k)

use crate::*;
use num::Complex;
use rand::Rng;

const X_LIM_MIN: f64 = -2.;
const X_LIM_MAX: f64 = 2.;

// const Y_OFFSET: f64 = 0.0;
// const Y_LIM_MIN: f64 = -(X_LIM_MAX - X_LIM_MIN + Y_OFFSET) / 2. * HEIGHT as f64 / WIDTH as f64;
// const Y_LIM_MAX: f64 = (X_LIM_MAX - X_LIM_MIN - Y_OFFSET) / 2. * HEIGHT as f64 / WIDTH as f64;


pub(crate) struct Modular;

impl Plugin for Modular {
    fn create() -> Image {
        let mut image = Image::blank(WIDTH, HEIGHT);
        for px in 0..WIDTH {
            for py in 0..HEIGHT {
                let result = modular_set(Complex::new(px as f64 / WIDTH as f64 , py as f64 / HEIGHT as f64), 2);
                let c:Color = Color::rgb(result.re as u8 * 255, 0, result.im as u8 * 255);
                image.set_pixel(px, py, c).unwrap();
            }
        }

        image
    }
}

fn modular_set(t: Complex<f64>, k: i32) -> Complex<f64> {

    let mut sum:Complex<f64> = Complex::new(0., 0.);
    let mut m = 0;
    let mut n = 0;
    let mut k = 0;
    loop{
        k += 1;
        (m,n) = match k%4 {
            0 => (m+1, n),
            1 => (m, n+1),
            2 => (-m, -n),
            3 => (-m, -n),
            _ => (0,0)
        };

        if m==0 && n==0 {
            continue;
        }
        if m > 1000 || n > 1000 {
            return sum;
        }

        let denom = (m as f64 + n as f64 * t).powi(2 * k);
        if denom.norm() < 1e-6 {
            return sum;
        }
        sum += 1. / denom;
    }
    sum
}