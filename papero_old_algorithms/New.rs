use hue::{Hsva, HsvaImage, Rgba, RgbaImage};
use hue::colours::*;
use crate::{Args, Generator};
use crate::Dalet::Hexagon;
use crate::Gimel::{ShapeObject, ShapeSet};
use crate::position::Position;
use rand::{random, Rng};
use noise::{NoiseFn, Perlin, Seedable};
use rayon::prelude::*;

pub struct New;

impl Generator for crate::New::New {
    fn create(&self, args: &Args) -> RgbaImage {
        let (width, height) = args.get_wh();

        let mut rng = rand::thread_rng();

        let points = (0_usize..500)
            .map(|x| (Position { x: rng.gen_range(0.0..1.), y: rng.gen_range(0.0..1.) }, Rgba::random()))
            .collect::<Vec<(Position, Rgba)>>();

        let mut image = RgbaImage::blank(width, height);

        for y1 in 0..height {
            let changes = (0..width).into_par_iter().map(|x1| {
                let (x, y) = (x1 as f64 / width as f64, y1 as f64 / width as f64);
                let position = Position{ x, y };
                let (_distance, colour) = points.iter()
                    .map(|(pos, col)| (pos.distance(&position), col))
                    .fold((f64::MAX, BLACK), |(current_dist, current_colour), (new_dist, new_colour)| {
                        if new_dist < current_dist {
                            (new_dist, *new_colour)
                        } else {
                            (current_dist, current_colour)
                        }
                    });

                (x1, colour)
            }).collect::<Vec<(usize, Rgba)>>();
            for (x1, colour) in changes {
                image.set_pixel(x1, y1, colour).unwrap();

            }
        }
        image
    }
}
