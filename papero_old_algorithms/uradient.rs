use std::ops::{BitAnd, BitOr, BitXor};
use raster::Image;
use raster::Color;


const MULT: u32 = 16;
const WIDTH: u32 = 65536/MULT;

pub fn main() {
    let mut image = Image::blank(WIDTH as i32, WIDTH as i32);
    for i in 0..image.width as u32 {
        for j in 0..image.height as u32 {
            image.set_pixel(i as i32, j as i32, t2(i,j)).unwrap();
        }
        println!("row {} done", i)
    }
    save_image(&image, "grad-square");
    // _ = u32s_to_colour(65535, 65535);
}

fn u32s_to_colour(a:u32,b:u32) -> Color {
    let x = a*b;
    u32_to_colour(x)
    // let x = x.sh(8);
}

fn f32ification(a:u32, b:u32) -> Color {
    let a = (a*MULT) as f32;
    let b = (b*MULT) as f32;
    let c = a+b;

    let x:u32 = unsafe {std::mem::transmute(c)};
    u32_to_colour(x)
}

fn highway_to_infinity(a:u32, b:u32) -> Color {
    let x = a as f64 / WIDTH as f64;
    let y = b as f64 / WIDTH as f64;

    let m = ((10.0*x/y).tan()).abs();
    let v = (m*2_f64.powf(30.0)) as u32;
    u32_to_colour(v)
}

fn t2(a:u32, b:u32) -> Color {
    let x = a as f64 / WIDTH as f64;
    let y = b as f64 / WIDTH as f64;

    let m = (1.1*x.powf(3.0*y)).atanh().abs();
    let v = (m*2_f64.powf(30.0)) as u32;
    u32_to_colour(v)
}

fn polar(a:u32, b:u32) -> Color {
    let x = a as f64 / WIDTH as f64;
    let y = b as f64 / WIDTH as f64;
    let m = ((x*10.0).sin() * (y*10.0).cos()).abs();
    let v = (m*2_f64.powf(31.9)) as u32;
    u32_to_colour(v)
}


fn f64_colour(a:u32, b:u32) -> Color {
    let x = a as f64 / WIDTH as f64;
    let y = b as f64;
    let f = x.powf(y);
    let h = (a as u128).pow(b) as f64 / (WIDTH as u128).pow(b) as f64;;

    let r = (h - f + 0.5)*127.0;
    let g = 50;
    let b = 50;

    Color::rgba(r as u8, b as u8, g as u8, 255)
}


fn xor(a:u32, b:u32) -> Color {
    let c = a.bitxor(b);
    u32_to_colour(c as u32)
}

fn sq_xor(a:u32, b:u32) -> Color {
    let c = (a*MULT).pow(2).bitxor((b*MULT).pow(2));
    u32_to_colour(c as u32)
}

fn u24_to_colour(x:u32) -> Color {
    let x = x as u32;
    let bytes:[u8; 4] = x.to_be_bytes();
    let r = bytes[1];
    let b = bytes[2];
    let g = bytes[3];
    // println!("X:{:b}, r:{:b}, g:{:b}, b:{:b}, a:{:b}", x, r, b, g, a);
    Color::rgba(r as u8, b as u8, g as u8, 255)
}


fn u32_to_colour(x:u32) -> Color {
    let bytes:[u8; 4] = x.to_be_bytes();
    let r = bytes[0];
    let b = bytes[1];
    let g = bytes[2];

    // println!("X:{:b}, r:{:b}, g:{:b}, b:{:b}, a:{:b}", x, r, b, g, a);
    Color::rgba(r as u8, b as u8, g as u8, 255)
}


fn save_image(image: &Image, name: &str) {
    let filename = format!("out/{}.png", name);
    std::fs::create_dir_all("out").unwrap();
    raster::save(&image, &filename).unwrap();
}
