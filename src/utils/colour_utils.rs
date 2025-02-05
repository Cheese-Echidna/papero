use crate::*;
use palette;
use palette::{convert, Clamp, FromColor, IntoColor};
use rand::random;
use crate::utils::num_utils::lerp;

pub(crate) trait ImageColour<T: ColourType>: Sized {
    fn from_const(named: palette::Srgb<u8>) -> Self {
        Self::from_srgb(Rgb([named.red, named.blue, named.green]))
    }
    fn from_srgb(colour: image::Rgb<u8>) -> Self;
    fn with_alpha(self) -> Rgba<T>;
}

impl ColourType for f32 {
    fn max() -> Self {
        1.0
    }

    fn from_u8(x: u8) -> Self {
        x as f32 / 255.
    }
}

impl ColourType for u8 {
    fn max() -> Self {
        255
    }

    fn from_u8(x: u8) -> Self {
        x
    }
}

trait ColourType: Sized {
    fn max() -> Self;
    fn from_u8(x: u8) -> Self;
}

impl<T: ColourType> ImageColour<T> for image::Rgb<T> {
    fn from_srgb(colour: Rgb<u8>) -> Self {
        Rgb::<T>(colour.0.map(|v| T::from_u8(v)))
    }

    fn with_alpha(self) -> image::Rgba<T> {
        let [r, g, b] = self.0;
        Rgba::<T>([r, g, b, T::max()])
    }
}

impl<T: ColourType> ImageColour<T> for image::Rgba<T> {
    fn from_srgb(colour: Rgb<u8>) -> Self {
        Rgb::<T>(colour.0.map(|v| T::from_u8(v))).with_alpha()
    }

    fn with_alpha(self) -> image::Rgba<T> {
        self
    }
}

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

pub(crate) fn custom_transition_gradient(x: f32, y: f32) -> Rgb<f32> {
    let h = lerp(y, 180., 265.) / 360.;
    let s = 1.;
    let l = x * 0.6 + 0.1;
    convert_from_ok_hsl(h, s, l)
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

pub(crate) fn into_f32<C>(color: C) -> image::Rgb<f32>
where
    C: IntoColor<palette::Srgb<f32>>, // Ensure the color can be converted into Srgba
{
    // Convert the input color to sRGBA (standard RGBA with floating-point components)
    let srgba: palette::Srgb = color.into_color();

    // Clamp each component to [0.0, 1.0] to avoid overflow/underflow
    let clamped = srgba.clamp();

    image::Rgb::from([clamped.red, clamped.green, clamped.blue])
}


pub(crate) fn into_no_alpha<C>(color: C) -> image::Rgb<u8>
where
    C: IntoColor<palette::Srgb>, // Ensure the color can be converted into Srgba
{
    // Convert the input color to sRGBA (standard RGBA with floating-point components)
    let srgba: palette::Srgb = color.into_color();

    // Clamp each component to [0.0, 1.0] to avoid overflow/underflow
    let clamped = srgba.clamp();

    // Scale each component from [0.0, 1.0] to [0, 255] and cast to u8
    let r = (clamped.red * 255.0).round() as u8;
    let g = (clamped.green * 255.0).round() as u8;
    let b = (clamped.blue * 255.0).round() as u8;

    image::Rgb([r, g, b])
}

pub(crate) fn random_colour() -> Rgba<f32> {
    Rgba::from([random(), random(), random(), 1.0])
}

pub fn random_rgb() -> Rgba<u8> {
    let mut rng = rand::thread_rng();

    let red = rng.gen_range(0..=255);
    let green = rng.gen_range(0..=255);
    let blue = rng.gen_range(0..=255);

    Rgba([red, green, blue, 255])
}


pub fn random_rgb_f32() -> Rgba<f32> {
    let mut rng = rand::thread_rng();

    let mut rand = || rng.gen_range((0.0)..(1.0));

    Rgba([rand(), rand(), rand(), 1.0])
}

pub fn random_ok_rgb_f32() -> Rgb<f32> {
    let mut rng = rand::thread_rng();

    let mut rand = || rng.gen_range((0.0)..(1.0));

    colour_utils::convert_from_ok_hsl(rand(), rand(), rand())
}

pub fn random_pretty_ok() -> Rgb<f32> {
    let mut rng = rand::thread_rng();

    colour_utils::convert_from_ok_hsl(rng.gen_range((0.0)..(1.0)), rng.gen_range((0.2)..(0.8)), rng.gen_range((0.2)..(0.8)))
}