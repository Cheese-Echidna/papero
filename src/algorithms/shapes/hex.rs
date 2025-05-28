use std::f32::consts::TAU;
use crate::*;
use crate::{Args, Generator};
use glam::{UVec2, Vec2};
use num::integer::Roots;
use rand::random;
use crate::algorithms::shapes::types::hex::Hexagon;
use crate::algorithms::shapes::types::shape_object::ShapeObject;
use crate::algorithms::shapes::types::shape_set::ShapeSet;

#[derive(Default)]
pub struct Hex;

impl Generator for Hex {
    fn generate(args: &Args) -> DynamicImage {
        let points = hexagons(args);
        let shapes = ShapeSet { objects: points };

        shapes.generate(args, None)
    }

    fn name() -> &'static str {
        "Hexagons"
    }
}


pub fn hexagons(args: &Args) -> Vec<Box<dyn ShapeObject<Rgb<f32>>>> {
    let (width, height) = args.wh();

    let vx = 120;
    let factor = 3_f32.sqrt() / 2.;
    let vy = (vx as f32 * factor) as u32;


    let mut points = vec![];
    for x in (0..(width + vx)).step_by(vx as usize) {
        for y in (0..(height)).step_by(vy as usize) {
            let row_odd = (y / vy) % 2;
            let offset = Vec2::new(vx as f32, vy as f32) / 2. * Vec2::new(row_odd as f32, 0.5);
            let pos = UVec2::new(x, y).as_vec2() + offset;
            let prop = pos / UVec2::new(width, height).as_vec2();
            let size = 50.;
            let colour = colour_utils::convert_from_ok_hsl(
                prop.x,
                1.0,
                prop.y * 0.5 + 0.5,
            );

            // let (flat, pointy) = (0.0, TAU / 12.);
            //
            // let rotation = if random::<f32>() < 0.5 {
            //     flat
            // } else {
            //     pointy
            // };

            let rotation = TAU / 12.;

            let p = Hexagon::new(
                pos,
                colour,
                size,
                rotation,
            );

            points.push(Box::new(p) as Box<dyn ShapeObject<Rgb<f32>>>);
        }
    }
    points
}
