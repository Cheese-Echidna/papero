use crate::*;
use indicatif;

pub fn create(config: Config) -> RgbaImage {
    let mut rng = rand::thread_rng();

    let mut image = RgbaImage::transparent(config.width, config.height);
    let mut pixels_left = vec![];
    for x in 0..config.width {
        for y in 0..config.height {
            pixels_left.push((x,y));
        }
    }

    for _ in 0..10 {
        let (init_x, init_y) = (rng.gen_range((image.width*2/5)..(image.width*3/5)), rng.gen_range((image.height/5)..(image.height*4/5)));
        let initial_colour = algorithms::spiral::random_rgb();
        image.set_pixel(init_x, init_y, initial_colour).unwrap();
        // pixels_left -= 1;
    }

    while pixels_left.len() != 0 {
        // image.ge
    }

    image

}