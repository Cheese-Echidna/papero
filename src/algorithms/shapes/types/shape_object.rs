use glam::Vec2;
use image::Rgb;
use crate::algorithms::shapes::types::ball::AveragableColour;

pub trait ShapeObject<T: AveragableColour>: Send + Sync {
    fn sdf(&self, position: &Vec2) -> f32;

    fn colour(&self) -> T;

    fn position_mut(&mut self) -> &mut Vec2;

    fn contains(&self, p: Vec2) -> bool {
        self.sdf(&p) <= 0.0
    }
}