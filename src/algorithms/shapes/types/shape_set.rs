use crate::utils::image_manager::Args;
use glam::{UVec2, Vec2, Vec3};
use image::{DynamicImage, Rgb, Rgb32FImage, Rgba};
use crate::algorithms::shapes::types::ball::{AveragableColour, Ball};
use crate::algorithms::shapes::types::shape_object::ShapeObject;
use rayon::prelude::*;

pub struct ShapeSet<T: AveragableColour> {
    pub(crate) objects: Vec<Box<dyn ShapeObject<T>>>,
}

impl<T: AveragableColour> ShapeSet<T> {
    pub fn new_uniform_of_balls(
        wh: Vec2,
        step_by: f32,
        radius_fn: fn(Vec2, Vec2) -> f32,
        colour_fn: fn(Vec2, Vec2) -> Rgb<f32>,
    ) -> ShapeSet<Rgb<f32>> {
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
                points.push(Box::new(p) as Box<dyn ShapeObject<Rgb<f32>>>);
            }
        }
        ShapeSet { objects: points }
    }

    pub fn generate(self, args: &Args, bg: Option<Rgb<f32>>) -> DynamicImage {
        let (width, height) = args.wh();

        let objects = &self.objects;

        let mut image = args.image_f32(bg.unwrap_or(Rgb([0.0, 0.0, 0.0])));
        (0..height).into_par_iter().flat_map(|y| {
            (0..width).into_par_iter().flat_map(move |x| {

                let pos = UVec2::new(x, y).as_vec2();

                let colours = objects.iter()
                    .filter_map(|object| {
                        let d = object.sdf(&pos);
                        if d <= 0.0 {
                            return Some(object.colour());
                        };
                        None
                    })
                    .collect::<Vec<T>>();

                if !colours.is_empty() {
                    Some((x, y, T::avg(&colours)))
                } else {
                    None
                }
            })
        }).collect::<Vec<(u32, u32, Rgb<f32>)>>()
            .into_iter()
            .for_each(|(x, y, c)| image.put_pixel(x, y, c));
        image.into()
    }

    pub fn new(objects: Vec<impl ShapeObject<T> + 'static>) -> Self {
        Self {
            objects: objects.into_iter().map(|x| Box::new(x) as Box<dyn ShapeObject<T>>).collect(),
        }
    }
    
    pub fn empty() -> Self {
        Self {
            objects: vec![],
        }
    }
    
    pub fn contains(&self, p: Vec2) -> bool {
        self.objects.iter().any(|x| x.contains(p))
    }

    pub fn sdf(&self, p: Vec2) -> f32 {
        self.objects.iter().map(|x| x.sdf(&p)).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    // pub fn colour(&self, p:Vec2) -> Rgb<f32> {
    //     self.objects.iter().find(|x| x.contains(p)).unwrap().colour()
    // }
    
    pub fn add(&mut self, object: impl ShapeObject<T> + 'static) {
        self.objects.push(Box::new(object) as Box<dyn ShapeObject<T>>);
    }
}
