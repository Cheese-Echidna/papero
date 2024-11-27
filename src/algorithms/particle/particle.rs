use glam::Vec2;
use palette::{IntoColor, LinSrgb};
use rand::Rng;

pub(crate) struct Particle {
    pub position: Vec2,
    pub colour: palette::LinSrgb<f32>
}

impl Particle {
    pub fn new(point: impl Into<glam::Vec2>, colour: impl IntoColor<palette::LinSrgb>) -> Self {
        Particle {
            position: point.into(),
            colour: colour.into_color(),
        }
    }
    // pub fn new_random(width:u32, height:u32) -> Self {
    //     Self::new(Point::random_particle_w_h(width as f64, height as f64), crate::utils::colour_utils::random_colour().into_color())
    // }
}

// pub(crate) fn random_particle_zero_one() -> Self {
//         let mut rng = rand::thread_rng();
//         Point::new(
//             rng.gen_range(0.0..1.),
//             rng.gen_range(0.0..1.)
//         )
//     }
//     pub(crate) fn random_particle_w_h(w:f64, h:f64) -> Self {
//         let mut rng = rand::thread_rng();
//         Point::new(
//             rng.gen_range(0.0..w),
//             rng.gen_range(0.0..h)
//         )
//     }
//     pub(crate) fn distance(&self, other: &Point) -> f64 {
//         ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
//     }
//     pub(crate) fn x(&self) -> f64 {
//         self.x
//     }
//     pub(crate) fn y(&self) -> f64 {
//         self.y
//     }
// }