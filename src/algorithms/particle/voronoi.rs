use crate::*;
use crate::algorithms::particle::particle::Point;
use rayon::prelude::*;

const BLACK: Rgb<f32> = Rgb([0.0, 0.0, 0.0]);

#[derive(Default)]
pub struct Voronoi;

impl Generator for Voronoi {
    fn generate(args: &Args) -> DynamicImage {
        let (w, h) = (args.width as f64, args.height as f64);
        let points = (0_usize..4000)
            .map(|_| {
                let p = Point::random_particle_w_h(w, h);
                let (x ,y) = ((p.x() / w) as f32,  (p.y() / h) as f32);
                let c= colour_utils::convert_from_ok_hsl(x, y, 0.5);
                (p, c)
            }
        ).collect::<Vec<(Point, Rgb<f32>)>>();

        let mut image = Rgb32FImage::new(args.width, args.height);

        for py in 0..args.height {
            (0..args.width).into_par_iter().map(|px| {
                let position = Point::new(px as f64, py as f64);
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
            }).collect::<Vec<(u32, Rgb<f32>)>>().into_iter().for_each(|(x1, colour)| {
                image.put_pixel(x1, py, colour);
            })
        }
        image.into()
    }

    fn name() -> &'static str {
        "Voronoi"
    }
}
