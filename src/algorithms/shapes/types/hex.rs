use glam::Vec2;
use image::Rgb;
use num::clamp;
use crate::algorithms::shapes::types::shape_object::ShapeObject;

// Note for now we are only doing pointy top hexagons
// size is the distance from the point to the centre
pub struct Hexagon {
    pos: Vec2,
    col: Rgb<f32>,
    size: f32,
    radius: f32,
}

impl Hexagon {
    pub fn new(pos: Vec2, col: Rgb<f32>, size: f32, radius: f32) -> Self {
        Self {
            pos,
            col,
            size,
            radius,
        }
    }
}

impl ShapeObject for Hexagon {
    fn sdf(&self, position: &Vec2) -> f32 {
        let k2 = Vec2::new(-0.866_025_4, 0.5);
        let z = 0.577_350_26;
        let s = self.size;
        let r = self.radius;

        // confusing line
        let mut p = (*position - self.pos).abs();
        p -= k2 * (2.0 * k2.dot(p).min(0.0));
        p -= Vec2::new(clamp(p.x, -z * s, z * s), s);
        p.length() * p.y.signum() - r
    }

    fn colour(&self) -> Rgb<f32> {
        self.col
    }

    fn position_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }
}