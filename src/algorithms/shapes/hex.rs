use crate::*;
use crate::{Args, Generator};
use glam::{UVec2, Vec2};
use crate::algorithms::shapes::types::hex::Hexagon;
use crate::algorithms::shapes::types::shape_object::ShapeObject;
use crate::algorithms::shapes::types::shape_set::ShapeSet;

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

                let p = Hexagon::new(
                    pos,
                    colour,
                    size,
                    0.0,
                );

                points.push(Box::new(p) as Box<dyn ShapeObject>);
            }
        }
        let shapes = ShapeSet { objects: points };

        shapes.generate(args)
    }

    fn name() -> &'static str {
        "Hexagons"
    }
}


