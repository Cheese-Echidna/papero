use crate::*;
use image::{RgbaImage, Rgba, GenericImageView, GenericImage};
use rand;
use rand::Rng;

pub(crate) struct Spiral {}

impl Generator for Spiral {
    fn gen_image(args: &Args) -> DynamicImage {
        let mut image = RgbaImage::new(args.width, args.height);
        
        let mut rng = rand::thread_rng();
        
        let (mut x, mut y) = (rng.gen_range((image.width()*2/5)..(image.width()*3/5)), rng.gen_range((image.height()/5)..(image.height()*4/5)));
        
        let initial_colour = random_rgb();
        image.put_pixel(x, y, initial_colour);
        
        for move_length in (1..args.width * 2).step_by(2) {
            for _right in 0..move_length {
                spiral_iteration(&mut image, (&mut x, &mut y), Direction::Right);
            }
            for _down in 0..move_length {
                spiral_iteration(&mut image, (&mut x, &mut y), Direction::Right);
            }
            for _left in 0..(move_length + 1) {
                spiral_iteration(&mut image, (&mut x, &mut y), Direction::Right);
            }
            for _up in 0..(move_length + 1) {
                spiral_iteration(&mut image, (&mut x, &mut y), Direction::Right);
            }
        }

        DynamicImage::ImageRgba8(image)
    }

    fn name() -> &'static str {
        "Spiral"
    }
}

fn spiral_iteration(image: &mut RgbaImage, (x,y): (&mut u32, &mut u32), dir: Direction) {
    match dir {
        Direction::Up => {*y = y.wrapping_sub(1)}
        Direction::Left => {*x = x.wrapping_sub(1)}
        Direction::Down => {*y += 1}
        Direction::Right => {*x += 1}
    }
    if !image.in_bounds(*x, *y) {
        return
    }
    let colour = adjacent_avg_incl(&image, *x, *y);
    image.put_pixel(*x, *y, colour);
}

enum Direction {
    Up,
    Left,
    Down,
    Right
}


pub fn random_rgb() -> Rgba<u8> {
    let mut rng = rand::thread_rng();

    let red = rng.gen_range(0..=255);
    let green = rng.gen_range(0..=255);
    let blue = rng.gen_range(0..=255);

    Rgba([red, green, blue, 255])
}

fn adjacent_avg_incl(image: &RgbaImage, x: u32, y: u32) -> Rgba<u8> {
    let mut rng = rand::thread_rng();

    let offsets = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    let (mut channels, count) = offsets.iter().filter_map(|(dx, dy)| {
        let (x, y) = (x.wrapping_add(*dx as u32), y.wrapping_add(*dy as u32));
        if image.in_bounds(x, y) {
            let pixel = image.get_pixel(x, y);
            if pixel.0[3] == 0 { // check for no alpha
                None
            } else {
                Some(pixel)
            }
        } else {
            None
        }
    }).fold(([0.0, 0.0, 0.0], 0), |(acc, i), pixel| {
        ([
            acc[0] + pixel.0[0] as f64,
            acc[1] + pixel.0[1] as f64,
            acc[2] + pixel.0[2] as f64,
        ], i + 1)
    });

    let denominator = count as f64;

    channels.iter_mut().for_each(|sum| *sum /= denominator);

    // For good multiplicative randomness
    // let min_mult = 0.95;
    // let max_mult = (min_mult + ((-min_mult * (3. * min_mult - 4.)) as f64).sqrt()) / (2. * min_mult);

    let min = -2.0;
    let max = 2.0;

    channels.iter_mut().for_each(|intensity| *intensity += rng.gen_range(min..=max));

    let c = channels.map(|value| value.round() as u8);

    Rgba([c[0], c[1], c[2], 255])
}