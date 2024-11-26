use image::{DynamicImage, GenericImageView, RgbaImage};

pub(crate) fn upscale(image: DynamicImage, n:u32) -> DynamicImage {
    let (old_w, old_h) =  (image.width(), image.height());
    let (w,h) = (old_w * n, old_h * n);
    let mut new_image = RgbaImage::new(w, h);
    for px in 0..old_w {
        for py in 0..old_h {
            let pixel = image.get_pixel(px, py);
            write_all(&mut new_image, &pixel, (px, py), n);
        }
    }
    new_image.into()
}

fn write_all(image: &mut RgbaImage, pixel: &image::Rgba<u8>, (px, py): (u32, u32), n:u32) {
    let (sx, sy) = (px * n, py * n);
    for dx in 0..n {
        for dy in 0..n {
            image.put_pixel(sx + dx, sy + dy, pixel.clone())
        }
    }
}