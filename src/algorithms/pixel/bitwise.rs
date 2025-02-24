use crate::*;
use image::DynamicImage;
use std::cmp::min;
use std::ops::BitXor;

const MULT: u32 = 16;
const WIDTH: u32 = 65536 / MULT;

#[derive(Default)]
pub(crate) struct Bitwise;

impl Generator for Bitwise {
    fn generate(args: &Args) -> DynamicImage {
        let mut image = RgbImage::new(args.width, args.height);
        let mult = (2_u32 << 12) / min(args.width, args.height);
        for x in 0..args.width {
            for y in 0..args.height {
                let (px, py) = (
                    x as i32 - args.width as i32 / 2,
                    y as i32 - args.height as i32 / 2,
                );
                image.put_pixel(x, y, sq_xor(px, py, mult as i32));
            }
        }

        image.into()
    }

    fn name() -> &'static str {
        "Bitwise Image Magic"
    }
}

fn u32s_to_colour(a: u32, b: u32) -> Rgb<u8> {
    let x = a * b;
    u32_to_colour(x)
    // let x = x.sh(8);
}

fn f32ification(a: u32, b: u32) -> Rgb<u8> {
    let a = (a * MULT) as f32;
    let b = (b * MULT) as f32;
    let c = a + b;

    let x: u32 = c.to_bits();
    u32_to_colour(x)
}

fn highway_to_infinity(a: u32, b: u32) -> Rgb<u8> {
    let x = a as f64 / WIDTH as f64;
    let y = b as f64 / WIDTH as f64;

    let m = ((10.0 * x / y).tan()).abs();
    let v = (m * 2_f64.powf(30.0)) as u32;
    u32_to_colour(v)
}

fn t2(a: u32, b: u32) -> Rgb<u8> {
    let x = a as f64 / WIDTH as f64;
    let y = b as f64 / WIDTH as f64;

    let m = (1.1 * x.powf(3.0 * y)).atanh().abs();
    let v = (m * 2_f64.powf(30.0)) as u32;
    u32_to_colour(v)
}

fn polar(a: u32, b: u32) -> Rgb<u8> {
    let x = a as f64 / WIDTH as f64;
    let y = b as f64 / WIDTH as f64;
    let m = ((x * 10.0).sin() * (y * 10.0).cos()).abs();
    let v = (m * 2_f64.powf(31.9)) as u32;
    u32_to_colour(v)
}

fn f64_colour(a: u32, b: u32) -> Rgb<u8> {
    let x = a as f64 / WIDTH as f64;
    let y = b as f64;
    let f = x.powf(y);
    let h = (a as u128).pow(b) as f64 / (WIDTH as u128).pow(b) as f64;

    let r = (h - f + 0.5) * 127.0;
    let g = 50;
    let b = 50;

    Rgb([r as u8, b as u8, g as u8])
}

fn xor(a: u32, b: u32) -> Rgb<u8> {
    let c = a.bitxor(b);
    u32_to_colour(c)
}

fn sq_xor(a: i32, b: i32, mult: i32) -> Rgb<u8> {
    let c = (a * mult).pow(2).bitxor((b * mult).pow(2));
    u24_to_colour(c)
}

fn u24_to_colour(x: i32) -> Rgb<u8> {
    let bytes: [u8; 4] = x.to_be_bytes();
    let r = bytes[1];
    let b = bytes[2];
    let g = bytes[3];
    Rgb([r, b, g])
}

fn u32_to_colour(x: u32) -> Rgb<u8> {
    let bytes: [u8; 4] = x.to_be_bytes();
    let r = bytes[0];
    let b = bytes[1];
    let g = bytes[2];

    // println!("X:{:b}, r:{:b}, g:{:b}, b:{:b}, a:{:b}", x, r, b, g, a);
    Rgb([r, b, g])
}
