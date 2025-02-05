use crate::*;
use util::hsva::*;

pub const RANDOMNESS: f64 = 0.98;

pub(crate) struct MultiAgile;

const SQUARES:i32 = 7;

impl Plugin for MultiAgile {
    fn create() -> Image {
        let mut image: HSVAImage = HSVAImage::new(WIDTH, HEIGHT, HSVA::new(0., 0., 0., 0.));
        for width in 0..SQUARES{
            for height in 0..SQUARES{
                actually_do_stuff(&mut image, (width*WIDTH)/SQUARES, ((width+1)*WIDTH)/SQUARES, (height*HEIGHT)/SQUARES, ((height+1)*HEIGHT)/SQUARES);
            }
        }

        image.to_rgb_image()
    }
}

fn actually_do_stuff(image: &mut HSVAImage, x_min: i32, x_max: i32, y_min: i32, y_max: i32) {
    let mut rng = rand::thread_rng();

    let (mut current_x, mut current_y) = (rng.gen_range(x_min..x_max), rng.gen_range(y_min..y_max));
    let initial_colour = random_hsv(100.);
    image.set_pixel(current_x, current_y, initial_colour);

    for move_length in (1..(x_max-x_min) * 2).step_by(2) {
        for _right in 0..move_length {
            current_x += 1;
            if !in_bounds(image, current_x, current_y, x_min, x_max, y_min, y_max) {
                continue;
            }
            let color = adjacent_avg_incl(image, current_x, current_y);
            image.set_pixel(current_x, current_y, color);
        }

        for _down in 0..move_length {
            current_y += 1;
            if !in_bounds(image, current_x, current_y, x_min, x_max, y_min, y_max) {
                continue;
            }
            let color = adjacent_avg_incl(image, current_x, current_y);
            image.set_pixel(current_x, current_y, color);
        }

        for _left in 0..(move_length + 1) {
            current_x -= 1;
            if !in_bounds(image, current_x, current_y, x_min, x_max, y_min, y_max) {
                continue;
            }
            let color = adjacent_avg_incl(image, current_x, current_y);
            image.set_pixel(current_x, current_y, color);
        }

        for _up in 0..(move_length + 1) {
            current_y -= 1;
            if !in_bounds(image, current_x, current_y, x_min, x_max, y_min, y_max) {
                continue;
            }
            let color = adjacent_avg_incl(image, current_x, current_y);
            image.set_pixel(current_x, current_y, color);
        }
    }
}

fn adjacent_avg_incl(image: &HSVAImage, x: i32, y: i32) -> HSVA {
    let mut rng = rand::thread_rng();

    let up_left = get_pixel(image, x - 1, y - 1);
    let up = get_pixel(image, x, y - 1);
    let up_right = get_pixel(image, x + 1, y - 1);

    let left = get_pixel(image, x - 1, y);
    let right = get_pixel(image, x + 1, y);

    let down_left = get_pixel(image, x - 1, y + 1);
    let down = get_pixel(image, x, y + 1);
    let down_right = get_pixel(image, x + 1, y + 1);

    let adjacents = vec![up_left, up, up_right, left, right, down_left, down, down_right];

    let mut hue: Vec<f64> = vec![];
    let mut sat: Vec<f64> = vec![];
    let mut val: Vec<f64> = vec![];

    for adj in adjacents {
        if let Some(color) = adj {
            hue.push(color.h);
            sat.push(color.s);
            val.push(color.v);
        }
    }

    let hue_avg: f64 = hue.iter().map(|x| *x as f64).sum::<f64>() / hue.len() as f64;
    let sat_avg: f64 = sat.iter().map(|x| *x as f64).sum::<f64>() / sat.len() as f64;
    let val_avg: f64 = val.iter().map(|x| *x as f64).sum::<f64>() / val.len() as f64;

    let min_mult = RANDOMNESS;
    let max_mult = (min_mult + ((-min_mult * (3. * min_mult - 4.)) as f64).sqrt()) / (2. * min_mult);
    let hue_mult = rng.gen_range(min_mult..max_mult);
    let sat_mult = rng.gen_range(min_mult..max_mult);
    let val_mult = rng.gen_range(min_mult..max_mult);


    let hue = hue_avg * hue_mult;
    let sat = sat_avg * sat_mult;
    let val = val_avg * val_mult;

    HSVA::new(hue % 360., sat.clamp(0., 100.), val.clamp(0., 100.), 100.)
}

fn get_pixel(image: &HSVAImage, x: i32, y: i32) -> Option<HSVA> {
    if !in_bounds(image, x, y, 0, image.width, 0, image.height) {
        return None;
    } else if image.get_pixel(x, y).unwrap().a == 0. {
        return None;
    }
    image.get_pixel(x, y)
}

fn in_bounds(image: &HSVAImage, x: i32, y: i32, x_min:i32, x_max:i32, y_min:i32, y_max:i32) -> bool {
    x >= 0 && x < image.width && y >= 0 && y < image.height
}

fn random_hsv(a: f64) -> HSVA {
    let mut rng = rand::thread_rng();
    HSVA::new(rng.gen_range(0.0..360.0), rng.gen_range(25.0..=100.0), rng.gen_range(50.0..=100.0), a)
}
