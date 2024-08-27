use crate::*;

pub fn create(c: Config) -> RgbaImage {

    let mut image = HsvaImage::new(c.width, c.height, Hsva::new(0., 0., 0., 0.));
    let mut rng = rand::thread_rng();

    let (mut current_x, mut current_y) = (rng.gen_range(0..image.width), rng.gen_range(0..image.height));
    let initial_colour = random_Hsva(1.);
    image.set_pixel(current_x, current_y, initial_colour);
    // println!("({current_x}, {current_y})");

    for move_length in (1..c.width * 2).step_by(2) {
        for _right in 0..move_length {
            current_x += 1;
            if !image.in_bounds(current_x, current_y) {
                continue;
            }
            let colour = adjacent_avg_incl(&image, current_x, current_y);
            image.set_pixel(current_x, current_y, colour);
        }

        for _down in 0..move_length {
            current_y += 1;
            if !image.in_bounds(current_x, current_y) {
                continue;
            }
            let colour = adjacent_avg_incl(&image, current_x, current_y);
            image.set_pixel(current_x, current_y, colour);
        }

        for _left in 0..(move_length + 1) {
            current_x -= 1;
            if !image.in_bounds(current_x, current_y) {
                continue;
            }
            let colour = adjacent_avg_incl(&image, current_x, current_y);
            image.set_pixel(current_x, current_y, colour);
        }

        for _up in 0..(move_length + 1) {
            current_y -= 1;
            if !image.in_bounds(current_x, current_y) {
                continue;
            }
            let colour = adjacent_avg_incl(&image, current_x, current_y);
            image.set_pixel(current_x, current_y, colour);
        }
    }
    
    image.to_rgba_image()
}

pub fn name() -> String {
    "Agile".to_string()
}

fn adjacent_avg_incl(image: &HsvaImage, x: usize, y: usize) -> Hsva {
    let mut rng = rand::thread_rng();

    let up_left = image.get_pixel(x - 1, y - 1);
    let up = image.get_pixel(x, y - 1);
    let up_right = image.get_pixel(x + 1, y - 1);

    let left = image.get_pixel(x - 1, y);
    let right = image.get_pixel(x + 1, y);

    let down_left = image.get_pixel(x - 1, y + 1);
    let down = image.get_pixel(x, y + 1);
    let down_right = image.get_pixel(x + 1, y + 1);

    let adjacents = vec![up_left, up, up_right, left, right, down_left, down, down_right];

    let mut hue: Vec<f64> = vec![];
    let mut sat: Vec<f64> = vec![];
    let mut val: Vec<f64> = vec![];

    for adj in adjacents {
        if let Some(colour) = adj {
            if colour.a != 0. {
                hue.push(colour.h);
                sat.push(colour.s);
                val.push(colour.v);
            }
        }
    }

    let hue_avg: f64 = average_angle(hue);
    let sat_avg: f64 = sat.iter().map(|x| *x).sum::<f64>() / sat.len() as f64;
    let val_avg: f64 = val.iter().map(|x| *x).sum::<f64>() / val.len() as f64;

    let min_mult:f64 = 0.95;
    let max_mult:f64 = (min_mult + (-min_mult * (3. * min_mult - 4.)).sqrt()) / (2. * min_mult);

    // let min_mult = 0.001*-1.*RANDOMNESS/MULTIPLIER as f64;
    // let max_mult = 0.001*RANDOMNESS/MULTIPLIER as f64;

    let hue_mult = rng.gen_range(min_mult..max_mult);
    let sat_mult = rng.gen_range(min_mult..max_mult);
    let val_mult = rng.gen_range(min_mult..max_mult);


    let hue = hue_avg * hue_mult;
    let sat = sat_avg * sat_mult;
    let val = val_avg * val_mult;

    Hsva::new(hue, sat, val, 1.)
}

fn average_angle(angles: Vec<f64>) -> f64 {
    // println!("{:?}", angles);
    let mut xs:Vec<f64> = Vec::new();
    let mut ys:Vec<f64> = Vec::new();
    for angle in angles {
        let rad = zero_one_to_rad(angle);
        xs.push(rad.cos());
        ys.push(rad.sin());
    }
    let x_avg: f64 = xs.iter().map(|x| *x).sum::<f64>() / xs.len() as f64;
    let y_avg: f64 = ys.iter().map(|x| *x).sum::<f64>() / ys.len() as f64;
    let ang = rad_to_zero_one(y_avg.atan2(x_avg));
    ang
}

fn zero_one_to_rad(z:f64) -> f64 {
    z*2.0*std::f64::consts::PI
}

fn rad_to_zero_one(z:f64) -> f64 {
    z/2.0/std::f64::consts::PI
}

fn random_Hsva(a: f64) -> Hsva {
    let mut rng = rand::thread_rng();
    Hsva::new(rng.gen_range(0.0..1.0), rng.gen_range(0.25..=1.0), rng.gen_range(0.5..=1.0), a)
}
