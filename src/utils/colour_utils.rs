use std::f32::consts::TAU;
use crate::*;
use palette;
use palette::{convert, Clamp};
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