use crate::*;
use indicatif;

pub fn create(config: ConfigData) -> RgbaImage {
    let image = everything_else(config);

    image
}

fn everything_else(config: ConfigData) -> RgbaImage {
    let mut rng = rand::thread_rng();

    let mut image = RgbaImage::transparent(config.width, config.height);

    let (mut current_x, mut current_y) = (rng.gen_range((image.width/3)..(image.width*2/3)), rng.gen_range((image.height/5)..(image.height*4/5)));
    let initial_colour = random_rgb();
    image.set_pixel(current_x, current_y, initial_colour).unwrap();

    let progress = indicatif::ProgressBar::new((config.width*2) as u64).with_style(indicatif::ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}").expect("Beans").progress_chars("#>-"));

    for move_length in (1..config.width * 2).step_by(2) {
        if move_length % 101 == 0 {
            progress.inc(101*2);
        }

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

    image
}

pub fn name() -> String {
    "Neo".to_string()
}

fn random_rgb() -> Rgba {
    let mut rng = rand::thread_rng();

    let red = rng.gen_range(50..=255);
    let green = rng.gen_range(50..=255);
    let blue = rng.gen_range(50..=255);

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

    let mut reds = vec![];
    let mut greens = vec![];
    let mut blues = vec![];

    for adj in adjacents {
        if let Some(colour) = adj {
            if colour.a == 0 {
                continue;
            }
            reds.push(colour.r);
            greens.push(colour.g);
            blues.push(colour.b);
        }
    }

    let red_avg = reds.iter().map(|x| *x as f64).sum::<f64>() / reds.len() as f64;
    let green_avg = greens.iter().map(|x| *x as f64).sum::<f64>() / greens.len() as f64;
    let blue_avg = blues.iter().map(|x| *x as f64).sum::<f64>() / blues.len() as f64;

    // let min_mult = -1.2*config.rand/(config.res_multiplier as f64).sqrt();
    // let max_mult = 1.2*config.rand/(config.res_multiplier as f64).sqrt();

    let min_mult = 0.95;
    let max_mult = (min_mult + ((-min_mult * (3. * min_mult - 4.)) as f64).sqrt()) / (2. * min_mult);

    let red_mult = rng.gen_range(min_mult..=max_mult);
    let green_mult = rng.gen_range(min_mult..=max_mult);
    let blue_mult = rng.gen_range(min_mult..=max_mult);

    let red = red_avg * red_mult;
    let green = green_avg * green_mult;
    let blue = blue_avg * blue_mult;

    Rgba::new(red.round() as u8, green.round() as u8, blue.round() as u8, 255)
}

fn mean_colour(v: Vec<Rgba>) -> Rgba {
    let mut r: f64 = 0.0;
    let mut g: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut a: f64 = 0.0;

    for colour in &v {
        r += colour.r as f64;
        g += colour.g as f64;
        b += colour.b as f64;
        a += colour.a as f64;
    }

    // NOTE: This could easily overflow, be careful, look for artifacts
    let n = v.len() as f64;
    return Rgba::new((r / n) as u8, (g / n) as u8, (b / n) as u8, (a / n) as u8);
}

fn average_images(images: Vec<RgbaImage>, config:ConfigData) -> RgbaImage {
    let mut image: RgbaImage = RgbaImage::blank(config.width, config.height);
    for y in 0..image.height {
        for x in 0..image.width {
            image.set_pixel(x, y, Rgba::new(0, 0, 0, 0)).unwrap();
        }
    }

    for j in 0..config.height {
        for i in 0..config.width {
            let mut pxs: Vec<Rgba> = Vec::new();
            for k in &images {
                pxs.push(k.get_pixel(i, j).unwrap())
            }
            image.set_pixel(i, j, mean_colour(pxs)).unwrap();
        }
    }
    image
}

fn rgb_imager(red: RgbaImage, green: RgbaImage, blue: RgbaImage, config:ConfigData) -> RgbaImage {
    let mut image: RgbaImage = RgbaImage::blank(config.width, config.height);

    for j in 0..config.height {
        for i in 0..config.width {
            image.set_pixel(i, j, Rgba::new(red.get_pixel(i, j).unwrap().r, green.get_pixel(i, j).unwrap().g, blue.get_pixel(i, j).unwrap().b, 255)).unwrap();
        }
    }
    image
}