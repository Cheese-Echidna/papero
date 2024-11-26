use std::f32::consts::TAU;
use crate::*;
use palette;
use palette::{convert, Clamp, Hsva, FromColor, IntoColor};
use rand::random;
use crate::utils::num_utils::lerp;

/// All in range [0,1]
pub(crate) fn convert_from_ok_hsl(h: f32, s:f32, l: f32) -> Rgb<f32> {
    let ok = palette::Okhsl::from_components((h*360.0, s, l));
    let srgb: palette::Srgb = convert::FromColorUnclamped::from_color_unclamped(ok);
    let srgb = srgb.clamp();
    Rgb([
        srgb.red,
        srgb.green,
        srgb.blue
    ])
}

/// All in range [0,1]
pub(crate) fn convert_from_ok_hsv(h: f32, s:f32, v: f32) -> Rgb<f32> {
    let ok = palette::Okhsv::from_components((h*360.0, s, v));
    let srgb: palette::Srgb = convert::FromColorUnclamped::from_color_unclamped(ok);
    let srgb = srgb.clamp();

    Rgb([
        srgb.red,
        srgb.green,
        srgb.blue
    ])
}

pub(crate) fn sick_gradient(x: f32, y: f32) -> Rgb<f32> {
    convert_from_ok_hsl(lerp(x, 0.0, 0.33), 0.75, lerp(y, 0.2, 0.7))
}

pub(crate) fn into<C>(color: C) -> image::Rgba<u8>
where
    C: IntoColor<palette::Srgba>, // Ensure the color can be converted into Srgba
{
    // Convert the input color to sRGBA (standard RGBA with floating-point components)
    let srgba: palette::Srgba = color.into_color();

    // Clamp each component to [0.0, 1.0] to avoid overflow/underflow
    let clamped = srgba.clamp();

    // Scale each component from [0.0, 1.0] to [0, 255] and cast to u8
    let r = (clamped.red * 255.0).round() as u8;
    let g = (clamped.green * 255.0).round() as u8;
    let b = (clamped.blue * 255.0).round() as u8;
    let a = (clamped.alpha * 255.0).round() as u8;

    image::Rgba([r, g, b, a])
}

pub(crate) fn random_colour() -> Rgba<f32> {
    Rgba::from([random(), random(), random(), 1.0])
}