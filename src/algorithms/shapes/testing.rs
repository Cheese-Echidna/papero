use glam::Vec2;
use image::{DynamicImage, Rgb};
use palette::named::BLACK;
use crate::algorithms::shapes::{composite, hex, types};
use crate::algorithms::shapes::types::shape_set::ShapeSet;
use crate::Generator;
use crate::utils::colour_utils::ImageColour;
use crate::utils::image_manager::Args;

#[derive(Default)]
pub(crate) struct ShapeTesting {}

impl Generator for ShapeTesting {
    fn generate(args: &Args) -> DynamicImage {
        let shapes = hex::hexagons(args);
        let s = ShapeSet { objects: shapes };
        let image = crate::algorithms::particle::flow::Flow::generate(args).into_rgb32f();
        composite::CompositeRenderer {
            generators: vec![(image, s)],
            default: args.image_f32(Rgb::from_const(BLACK)),
        }.generate(args)
    }

    fn name() -> &'static str {
        "Hexagon mapped flow"
    }
}