use std::f32::consts::TAU;
use crate::*;
use palette;
use palette::{convert, Clamp};

/// All in range [0,1]
pub(crate) fn convert_from_ok_hsl(h: f32, s:f32, l: f32) -> Rgb<f32> {
    let ok = palette::Okhsl::from_components((h*360.0, s, l));
    let un_clamped: palette::Srgb = convert::FromColorUnclamped::from_color_unclamped(ok);
    let srgb = un_clamped.clamp();
    // let srgb = un_clamped;
    Rgb([
        srgb.red,
        srgb.green,
        srgb.blue
    ])
}

/// All in range [0,1]
pub(crate) fn convert_from_ok_hsv(h: f32, s:f32, v: f32) -> Rgb<f32> {
    let ok = palette::Okhsv::from_components((h*360.0, s, v));
    let un_clamped: palette::Srgb = convert::FromColorUnclamped::from_color_unclamped(ok);
    // let srgb = un_clamped.clamp();
    let srgb = un_clamped;
    Rgb([
        srgb.red,
        srgb.green,
        srgb.blue
    ])
}

