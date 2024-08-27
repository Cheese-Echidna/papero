use hue::images::Hsva01Image;
use hue::colours::Hsva01;

struct Complex {
    re: f64,
    im: f64
}

impl Complex {
    fn new(re:f64, im:f64) -> Complex {
        Complex{re, im}
    }

    fn negate(&self) -> Complex {
        Complex::new(-self.re, -self.im)
    }

    fn plus(&self, other: &Complex) -> Complex {
        Complex::new(self.re+other.re, self.im+other.im)
    }

    fn minus(&self, other: &Complex) -> Complex {
        self.plus(&(other.negate()))
    }

    fn square(&self) -> Complex {
        let a = self.re;
        let b = self.im;
        Complex::new(a.powi(2) - b.powi(2), 2.0*a*b)
    }

    // squared euclidean distance
    fn sed(&self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }

    fn mag(&self) -> f64 {
        self.sed().sqrt()
    }

    fn clone(&self) -> Complex {
        Complex::new(self.re, self.im   )
    }

}

fn main() {
    let factor = 4;
    let width = 1920*factor;
    let height = 1080*factor;
    let thresh = 100.0;

    let mut min = f64::MAX;
    let mut max = 0.0;

    let mut image = Hsva01Image::new(width, height, Hsva01::black());
    for x in 0..image.width {
        for y in 0..image.height {
            let a = ((x as f64 / image.width as f64) * 2.0 - 1.0) * 1.5;
            let b = (((y as f64 / image.height as f64) * 2.0 - 1.0)*image.height as f64 / image.width as f64) * 1.5;
            let c = Complex::new(a,b);

            let iter_max = 10_u32.pow(3);
            let mut z = c.clone();
            for i in 0..iter_max {
                let mag = z.mag();
                // if mag > thresh {
                //     println!("({}, {}): {}", x, y, mag);
                    // break
                // }
                if mag > max {
                    max = mag
                }
                if mag < min {
                    min = mag
                }
                z = z.square().plus(&c);
            }
            let v = z.mag();
            let colour = Hsva01::new(1.0, 0.0, v, 1.0);
            image.set_pixel(x,y, colour).unwrap()
        }
    }
    println!("min: {}, max: {}", min, max);
    let r_image = image.to_rgba_image();
    let down_image = hue::images::downscale(r_image, factor);
    down_image.save("out.png").unwrap()
}
