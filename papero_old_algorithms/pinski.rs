use crate::*;

type U24 = u32;

pub fn create(config: Config) -> RgbaImage {
    let mut image = RgbaImage::new(config.width, config.height, Rgba::black());

    for i in 0..(config.width/3) {
        image.set_pixel(i*3,0,Rgba::white()).unwrap();
    }

    // image.set_pixel(config.width/2-1, 0, Rgba::new(255,255, 255,255));

    for row in 1..image.height {
        for mid in 0..image.width {
            let right = mid + 1;
            let left = mid - 1;
            let left_v = u24_from_colour(&image.get_pixel(left, row-1).unwrap_or(Rgba::black()));
            let right_v = u24_from_colour(&image.get_pixel(right, row-1).unwrap_or(Rgba::black()));
            let sum = left_v + right_v;

            // let c = if sum%2 == 1 {
            //     Rgba::white()
            // } else {
            //     Rgba::black()
            // };

            // println!("({}, {}):  {} + {} = {}", mid, row, left_v, right_v, sum);
            let c = colour_from_u24(sum);
            image.set_pixel(mid,row,c).unwrap()
        }
    }

    return image

    // let mut new_image = RgbaImage::new(config.width, config.height, Rgba::black());
    // for x in 0..config.width {
    //     for y in 0..config.height {
    //         let k = if y%2 == 1 {
    //             1
    //         } else {
    //             0
    //         };
    //         new_image.set_pixel(x, y, image.get_pixel(2*x+k,y).unwrap()).unwrap()
    //     }
    // }
    //
    // return new_image
}

pub fn create2(config: Config) -> RgbaImage {
    let mut image = Hsva01Image::new(config.width, config.height, Hsva01::new(0.0,1.0,1.0,1.0));
    image.set_pixel(config.width/2-1, 0, Hsva01::new(0.5,1.0,1.0,1.0));
    for row in 1..image.height {
        for mid in 0..image.width {
            let right = mid + 1;
            let left = mid - 1;
            let left_v = image.get_pixel(left, row-1).unwrap_or(Hsva01::new(0.0,1.0,0.0,1.0)).h;
            let right_v = image.get_pixel(right, row-1).unwrap_or(Hsva01::new(0.0,1.0,0.0,1.0)).h;
            let h_new = angle_mean_01(left_v, right_v);
            // let c = if sum == 1 {
            //     Rgba::white()
            // } else {
            //     Rgba::black()
            // };

            // println!("({}, {}):  {} + {} = {}", mid, row, left_v, right_v, sum);
            let c = Hsva01::new(h_new, 1.0, 1.0, 1.0);
            image.set_pixel(mid,row,c).unwrap()
        }
    }
    return image.to_rgba_image()
}

pub fn name() -> String {
    "Sierpiński's Triangle".to_string()
}

fn u24_from_colour(c: &Rgba) -> U24 {
    let a = c.r as u32;
    let b = c.g as u32;
    let c = c.b as u32;
    let x = (a << 2*8) + (b << 1*8) + c;
    return x
}

fn colour_from_u24(x:U24) -> Rgba {
    let r = ((x & 0xff0000) >> 2*8) as u8;
    let g = ((x & 0x00ff00) >> 1*8) as u8;
    let b = (x & 0x0000ff) as u8;

    return Rgba::new(r,g,b,255);
}

fn angle_mean_01(a:f64, b:f64) -> f64 {
    let x = zero_one_to_rad(a).cos() + zero_one_to_rad(b).cos() / 2.0;
    let y = zero_one_to_rad(a).sin() + zero_one_to_rad(b).sin() / 2.0;
    let ang = y.atan2(x);
    return rad_to_01(ang)
}

fn rad_to_01(rad:f64) -> f64 {
    return rad / (2.0 * std::f64::consts::PI)
}

fn zero_one_to_rad(zero_one:f64) -> f64 {
    return zero_one * (2.0 * std::f64::consts::PI)
}


#[cfg(test)]
mod tests {
    use hue::colours::Rgba;
    use crate::algorithms::pinski::{colour_from_u24, u24_from_colour};

    #[test]
    fn u24_cast_in() {
        let starter = Rgba::white();
        let x = u24_from_colour(&starter);
        assert_eq!(x, 0xff_ff_ff)
    }

    #[test]
    fn u24_cast_out() {
        let finisher = Rgba::white();
        let x = colour_from_u24(0xff_ff_ff);
        assert_eq!(x, finisher)
    }

    #[test]
    fn u24_cast_out_red() {
        let finisher = Rgba::red();
        let x = colour_from_u24(0xff_00_00);
        assert_eq!(x, finisher)
    }

    #[test]
    fn u24_cast_in_out_all() {
        for i in 0..=0xffffff {
            let x = colour_from_u24(i);
            let y = u24_from_colour(&x);
            assert_eq!(y, i)
        }
    }

    #[test]
    fn u24_cast_out_in_all() {
        for r in 0..=0xff {
            for g in 0..=0xff {
                for b in 0..=0xff {
                    let x = Rgba::new(r,g,b,255);
                    let y = u24_from_colour(&x);
                    let c = colour_from_u24(y);
                    assert_eq!(x,c);
                }
            }
        }
    }

}