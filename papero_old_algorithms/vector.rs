use crate::*;
use hue::images::*;
use hue::colours::*;
use hue::matrix::*;

pub(crate) struct Vector;

impl Plugin for Vector {
    fn create() -> raster::Image {
        let mut image1 = RgbaImage::new(WIDTH as usize, HEIGHT as usize, Rgba::black());

        for x in 0..(image1.width as u32) {
            for y in 0..(image1.height as u32) {
                let a = u32::MAX - (x*x-y);
                let a = a.to_be_bytes();
                let c1 = Rgba::new(a[0], a[1], a[2], 255);


                image1.set_pixel(x as usize, y as usize, c1).unwrap();
            }
        }
        image1.to_raster_image()
    }
}
fn create2() -> raster::Image {
    let mut image1 = RgbaImage::new(WIDTH as usize, HEIGHT as usize, Rgba::black());
    let mut image2 = RgbaImage::new(WIDTH as usize, HEIGHT as usize, Rgba::black());
    let mut image3 = RgbaImage::new(WIDTH as usize, HEIGHT as usize, Rgba::black());
    for x in 0..(image1.width as u32) {
        for y in 0..(image1.height as u32) {
            let a = (x*x).to_be_bytes();
            let c1 = Rgba::new(a[1], a[2], a[3], 255);
            let b = (100*y).to_be_bytes();
            let c2 = Rgba::new(b[1],b[2],b[3],255);

            let c = c1.clone()*c2.clone();

            image1.set_pixel(x as usize,y as usize,c).unwrap();
            image2.set_pixel(x as usize, y as usize, c1).unwrap();
            image3.set_pixel(x as usize, y as usize, c2).unwrap();
        }
    }
    image2.save("./out/1-cross1.png");
    image3.save("./out/1-cross2.png");
    image1.to_raster_image()

}
