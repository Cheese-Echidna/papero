use crate::algorithms::shapes::shape::{ShapeObject, ShapeSet};
use crate::*;
use crate::{Args, Generator};
use glam::{UVec2, Vec2};
use rand_distr::num_traits::clamp;

#[derive(Default)]
pub struct Hex;

impl Generator for Hex {
    fn generate(args: &Args) -> DynamicImage {
        let (width, height) = args.wh();

        let mut points = vec![];
        for x in (0..width).step_by(120) {
            for y in (0..height).step_by(120) {
                let pos = UVec2::new(x, y).as_vec2();
                let size = (x as f32).sqrt();
                let colour = colour_utils::convert_from_ok_hsl(
                    x as f32 / width as f32,
                    1.0,
                    y as f32 / height as f32 * 0.5 + 0.5,
                );

                let p = Hexagon {
                    pos,
                    col: colour,
                    size,
                    radius: 0.0,
                };
                points.push(Box::new(p) as Box<dyn ShapeObject>);
            }
        }
        let shapes = ShapeSet { objects: points };

        shapes.generate(&args)
    }

    fn name() -> &'static str {"Hexagons"}
}

// Note for now we are only doing pointy top hexagons
// size is the distance from the point to the centre
pub struct Hexagon {
    pos: Vec2,
    col: Rgb<f32>,
    size: f32,
    radius: f32,
}

impl ShapeObject for Hexagon {
    fn sdf(&self, position: &Vec2) -> f32 {
        let k2 = Vec2::new(-0.866025404, 0.5);
        let z = 0.577350269;
        let s = self.size;
        let r = self.radius;

        // confusing line
        let mut p = (*position - self.pos).abs();
        p -= k2 * (2.0 * f32_min(k2.dot(p), 0.0));
        p -= Vec2::new(clamp(p.x, -z * s, z * s), s);
        p.length() * sign(p.y) - r
    }

    fn colour(&self) -> Rgb<f32> {
        self.col.clone()
    }

    fn position_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }
}

fn f32_min(x: f32, y: f32) -> f32 {
    if x > y {
        return y;
    }
    return x;
}

fn sign(x: f32) -> f32 {
    if x >= 0.0 {
        1.0
    } else {
        -1.0
    }
}
