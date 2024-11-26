use crate::*;
use indicatif;

pub fn create(config: Config) -> RgbaImage {
    let image = everything_else(config);
    image
}

fn everything_else(config: Config) -> RgbaImage {
    let mut rng = rand::thread_rng();

    let mut image = Hsva01Image::transparent(config.width, config.height);

    let (mut current_x, mut current_y) = (rng.gen_range((image.width/3)..(image.width*2/3)), rng.gen_range((image.height/5)..(image.height*4/5)));
    let initial_colour = random_hsva01();
    image.set_pixel(current_x, current_y, initial_colour).unwrap();

    for move_length in (1..config.width * 2).step_by(2) {
        for _right in 0..move_length {
            current_x += 1;
            if !image.in_bounds(current_x, current_y) {
                continue;
            }
            let colour = adjacent_avg_incl(&image, current_x, current_y, config);
            image.set_pixel(current_x, current_y, colour).unwrap();
        }

        for _down in 0..move_length {
            current_y += 1;
            if !image.in_bounds(current_x, current_y) {
                continue;
            }
            let colour = adjacent_avg_incl(&image, current_x, current_y, config);
            image.set_pixel(current_x, current_y, colour).unwrap();
        }

        for _left in 0..(move_length + 1) {
            current_x -= 1;
            if !image.in_bounds(current_x, current_y) {
                continue;
            }
            let colour = adjacent_avg_incl(&image, current_x, current_y, config);
            image.set_pixel(current_x, current_y, colour).unwrap();
        }

        for _up in 0..(move_length + 1) {
            current_y -= 1;
            if !image.in_bounds(current_x, current_y) {
                continue;
            }
            let colour = adjacent_avg_incl(&image, current_x, current_y, config);
            image.set_pixel(current_x, current_y, colour).unwrap();
        }
    }

    image.to_rgba_image()
}

pub fn name() -> String {
    "Neo".to_string()
}

fn random_hsva01() -> Hsva01 {
    let mut rng = rand::thread_rng();

    let h = rng.gen_range(5..=95) as f64 / 100.0;
    let s = rng.gen_range(5..=95) as f64 / 100.0;
    let v = rng.gen_range(5..=95) as f64 / 100.0;

    Hsva01::new(h,s,v,1.0)
}

fn adjacent_avg_incl(image: &Hsva01Image, x: usize, y: usize, config: Config) -> Hsva01 {
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

    let mut hues = vec![];

    for adj in adjacents {
        if let Some(colour) = adj {
            if colour.a == 0.0 {
                continue;
            }
            hues.push(colour);
            // Switch to Hsva01 when I can
        }
    }

    let mut c = Hsva01::average(&hues);

    // let min_mult = -1.2*config.rand/(config.res_multiplier as f64).sqrt();
    // let max_mult = 1.2*config.rand/(config.res_multiplier as f64).sqrt();

    let min_add = (100.0-1.8/(config.res_multiplier*config.num_screens) as f64)/100.0;
    let max_add = 2.0 - min_add;

    let hue_add = rng.gen_range(min_add..=max_add);
    let sat_add = rng.gen_range(min_add..=max_add);
    let val_add = rng.gen_range(min_add..=max_add);

    c.h += hue_add;
    c.s += sat_add;
    c.v += val_add;
    c.a = 1.;

    c.normalise()
}
