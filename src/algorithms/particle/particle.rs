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
}

