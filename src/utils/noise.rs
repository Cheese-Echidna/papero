use glam::f64::DVec2;
use noise::{MultiFractal, NoiseFn, OpenSimplex};
use std::f64::consts::{FRAC_1_PI, FRAC_1_SQRT_2};

/// Fractional Brownian motion (FBM) noise is a fractal like noise
///
/// This means that there is more and more detail as you zoom in
///
/// h is the scaling factor, as h increases the zoom level increases
///
/// n is the number of octaves, as n increases the detail increases
pub fn fbm(seed: u32, h: f64, n: usize, x: DVec2) -> f64 {
    let offset = DVec2::new(FRAC_1_PI, FRAC_1_SQRT_2);
    noise::Fbm::<OpenSimplex>::new(seed)
        .set_octaves(n)
        .get((x * h + offset).to_array())
}

// todo: Add a tillable fmb function
