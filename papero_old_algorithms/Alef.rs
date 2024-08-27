use hue::*;
use crate::{Args, Generator};
use rand::Rng;
use rand_distr;
use rand_distr::Distribution;
pub struct Alef;

impl Generator for Alef {
    fn create(&self, args: &Args) -> RgbaImage {
        let mut rng = rand::thread_rng();

        let (width, height) = args.get_wh();

        let mut image = RgbaImage::transparent(width, height);

        let (mut current_x, mut current_y) = (rng.gen_range((image.width/3)..(image.width*2/3)), rng.gen_range((image.height/3)..(image.height*2/3)));
        let initial_colour = random_rgb();
        image.set_pixel(current_x, current_y, initial_colour).unwrap();

        for move_length in (1..width * 2).step_by(2) {
            // if move_length % 101 == 0 {
            //     progress.inc(101*2);
            // }

            for _right in 0..move_length {
                current_x += 1;
                if !image.in_bounds(current_x, current_y) {
                    continue;
                }
                let colour = adjacent_avg_incl(&image, current_x, current_y);
                image.set_pixel(current_x, current_y, colour).unwrap();
            }

            for _down in 0..move_length {
                current_y += 1;
                if !image.in_bounds(current_x, current_y) {
                    continue;
                }
                let colour = adjacent_avg_incl(&image, current_x, current_y);
                image.set_pixel(current_x, current_y, colour).unwrap();
            }

            for _left in 0..(move_length + 1) {
                current_x -= 1;
                if !image.in_bounds(current_x, current_y) {
                    continue;
                }
                let colour = adjacent_avg_incl(&image, current_x, current_y);
                image.set_pixel(current_x, current_y, colour).unwrap();
            }

            for _up in 0..(move_length + 1) {
                current_y -= 1;
                if !image.in_bounds(current_x, current_y) {
                    continue;
                }
                let colour = adjacent_avg_incl(&image, current_x, current_y);
                image.set_pixel(current_x, current_y, colour).unwrap();
            }
        }

        image

    }
}

pub fn random_rgb() -> Rgba {
    let mut rng = rand::thread_rng();

    let red = rng.gen_range(0.0..=1.0);
    let green = rng.gen_range(0.0..=1.0);
    let blue = rng.gen_range(0.0..=1.0);

    Rgba::new(red, green, blue,1.0)
}

fn adjacent_avg_incl(image: &RgbaImage, x: usize, y: usize) -> Rgba {
    let mut rng = rand::thread_rng();

    let up_left = image.get_pixel(x.wrapping_sub(1), y.wrapping_sub(1));
    let up = image.get_pixel(x, y.wrapping_sub(1));
    let up_right = image.get_pixel(x + 1, y.wrapping_sub(1));

    let left = image.get_pixel(x.wrapping_sub(1), y);
    let right = image.get_pixel(x + 1, y);

    let down_left = image.get_pixel(x.wrapping_sub(1), y + 1);
    let down = image.get_pixel(x, y + 1);
    let down_right = image.get_pixel(x + 1, y + 1);

    let adjacents = vec![up_left, up, up_right, left, right, down_left, down, down_right];

    let mut reds = vec![];
    let mut greens = vec![];
    let mut blues = vec![];

    for adj in adjacents {
        if let Some(colour) = adj {
            if colour.a == 0.0 {
                continue;
            }
            reds.push(colour.r);
            greens.push(colour.g);
            blues.push(colour.b);
        }
    }

    let red_avg = reds.iter().sum::<f64>() / reds.len() as f64;
    let green_avg = greens.iter().sum::<f64>() / greens.len() as f64;
    let blue_avg = blues.iter().sum::<f64>() / blues.len() as f64;

    let mut distr = rand_distr::Normal::new(0.0, 0.0175).unwrap();

    let rng = &mut rand::thread_rng();

    let red_add = distr.sample(rng);
    let green_add = distr.sample(rng);
    let blue_add = distr.sample(rng);

    let red = red_avg + red_add;
    let green = green_avg + green_add;
    let blue = blue_avg + blue_add;

    Rgba::new(red, green, blue, 1.0)
}
