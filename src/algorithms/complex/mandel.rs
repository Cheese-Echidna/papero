use num::complex::{Complex64, ComplexFloat};
use crate::*;

#[derive(Default)]
pub(crate) struct Mandel;

impl Generator for Mandel {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = Rgb32FImage::new(args.width, args.height);
        let (width, height) = (args.width as f64, args.height as f64);

        for py in 0..args.height {
            for px in 0..args.width {
                let centre = Complex64::new(1.4 ,0.0);
                let mandel_width = 6.0;

                let (x,y) = ((px as f64 / width - 0.5), -((py as f64 / height - 0.5) * height / width));
                let c = Complex64::new(x, y).scale(mandel_width) + centre;

                let (dist_bound, iter_bound) = (2.0, 200);

                let esc = escape(c.recip(), dist_bound, iter_bound);
                let i = esc.1;

                let h = (i as f32 / iter_bound as f32).clamp(0.0, 1.0);
                let colour = colour_utils::convert_from_ok_hsl(h*0.8, 1.0, steepstep(h));
                image.put_pixel(px, py,colour);
            };
        };

        image.into()
    }


    fn name() -> &'static str {
        "Mandelbrot"
    }

}

fn escape(c: Complex64, escaped: f64, limit: u32) -> (Complex64, u32) {
    let mut num = 0_u32;
    let mut z = c;

    let escaped_sqr = escaped.powi(2);

    while z.norm_sqr() < escaped_sqr && num < limit {
        z = z.powu(2) + c;
        num += 1;
    }

    (c, num)
}

fn smoothstep(x: f32) -> f32 {
    3.0 * x.powi(2) - 2.0 * x.powi(3)
}

fn steepstep(x:f32) -> f32 {
    let semi = |a:f32| (a*(1.0-a)).sqrt();
    if x <= 0.5 {
        semi(x)
    } else {
        1.0 - semi(1.0 - x)
    }
}