// tau (t) element C, Im(t) > 0
// k element Z, k >= 2
// Sum (m,n) of (1/(m+nt)^2k)

use crate::*;
use num::Complex;
use palette::named::BLACK;
use crate::utils::colour_utils::ImageColour;

const X_LIM_MIN: f64 = -2.;
const X_LIM_MAX: f64 = 2.;

#[derive(Default)]
pub(crate) struct Modular;

impl Generator for Modular {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = args.image_f32(Rgb::from_const(BLACK));
        for px in 0..args.width {
            for py in 0..args.height {
                let result = modular_set(Complex::new(px as f64 / args.width as f64 , py as f64 / args.height as f64), 2);
                let c = Rgb([result.re as f32, 0., result.im as f32]);
                image.put_pixel(px, py, c);
            }
        }

        image.into()
    }

    fn name() -> &'static str {
        "Complex Modular Set Generator"
    }
}

fn modular_set(t: Complex<f64>, k: i32) -> Complex<f64> {
    let mut sum:Complex<f64> = Complex::new(0., 0.);
    let mut m = 0;
    let mut n = 0;
    let mut k = k;
    for _i in 0..100 {
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