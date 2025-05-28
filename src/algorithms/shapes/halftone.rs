use std::f32::consts::TAU;
use std::path::PathBuf;
use glam::{Vec2, Vec3, Vec4};
use crate::algorithms::shapes::rotated_grid::RotatedGrid;
use image;
use image::{DynamicImage, Rgb, Rgba, Rgba32FImage};
use image::math::Rect;
use palette::stimulus::IntoStimulus;
use crate::algorithms::shapes::types::ball::Ball;
use crate::algorithms::shapes::types::shape_set::ShapeSet;
use crate::Generator;
use crate::utils::image_manager::Args;

#[derive(Default)]
pub(crate) struct Halftone {}

#[derive(Clone)]
pub(crate) struct CMYK(pub(crate) Vec4);

impl Generator for Halftone {
    fn generate(args: &Args) -> DynamicImage {
        let path = PathBuf::from("remus-test.jpg");
        shapes(args, path).generate(args, Some(Rgb([1.0, 1.0, 1.0])))
    }

    fn name() -> &'static str {
        "Colour Halftone"
    }
}

fn shapes(args: &Args, picture: PathBuf) -> ShapeSet<CMYK> {
    let image = image::open(picture).unwrap().to_rgba32f();
    let mut shape_set = ShapeSet::empty();
    let spacing = 10.;

    let angles: [f32; 4] = [0., radians(15.), radians(45.), radians(75.)];

    for (i, angle) in angles.into_iter().enumerate().rev() {
        if i == 3 {
            continue;
        }
        for point in RotatedGrid::new(
            (0.0, args.width as f32),
            (0.0, args.height as f32),
            spacing,
            angle,
        ) {
            //.powf(0.5) ...  * 0.55
            let max_radius = spacing * 0.55;
            let cmyk = rgb_to_cmyk(translate_pixel(point, &image, false, args));
            let radius = cmyk.0.to_array()[i].clamp(0.0, 1.0).sqrt() * max_radius;
            let circle = Ball::new(point, cmyk, radius);
            shape_set.add(circle);
        }
    }

    shape_set
}

fn translate_pixel(v: Vec2, picture: &Rgba32FImage, stretch: bool, args: &Args) -> Rgba<f32> {
    // `v` is a vec in range x: (0.0, args.width as f32), y: (0.0, args.height as f32),
    // if stretch is true then the `picture` should be streched to fit the dimensions of output (args.width, args.height)
    // if not it should be scaled to fit (and centered)
    let (pic_width, pic_height) = (picture.width() as f32, picture.height() as f32);
    let (out_width, out_height) = (args.width as f32, args.height as f32);

    let (x, y) = if stretch {
        (
            v.x * pic_width / out_width,
            v.y * pic_height / out_height,
        )
    } else {
        let scale = (out_width / pic_width).min(out_height / pic_height);
        let scaled_width = pic_width * scale;
        let scaled_height = pic_height * scale;
        let offset_x = (out_width - scaled_width) / 2.0;
        let offset_y = (out_height - scaled_height) / 2.0;
        (
            (v.x - offset_x) / scale,
            (v.y - offset_y) / scale,
        )
    };

    let px = x.round() as u32;
    let py = y.round() as u32;

    if px >= picture.width() || py >= picture.height() || x.round() < 0.  || y.round() < 0. {
        Rgba([1.0, 1.0, 1.0, 1.0])
    } else {
        picture.get_pixel(px, py).clone()
    }
}

fn rgb_to_cmyk(colour: Rgba<f32>) -> CMYK {
    let [r, g, b, _] = colour.0;

    let k = 1.0 - r.max(g).max(b);
    if k >= 1.0 {
        return CMYK(Vec4::from_array([0.0, 0.0, 0.0, 1.0]));
    }

    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);

    CMYK(Vec4::from_array([c, m, y, k]))
}

// deg to rad
fn radians(x: f32) -> f32 {
    x / 360. * TAU
}
