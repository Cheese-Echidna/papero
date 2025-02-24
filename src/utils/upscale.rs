#![allow(dead_code)]
use glam::Vec4;
use image::{DynamicImage, GenericImageView, Rgba32FImage, RgbaImage, Rgba};
use crate::utils::colour_utils::Colour4;

pub(crate) fn upscale(image: DynamicImage, n:u32) -> DynamicImage {
    let (old_w, old_h) =  (image.width(), image.height());
    let (w,h) = (old_w * n, old_h * n);
    let mut new_image = RgbaImage::new(w, h);
    for px in 0..old_w {
        for py in 0..old_h {
            let pixel = image.get_pixel(px, py);
            write_all(&mut new_image, pixel, (px, py), n);
        }
    }
    new_image.into()
}

pub fn downscale(image: DynamicImage, n:u32) -> DynamicImage {
    let old_image = image.to_rgba32f();
    let (w,h) = image.dimensions();
    let mut new_image = Rgba32FImage::new(w / n, h / n);
    for px in 0..(w / n) {
        for py in 0..(h / n) {
            // Loop through all the pixels to average
            let mut total = Vec4::ZERO;
            for dx in 0..n {
                for dy in 0..n {
                    let pixel = old_image.get_pixel(px * n + dx, py * n + dy).to_vec4();
                    total += pixel;
                }
            }
            let colour = Rgba::<f32>::from_vec4(total / (n * n) as f32);

            new_image.put_pixel(px, py, colour);
        }
    }
    new_image.into()
}

fn write_all(image: &mut RgbaImage, pixel: image::Rgba<u8>, (px, py): (u32, u32), n:u32) {
    let (sx, sy) = (px * n, py * n);
    for dx in 0..n {
        for dy in 0..n {
            image.put_pixel(sx + dx, sy + dy, pixel);
        }
    }
}