use crate::*;

pub struct Isak {}

impl Generator for Isak {
    fn gen_image(args: &Args) -> DynamicImage {
        let mut rng = rand::thread_rng();

        let mut image = RgbaImage::new(args.width, args.height);
        let mut pixels_left = vec![];
        for x in 0..args.width {
            for y in 0..args.height {
                pixels_left.push((x,y));
            }
        }

        for _ in 0..10 {
            let (init_x, init_y) = (rng.gen_range((args.width*2/5)..(args.width*3/5)), rng.gen_range((args.height/5)..(args.height*4/5)));
            let initial_colour = algorithms::spiral::random_rgb();
            image.put_pixel(init_x, init_y, initial_colour);
            // pixels_left -= 1;
        }

        // while pixels_left.len() != 0 {
        //     // image.ge
        // }

        image.into()
    }

    fn name() -> &'static str {
        "Isak"
    }
}
