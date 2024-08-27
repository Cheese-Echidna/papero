use crate::*;
use raster::*;

pub(crate) struct Waterfall;

impl Plugin for Waterfall {
    fn create() -> Image {
        let mut image = Image::blank(WIDTH, HEIGHT);

        for y in 0..image.height {
            for x in 0..image.width {
                let x1 = (x as f64) / (image.width as f64);

                let color = if y == 0 {
                    Color::rgba((255. * x1.sin()) as u8, (255. * x1.cos()) as u8, (255. * x1.tan()) as u8, 255)
                } else {
                    let mut r = Vec::new();
                    let mut g = Vec::new();
                    let mut b = Vec::new();

                    for i in -1..2 {
                        if (x + i) == -1 {
                            r.push((image.get_pixel(image.width - 1, y - 1).unwrap().r as f64) / 255.);
                            g.push((image.get_pixel(image.width - 1, y - 1).unwrap().g as f64) / 255.);
                            b.push((image.get_pixel(image.width - 1, y - 1).unwrap().b as f64) / 255.);
                        } else if (x + i) == image.width {
                            r.push((image.get_pixel(0, y - 1).unwrap().r as f64) / 255.);
                            g.push((image.get_pixel(0, y - 1).unwrap().g as f64) / 255.);
                            b.push((image.get_pixel(0, y - 1).unwrap().b as f64) / 255.);
                        } else {
                            r.push((image.get_pixel(x + i, y - 1).unwrap().r as f64) / 255.);
                            g.push((image.get_pixel(x + i, y - 1).unwrap().g as f64) / 255.);
                            b.push((image.get_pixel(x + i, y - 1).unwrap().b as f64) / 255.);
                        }
                    }

                    let red = (0.3 * r[0] + 0.7 * r[1]) * 255.;
                    let green = 100. * g[0].powf(g[1]);
                    let blue = (b[2] + b[1]) * 127.5;
                    Color::rgba(red as u8, green as u8, blue as u8, 255)
                };
                image.set_pixel(x, y, color).unwrap();
            }
        }
        image
    }
}