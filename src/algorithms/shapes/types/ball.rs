use crate::algorithms::shapes::halftone::CMYK;
use crate::algorithms::shapes::types::shape_object::ShapeObject;
use crate::utils::colour_utils::Colour3;
use glam::{Vec2, Vec3};
use image::Rgb;

pub trait AveragableColour: Sized + Send + Sync + Clone {
    fn avg(d: &[Self]) -> Rgb<f32>;
}

impl AveragableColour for Rgb<f32> {
    fn avg(colours: &[Self]) -> Rgb<f32> {
        Rgb::<f32>::from_vec3(
            colours
                .iter()
                .map(|x| x.0)
                .fold(Vec3::ZERO, |acc, x| acc + Vec3::from_array(x))
                / colours.len() as f32,
        )
    }
}

impl AveragableColour for CMYK {
    fn avg(dots: &[Self]) -> Rgb<f32> {
        // Accumulate inverse transparencies
        let mut inv_c = 1.0_f32;
        let mut inv_m = 1.0_f32;
        let mut inv_y = 1.0_f32;
        let mut inv_k = 1.0_f32;

        for dot in dots.iter() {
            let dot = dot.0.to_array();
            let (c, m, y, k) = (dot[0], dot[1], dot[2], dot[3]);
            inv_c *= 1.0 - c;
            inv_m *= 1.0 - m;
            inv_y *= 1.0 - y;
            inv_k *= 1.0 - k;
        }

        // Total coverages
        let tc = 1.0 - inv_c;
        let tm = 1.0 - inv_m;
        let ty = 1.0 - inv_y;
        let tk = 1.0 - inv_k;

        // Convert back to RGB
        let r = (1.0 - tc) * (1.0 - tk);
        let g = (1.0 - tm) * (1.0 - tk);
        let b = (1.0 - ty) * (1.0 - tk);

        Rgb::from([r, g, b])
    }
}

pub struct Ball<T> {
    position: Vec2,
    colour: T,
    radius: f32,
}

impl<T> Ball<T> {
    pub fn new(position: Vec2, colour: T, radius: f32) -> Self {
        Ball {
            position,
            colour,
            radius,
        }
    }
}

impl<T: AveragableColour> ShapeObject<T> for Ball<T> {
    fn sdf(&self, point: &Vec2) -> f32 {
        self.position.distance(*point) - self.radius
    }

    fn colour(&self) -> T {
        self.colour.clone()
    }

    fn position_mut(&mut self) -> &mut Vec2 {
        &mut self.position
    }
}
