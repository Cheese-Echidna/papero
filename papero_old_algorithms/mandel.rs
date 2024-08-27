use num::complex::Complex64;
use crate::*;

pub(crate) fn name() -> String {
    "Mandelbrot".to_string()
}

pub(crate) fn create(p0: Config) -> RgbaImage {
    let (width, height) = (p0.width, p0.height);
    let mut image = Hsva01Image::blank(width, height);
    let mut rng = rand::thread_rng();

    for y in 0..height {
        for x in 0..width {
            let c = Complex64::new(x as f64/width as f64 - 0.5, (y as f64/height as f64 - 0.5)*height as f64/width as f64);
            let c = c.scale(4.0);
            // let esc = escape(c, 10.0_f64.powi(3), 100);
            let esc = c;
            let (h, _s, _v) = (esc.norm(), esc.re, esc.im);
            let colour = Hsva01::new(h, 1.0, 1.0,1.0);
            image.set_pixel(x,y,colour);
        };
    };

    image.to_rgba_image()
}

fn escape(c: Complex64, escaped:f64, limit:u32) -> Complex64 {
    let mut mag:f64 = 0.0;
    let mut num = 0_u32;
    let mut c = c;
    while mag < escaped && num < limit {
        c = c.powu(2) + c;
        num += 1;
    };
    c
}