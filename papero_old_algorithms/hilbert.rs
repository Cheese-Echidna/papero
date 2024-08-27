use crate::*;
use raster::*;
use fast_hilbert;
use num::PrimInt;

pub(crate) struct Hilbert;

impl Plugin for Hilbert {
    fn create() -> Image {
        if WIDTH > u16::MAX as i32 {
            panic!("TO Wide bruz")
        }
        let w = WIDTH as u16;
        let h = w.clone();

        let order = 100;
        let mut image = Image::blank(w as i32, h as i32);
        for x in 0..w {
            for y in 0..h {
                let n = fast_hilbert::xy2h(x,y,order);
                let c = util::hsva::HSVA::new(n as f64/1000.,n as f64/1000.,n as f64/1000.,100.0 - n as f64/100000.).to_rgb();

                image.set_pixel(x as i32, y as i32, c).unwrap();
            }
        }

        image
    }
}


pub fn u32_to_colour(x:u32) -> Color {
    let bytes:[u8; 4] = x.to_be_bytes();
    let r = bytes[0];
    let b = bytes[1];
    let g = bytes[2];
    let a = bytes[3];

    // println!("X:{:b}, r:{:b}, g:{:b}, b:{:b}, a:{:b}", x, r, b, g, a);
    raster::Color::rgba(r, g, b, 255)
}
