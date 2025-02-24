use std::f32::consts::{FRAC_PI_2, TAU};
use glam::f64::DVec2;
use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex};
use std::f64::consts::{FRAC_1_PI, FRAC_1_SQRT_2};
use glam::Vec2;

/// Fractional Brownian motion (FBM) noise is a fractal like noise
///
/// This means that there is more and more detail as you zoom in
///
/// h is the scaling factor, as h increases the zoom level increases
///
/// n is the number of octaves, as n increases the detail increases
pub struct BetterFbm {
    fbm: Fbm<OpenSimplex>,
    scale: f64,
    octaves: usize,
    seed: u32
}

impl BetterFbm {
    pub fn get(&self, x:DVec2) -> f64 {
        let offset = DVec2::new(FRAC_1_PI, FRAC_1_SQRT_2);
        self.fbm.get((x * self.scale + offset).to_array())
    }

    pub fn new(seed: u32, octaves: usize, scale: f64) -> BetterFbm {
        let fbm = noise::Fbm::<OpenSimplex>::new(seed).set_octaves(octaves);
        Self {
            fbm,
            scale,
            octaves,
            seed,
        }
    }
}