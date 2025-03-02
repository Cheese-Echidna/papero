use glam::Vec2;
use image::Rgb;
use crate::algorithms::shapes::types::shape_object::ShapeObject;

pub struct Ball {
    position: Vec2,
    colour: Rgb<f32>,
    radius: f32,
}

impl Ball {
    pub fn new(position: Vec2, colour: Rgb<f32>, radius: f32) -> Self {
        Ball {
            position,
            colour,
            radius,
        }
    }
}

impl ShapeObject for Ball {
    fn sdf(&self, point: &Vec2) -> f32 {
        self.position.distance(*point) - self.radius
    }

    fn colour(&self) -> Rgb<f32> {
        self.colour
    }

    fn position_mut(&mut self) -> &mut Vec2 {
        &mut self.position
    }
}
