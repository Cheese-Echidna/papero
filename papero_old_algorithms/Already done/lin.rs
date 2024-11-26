use crate::*;

pub fn create(config: ConfigData) -> RgbaImage {

    let mut rng = rand::thread_rng();

    let mut image = RgbaImage::transparent(config.width, config.height);

    let (mut current_x, mut current_y) = (rng.gen_range((image.width/3)..(image.width*2/3)), rng.gen_range((image.height/5)..(image.height*4/5)));
    let initial_colour = random_rgb();
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

    // util::aa::smooth(&mut image);

    image
}

pub fn name() -> String {
    "Neo".to_string()
}

fn random_rgb() -> Rgba {
    let mut rng = rand::thread_rng();

    let red = rng.gen_range(0..=255);
    let green = rng.gen_range(0..=255);
    let blue = rng.gen_range(0..=255);

    Rgba::new(red, green, blue,255)
}

fn adjacent_avg_incl(image: &RgbaImage, x: usize, y: usize, config:ConfigData) -> Rgba {
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

    let mut xs = vec![];
    let mut ys = vec![];
    let mut zs = vec![];

    for adj in adjacents {
        if let Some(colour) = adj {
            if colour.a == 0 {
                continue;
            }
            let c2 = colour.to_xyz();


            xs.push(c2.x);
            ys.push(c2.y);
            zs.push(c2.z);
        }
    }

    let x_avg = xs.iter().map(|x| *x as f64).sum::<f64>() / xs.len() as f64;
    let y_avg = ys.iter().map(|x| *x as f64).sum::<f64>() / ys.len() as f64;
    let z_avg = zs.iter().map(|x| *x as f64).sum::<f64>() / zs.len() as f64;

    let min_mult = -0.01*config.rand/(config.res_multiplier as f64);
    let max_mult = 0.01*config.rand/(config.res_multiplier as f64);

    // let min_mult = 0.975;
    // let max_mult = (min_mult + ((-min_mult * (3. * min_mult - 4.)) as f64).sqrt()) / (2. * min_mult);

    let x_mult = rng.gen_range(min_mult..=max_mult);
    let y_mult = rng.gen_range(min_mult..=max_mult);
    let z_mult = rng.gen_range(min_mult..=max_mult);

    // let min_add = -1.;
    // let max_add = -min_add;
    // let red_add = rng.gen_range(min_add..max_add);
    // let green_add = rng.gen_range(min_add..max_add);
    // let blue_add = rng.gen_range(min_add..max_add);

    let x = x_avg + x_mult;
    let y = y_avg + y_mult;
    let z = z_avg + z_mult;

    let c = hue::colours::Xyz::new(x,y,z);
    c.to_rgba()
}
