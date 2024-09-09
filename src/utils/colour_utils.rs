use std::f32::consts::TAU;
use crate::*;
use palette;
use palette::convert;

pub(crate) fn convert_from_ok_hsl(h: f32, s:f32, l: f32) -> Rgb<f32> {
    let ok = palette::Okhsl::from_components((h*360.0, s, l));
    let srgb: palette::Srgb = convert::FromColorUnclamped::from_color_unclamped(ok);
    Rgb([
        srgb.red,
        srgb.green,
        srgb.blue
    ])
}