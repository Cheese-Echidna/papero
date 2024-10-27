use crate::*;

#[derive(Default)]
pub(crate) struct V;

impl Generator for V {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = RgbImage::new(args.width, args.height);

        for x in 0..(args.width) {
            for y in 0..(args.height) {
                let a = u32::MAX - (x*x-y);
                let a = a.to_be_bytes();
                let c1 = Rgb([a[0], a[1], a[2]]);


                image.put_pixel(x, y, c1);
            }
        }
        image.into()
    }

    fn name() -> &'static str {
        "Pixel Gradient"
    }
}
// fn create2() -> raster::Image {
//     let mut image1 = RgbaImage::new(WIDTH as usize, HEIGHT as usize, Rgba::black());
//     let mut image2 = RgbaImage::new(WIDTH as usize, HEIGHT as usize, Rgba::black());
//     let mut image3 = RgbaImage::new(WIDTH as usize, HEIGHT as usize, Rgba::black());
//     for x in 0..(image1.width as u32) {
//         for y in 0..(image1.height as u32) {
//             let a = (x*x).to_be_bytes();
//             let c1 = Rgba::new(a[1], a[2], a[3], 255);
//             let b = (100*y).to_be_bytes();
//             let c2 = Rgba::new(b[1],b[2],b[3],255);
//
//             let c = c1.clone()*c2.clone();
//
//             image1.set_pixel(x as usize,y as usize,c).unwrap();
//             image2.set_pixel(x as usize, y as usize, c1).unwrap();
//             image3.set_pixel(x as usize, y as usize, c2).unwrap();
//         }
//     }
//     image2.save("./out/1-cross1.png");
//     image3.save("./out/1-cross2.png");
//     image1.to_raster_image()
//
// }
