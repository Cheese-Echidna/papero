use crate::algorithms::shapes::types::shape_set::ShapeSet;
use crate::utils::colour_utils::ImageColour;
use crate::utils::image_manager::Args;
use glam::UVec2;
use image::{DynamicImage, Rgb, Rgb32FImage};
use palette::named::BLACK;

pub struct CompositeRenderer {
    pub generators: Vec<(Rgb32FImage, ShapeSet)>,
    pub default: Rgb32FImage,
}

impl CompositeRenderer {
    pub fn from_generators_shapes(
        generators: Vec<Box<dyn Fn(&Args) -> DynamicImage>>,
        shapes: Vec<ShapeSet>,
        args: &Args,
        default: Box<dyn Fn(&Args) -> DynamicImage>,
    ) -> Self {
        let g = generators
            .into_iter()
            .zip(shapes)
            .map(|(x, y)| ((x)(args).to_rgb32f(), y))
            .collect::<Vec<_>>();
        Self {
            generators: g,
            default: (default)(args).to_rgb32f(),
        }
    }

    pub fn generate(self, args: &Args) -> DynamicImage {
        let mut image = args.image_f32(Rgb::from_const(BLACK));

        for y in 0..args.height {
            for x in 0..args.width {
                let pos = UVec2::new(x, y).as_vec2();
                for (ref_image, shape) in self.generators.iter() {
                    if shape.contains(pos) {
                        let ref_pix = ref_image.get_pixel(x, y);
                        let shape_pix = shape.colour(pos);
                        image.put_pixel(x, y, mult(*ref_pix, shape_pix));
                    }
                }
            }
        }
        image.into()
    }
}

fn mult(bg: Rgb<f32>, fg: Rgb<f32>) -> Rgb<f32> {
    Rgb(bg.0.iter().zip(fg.0).map(|(a, b)| a * b).collect::<Vec<_>>().try_into().unwrap())
}