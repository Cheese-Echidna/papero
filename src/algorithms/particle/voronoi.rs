use crate::*;
use crate::algorithms::particle::particle::Point;
use rayon::prelude::*;

const BLACK: Rgba<f32> = Rgba([0.0, 0.0, 0.0, 1.0]);

pub struct Voronoi;

impl Generator for Voronoi {
    fn gen_image(args: &Args) -> DynamicImage {
        let points = (0_usize..500)
            .map(|_| {
                (Point::random_particle_zero_one(), random_utils::random_colour::random_rgb_f32())
            }
        ).collect::<Vec<(Point, Rgba<f32>)>>();

        let mut image = Rgba32FImage::new(args.width, args.height);

        for py in 0..args.height {
            let changes = (0..args.width).into_par_iter().map(|px| {
                let (x, y) = (px as f64 / args.width as f64, py as f64 / args.width as f64);
                let position = Point::new(x, y);
                let (_distance, colour) = points.iter()
                    .map(|(pos, col)| (pos.distance(&position), col))
                    .fold((f64::MAX, BLACK), |(current_dist, current_colour), (new_dist, new_colour)| {
                        if new_dist < current_dist {
                            (new_dist, *new_colour)
                        } else {
                            (current_dist, current_colour)
                        }
                    });

                (px, colour)
            }).collect::<Vec<(u32, Rgba<f32>)>>();
            for (x1, colour) in changes {
                image.put_pixel(x1, py, colour);
            }
        }
        image.into()
    }

    fn name() -> &'static str {
        "Voronoi"
    }
}
