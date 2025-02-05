use crate::utils::colour_utils::ImageColour;
use crate::utils::image_manager::Args;
use crate::{utils, Generator};
use glam::Vec4;
use image::{DynamicImage, GenericImageView, Pixel, Rgb, Rgba, Rgba32FImage};
use palette::named::BLACK;



// THIS NEVER FINISHES???

const RANDOMNESS: f64 = 0.98;

#[derive(Default)]
pub(crate) struct Maze;

impl Generator for Maze {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = args.image_f32_alpha(Rgba::from_const(BLACK));

        let mut x: u32 = 0;
        let mut y: u32 = 0;

        let mut d: u32 = 1;
        let mut counter: u32 = 0;

        let (width, height) = args.wh();
        let starting_colour = utils::colour_utils::random_colour().with_alpha();

        let try_put = |image: &mut Rgba32FImage, x: u32, y: u32, colour: Rgba<f32>| {
            if image.in_bounds(x, y) {
                image.put_pixel(x, y, colour);
                true
            } else {
                false
            }
        };

        try_put(&mut image, x, y, starting_colour);

        for _i in 0..(width - d) {
            x += 1;
            counter += 1;
            try_put(&mut image, x, y, starting_colour);
        }

        for i in 0..(height - d) {
            y += 1;
            counter += 1;
            try_put(&mut image, x, y, starting_colour);
        }

        for i in 0..(width - d) {
            x -= 1;
            counter += 1;
            try_put(&mut image, x, y, starting_colour);
        }

        d += 1;
        for i in 0..(height - d) {
            y -= 1;
            counter += 1;

            try_put(&mut image, x, y, starting_colour);
        }

        loop {
            for _i in 0..(width - d) {
                x += 1;
                counter += 1;
                let c = adjacent_avg_incl(&image, x, y);
                let c = adjacent_avg_incl(&image, x, y);
                try_put(&mut image, x, y, c);
                try_put(&mut image, x, y, c);
            }

            d += 1;
            if d > width {
                break;
            }

            for _i in 0..(height - d) {
                y += 1;
                counter += 1;
                let c = adjacent_avg_incl(&image, x, y);
                try_put(&mut image, x, y, c);
            }

            for _i in 0..(width - d) {
                x -= 1;
                counter += 1;
                let c = adjacent_avg_incl(&image, x, y);
                try_put(&mut image, x, y, c);
            }

            d += 1;
            if d > width {
                break;
            }

            for _i in 0..(height - d) {
                y -= 1;
                counter += 1;
                let c = adjacent_avg_incl(&image, x, y);
                try_put(&mut image, x, y, c);
            }
        }

        image.into()
    }

    fn name() -> &'static str {
        "Maze"
    }
}

pub(crate) fn adjacent_avg_incl(image: &Rgba32FImage, x: u32, y: u32) -> Rgba<f32> {
    let mut rng = rand::thread_rng();

    let offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let (count, mut channels) = offsets
        .iter()
        .filter_map(|(dx, dy)| {
            let (x, y) = (x.wrapping_add(*dx as u32), y.wrapping_add(*dy as u32));
            if image.in_bounds(x, y) {
                let pixel = image.get_pixel(x, y);
                if pixel.0[3] == 0. {
                    // check for no alpha
                    None
                } else {
                    Some(pixel)
                }
            } else {
                None
            }
        })
        .fold((0, Vec4::ZERO), |(i, acc), pixel| {
            (i + 1, acc + Vec4::from_array(pixel.0))
        });

    let denominator = count as f32;

    channels.as_mut().iter_mut().for_each(|x| *x /= denominator);

    use rand_distr::Distribution;
    let distribution = rand_distr::Normal::new(0.0, 1.3).unwrap();

    channels
        .as_mut()
        .iter_mut()
        .for_each(|intensity| *intensity += distribution.sample(&mut rng));

    Rgba(channels.to_array())
}
