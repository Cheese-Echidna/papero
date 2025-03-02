use glam::Vec2;
use image::Rgb;

pub trait ShapeObject: Send + Sync {
    fn sdf(&self, position: &Vec2) -> f32;

    fn colour(&self) -> Rgb<f32>;

    fn position_mut(&mut self) -> &mut Vec2;
}