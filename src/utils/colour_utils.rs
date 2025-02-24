use glam::{Vec3, Vec4};
use crate::*;
use palette;
use palette::{convert, Clamp, FromColor, IntoColor};
use rand::random;
use crate::utils::num_utils::lerp;

pub(crate) trait ImageColour<T: ColourType>: Sized {
    fn from_const(named: palette::Srgb<u8>) -> Self {
        Self::from_u8(Rgb([named.red, named.blue, named.green]))
    }
    fn from_u8(colour: impl ImageColour<u8>) -> Self;
    fn from_f32(colour: impl ImageColour<f32>) -> Self;
    fn with_alpha(self) -> Rgba<T>;
    fn with_alpha_of(self, a: T) -> Rgba<T>;
    fn without_alpha(self) -> Rgb<T>;
    fn to_u8(self) -> impl ImageColour<u8>;
    fn to_f32(self) -> impl ImageColour<f32>;
}

impl ColourType for f32 {
    fn max() -> Self {
        1.0
    }

    fn from_u8(x: u8) -> Self {
        x as f32 / 255.
    }

    fn from_f32(x: f32) -> Self {
        x
    }

    fn to_u8(self) -> u8 {
        u8::from_f32(self)
    }

    fn to_f32(self) -> f32 {
        self
    }
}

impl ColourType for u8 {
    fn max() -> Self {
        255
    }

    fn from_u8(x: u8) -> Self {
        x
    }

    fn from_f32(x: f32) -> Self {
        (x * 255.) as u8
    }

    fn to_u8(self) -> u8 {
        self
    }

    fn to_f32(self) -> f32 {
        f32::from_u8(self)
    }
}

trait ColourType: Sized {
    fn max() -> Self;
    fn from_u8(x: u8) -> Self;
    fn from_f32(x: f32) -> Self;
    fn to_u8(self) -> u8;
    fn to_f32(self) -> f32;
}

impl<T: ColourType> ImageColour<T> for image::Rgb<T> {
    fn from_u8(colour: impl ImageColour<u8>) -> Self {
        Rgb(colour.without_alpha().0.map(|x| T::from_u8(x)))
    }

    fn from_f32(colour: impl ImageColour<f32>) -> Self {
        Rgb(colour.without_alpha().0.map(|x| T::from_f32(x)))
    }


    fn with_alpha(self) -> image::Rgba<T> {
        let [r, g, b] = self.0;
        Rgba::<T>([r, g, b, T::max()])
    }

    fn with_alpha_of(self, a: T) -> Rgba<T> {
        let [r, g, b] = self.0;
        Rgba::<T>([r, g, b, a])
    }

    fn without_alpha(self) -> Rgb<T> {
        self
    }

    fn to_u8(self) -> impl ImageColour<u8> {
        Rgb(self.0.map(|x| x.to_u8()))
    }

    fn to_f32(self) -> impl ImageColour<f32> {
        Rgb(self.0.map(|x| x.to_f32()))
    }
}

impl<T: ColourType> ImageColour<T> for image::Rgba<T> {
    fn from_u8(colour: impl ImageColour<u8>) -> Self {
        Rgba(colour.with_alpha().0.map(|x| T::from_u8(x)))
    }

    fn from_f32(colour: impl ImageColour<f32>) -> Self {
        Rgba(colour.with_alpha().0.map(|x| T::from_f32(x)))
    }

    fn with_alpha(self) -> image::Rgba<T> {
        self
    }

    fn with_alpha_of(self, a: T) -> Rgba<T> {
        let mut new = self;
        new.0[3] = a;
        new
    }

    fn without_alpha(self) -> Rgb<T> {
        let [r, g, b, _] = self.0;
        Rgb([r, g, b])
    }

    fn to_u8(self) -> impl ImageColour<u8> {
        Rgba(self.0.map(|x| x.to_u8()))
    }

    fn to_f32(self) -> impl ImageColour<f32> {
        Rgba(self.0.map(|x| x.to_f32()))
    }
}

pub(crate) trait Colour3 {
    fn to_vec3(self) -> glam::Vec3;
    fn from_vec3(x: glam::Vec3) -> Self;
}

impl Colour3 for Rgb<f32> {
    fn to_vec3(self) -> Vec3 {
        Vec3::from_array(self.0)
    }

    fn from_vec3(x: Vec3) -> Self {
        Self::from(x.to_array())
    }
}

pub(crate) trait Colour4 {
    fn to_vec4(self) -> Vec4;
    fn from_vec4(x: Vec4) -> Self;
}

impl Colour4 for Rgba<f32> {
    fn to_vec4(self) -> Vec4 {
        Vec4::from_array(self.0)
    }

    fn from_vec4(x: Vec4) -> Self {
        Self::from(x.to_array())
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

pub(crate) fn random_colour() -> Rgb<f32> {
    Rgb([random(), random(), random()])
}

pub fn random_ok_rgb_f32() -> Rgb<f32> {
    let mut rng = rand::thread_rng();

    let mut rand = || rng.gen_range((0.0)..(1.0));

    colour_utils::convert_from_ok_hsl(rand(), rand(), rand())
}

pub fn random_pretty_ok() -> Rgb<f32> {
    let mut rng = rand::thread_rng();
    let mut range = || rng.gen_range(0.2_f32..0.8);

    colour_utils::convert_from_ok_hsl(range(), range(), range())
}