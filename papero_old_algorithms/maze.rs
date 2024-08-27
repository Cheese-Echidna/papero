use crate::*;
use crate::util::hsva::*;

const RANDOMNESS: f64 = 0.98;

pub(crate) struct Maze;

impl Plugin for Maze {
    fn create() -> Image {
        let mut image = HSVAImage::new(WIDTH, HEIGHT, HSVA::new(0., 0., 0., 0.));

        let mut x:i32 = 0;
        let mut y:i32 = 0;
        let mut n:i32 = 0;
        let mut k: i32 = 0;
        let mut starting_colour = random_hsv(100.);
        image.set_pixel(x, y, starting_colour.clone()).unwrap();
        // image.set_pixel(x, y, HSVA::new(n as f64, 100., 100., 100.)).unwrap();


        for i in 0..(WIDTH-1) {
            x += 1;
            k += 1;
            image.set_pixel(x, y, starting_colour.clone()).unwrap();
        }

        n+=1;

        for i in 0..(HEIGHT-n) {
            y += 1;
            k += 1;
            image.set_pixel(x, y, starting_colour.clone()).unwrap();
            // image.set_pixel(x, y, HSVA::new(n as f64, 100., 100., 100.)).unwrap();

            // print!("{} - Second loop: ", k);
            // println!("set_pixel: {}, {}", x, y);
        }


        for i in 0..(WIDTH-n) {
            x -= 1;
            k += 1;
            image.set_pixel(x, y, starting_colour.clone()).unwrap();
            // image.set_pixel(x, y, HSVA::new(n as f64, 100., 100., 100.)).unwrap();

            // print!("{} - Third loop: ", k);
            // println!("set_pixel: {}, {}", x, y);
        }

        n+=1;
        for i in 0..(HEIGHT-n) {
            y -= 1;
            k += 1;
            // image.set_pixel(x, y, HSVA::new(n as f64, 100., 100., 100.)).unwrap();

            image.set_pixel(x, y, starting_colour.clone()).unwrap();
            //
            // print!("{} - Fourth loop: ", k);
            // println!("set_pixel: {}, {}", x, y);
        }


        loop {

            for i in 0..(WIDTH-n) {
                x += 1;
                k += 1;
                image.set_pixel(x, y, adjacent_avg_incl(&image, x, y)).unwrap();
                // image.set_pixel(x, y, HSVA::new(n as f64, 100., 100., 100.)).unwrap();
                // print!("{} - First loop: ", k);
                // println!("set_pixel: {}, {}", x, y);
            }


            n+=1;
            if n > WIDTH {
                break;
            }


            for i in 0..(HEIGHT-n) {
                y += 1;
                k += 1;
                image.set_pixel(x, y, adjacent_avg_incl(&image, x, y)).unwrap();
                // image.set_pixel(x, y, HSVA::new(n as f64, 100., 100., 100.)).unwrap();

                // print!("{} - Second loop: ", k);
                // println!("set_pixel: {}, {}", x, y);
            }


            for i in 0..(WIDTH-n) {
                x -= 1;
                k += 1;
                image.set_pixel(x, y, adjacent_avg_incl(&image, x, y)).unwrap();
                // image.set_pixel(x, y, HSVA::new(n as f64, 100., 100., 100.)).unwrap();

                // print!("{} - Third loop: ", k);
                // println!("set_pixel: {}, {}", x, y);
            }

            n+=1;
            if n > WIDTH {
                break;
            }

            for i in 0..(HEIGHT-n) {
                y -= 1;
                k += 1;
                // image.set_pixel(x, y, HSVA::new(n as f64, 100., 100., 100.)).unwrap();

                image.set_pixel(x, y, adjacent_avg_incl(&image, x, y)).unwrap();
                //
                // print!("{} - Fourth loop: ", k);
                // println!("set_pixel: {}, {}", x, y);
            }

        }

        image.to_rgb_image()

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
    if !in_bounds(image, x, y) {
        return None;
    } else if image.get_pixel(x, y).unwrap().a == 0. {
        return None;
    }
    image.get_pixel(x, y)
}

fn in_bounds(image: &HSVAImage, x: i32, y: i32) -> bool {
    x >= 0 && x < image.width && y >= 0 && y < image.height
}

fn random_hsv(a: f64) -> HSVA {
    let mut rng = rand::thread_rng();
    HSVA::new(rng.gen_range(0.0..360.0), rng.gen_range(25.0..=100.0), rng.gen_range(50.0..=100.0), a)
}
