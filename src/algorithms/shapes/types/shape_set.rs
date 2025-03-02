use crate::utils::image_manager::Args;
use glam::{UVec2, Vec2, Vec3};
use image::{DynamicImage, GenericImage, Rgb, Rgb32FImage};
use crate::algorithms::shapes::types::ball::Ball;
use crate::algorithms::shapes::types::shape_object::ShapeObject;

pub struct ShapeSet {
    pub(crate) objects: Vec<Box<dyn ShapeObject>>,
}


impl ShapeSet {
    pub fn new_uniform_of_balls(
        wh: Vec2,
        step_by: f32,
        radius_fn: fn(Vec2, Vec2) -> f32,
        colour_fn: fn(Vec2, Vec2) -> Rgb<f32>,
    ) -> ShapeSet {
        let [width, height] = wh.to_array();
        let mut points = vec![];
        for x in (0..=(width as u32)).step_by(step_by as usize) {
            let x = x as f32;
            for y in (0..=(height as u32)).step_by(step_by as usize) {
                let y = y as f32;
                let pos = Vec2::new(x, y);
                let radius = radius_fn(pos, wh);
                let colour = colour_fn(pos, wh);
                let p = Ball::new(pos, colour, radius);
                points.push(Box::new(p) as Box<dyn ShapeObject>);
            }
        }
        ShapeSet { objects: points }
    }

    pub fn generate(self, args: &Args) -> DynamicImage {
        let (width, height) = args.wh();

        let mut image = Rgb32FImage::new(args.width, args.height);
        for y in 0..height {
            for x in 0..width {
                let pos = UVec2::new(x, y).as_vec2();

                let colours = self
                    .objects
                    .iter()
                    .filter_map(|object| {
                        let d = object.sdf(&pos);
                        if d < 0.0 {
                            return Some(object.colour());
                        };
                        None
                    })
                    .collect::<Vec<Rgb<f32>>>();

                if !colours.is_empty() {
                    let new_col = colours
                        .iter()
                        .map(|x| x.0)
                        .fold(Vec3::ZERO, |acc, x| acc + Vec3::from_array(x))
                        / colours.len() as f32;
                    image.put_pixel(x, y, Rgb::<f32>::from(new_col.to_array()));
                } else {
                    image.put_pixel(x, y, Rgb::from([0.0, 0.0, 0.0]))
                }
            }
        }
        image.into()
    }

    pub fn new(objects: Vec<impl ShapeObject + 'static>) -> Self {
        Self {
            objects: objects.into_iter().map(|x| Box::new(x) as Box<dyn ShapeObject>).collect(),
        }
    }
}
