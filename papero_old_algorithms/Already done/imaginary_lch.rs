use crate::*;

// const X_LIM_MIN: f64 = -0.65;
// const X_LIM_MAX: f64 = -0.15;
//
// const Y_OFFSET: f64 = 0.65;
// const Y_LIM_MIN: f64 = -(X_LIM_MAX - X_LIM_MIN) / 2. * HEIGHT as f64 / WIDTH as f64 - Y_OFFSET;
// const Y_LIM_MAX: f64 = (X_LIM_MAX - X_LIM_MIN) / 2. * HEIGHT as f64 / WIDTH as f64 - Y_OFFSET;


const CENTRE: (f64, f64) = (-0.4, 0.65);
const Z_WIDTH: f64 = 1/200.0;

const PI:f64 = std::f64::consts::PI;

pub(crate) struct Imaginary;

impl Plugin for Imaginary {
    fn create() -> Image {
        gen(Z_WIDTH)
    }
}

pub fn gen(z_width: f64) -> Image {
    let x_lim_min = CENTRE.0 - z_width / 2.;
    let x_lim_max = CENTRE.0 + z_width / 2.;
    let y_lim_min = CENTRE.1 - z_width / 2. * HEIGHT as f64 / WIDTH as f64;
    let y_lim_max = CENTRE.1 + z_width / 2. * HEIGHT as f64 / WIDTH as f64;


    let mut image = Image::blank(WIDTH, HEIGHT);

    let x_inc = (x_lim_max - x_lim_min) / WIDTH as f64;
    let y_inc = (y_lim_max - y_lim_min) / HEIGHT as f64;

    let ln2 = 2f64.ln();

    let max_iteration = 255;

    let bailout = 4f64;

    for Py in 0..HEIGHT {
        for Px in 0..WIDTH {
            let x0 = x_lim_min + Px as f64 * x_inc;
            let y0 = y_lim_min + Py as f64 * y_inc;

            let mut x: f64 = 0.;
            let mut y: f64 = 0.;

            let mut iteration = 0;

            while x.powi(2) + y.powi(2) <= bailout && iteration < max_iteration {
                let x_temp: f64 = x.powi(2) - y.powi(2) + x0;
                y = (x + x) * y + y0;
                x = x_temp;
                iteration += 1;
            }
            let s: f64 = iteration as f64 / max_iteration as f64;
            let v:f64 = 1.0 - f64::cos(PI * s).powf(2.0);
            let lch: [f64; 3] = [75. - (75. * v), 28. + (75. - (75. * v)),(360. * s).powf(1.5) % 360.];

            let c = util::lch::rbg_from_lch(lch);
            image.set_pixel(Px, Py, c).unwrap();
        }
    }
    if MULT == 1 {
        return image
    }
    util::aa::downscale(image, MULT)
}


fn smoothstep(x: f64) -> f64{
    if x<=0. {
        return 0.;
    }
    if x>=1. {
        return  1.;
    }
    return x * x * (3. - 2. * x)
}

fn smootherstep(x: f64) -> f64 {
    if x<=0. {
        return 0.;
    }
    if x>=1. {
        return 1.;
    }
    return x * x * x * (10. + x * (6. * x - 15.))
}
