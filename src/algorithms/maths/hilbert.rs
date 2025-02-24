use crate::*;

#[derive(Default)]
pub(crate) struct Hilbert;

impl Generator for Hilbert {
    fn generate(args: &Args) -> DynamicImage {
        let (width, height) = (args.width, args.height);
        if width > u16::MAX as u32 {
            panic!("Too wide")
        }

        let mut image = RgbaImage::new(width, height);
        for x in 0..width {
            for y in 0..height {
                image.put_pixel(x, y, Rgba([0,0,0,255]))
            }
        }

        let dim = std::cmp::max(width, height);
        // (10, 63), (12, 127), (20, 2047)
        let order = (2.0 * ((dim as f64).log2() - 1.0)).ceil() as u8;

        let total = 2.0_f64.powi(order as i32) - 1.0/2.0_f64.powi(order as i32);

        let c = |t: f32| palette::Okhsl::new(t / total as f32 * 360., 1.0, 0.65);

        let mut prev: Option<(u32, u32, u64)> = None;
        for t in 0..=(total as u64) {
            let (x1, y1) = fast_hilbert::h2xy::<u32>(t, order);
            let (x,y) = (x1 * 2, y1 * 2);
            if image.in_bounds(x,y) {
                image.put_pixel(x, y, into(c(t as f32)));
            }
            if let Some((prev_x, prev_y, prev_t)) = prev {
                let (mix_x, mix_y, mix_t) = ((x + prev_x)/2, (y+prev_y)/2, (t as f32 + prev_t as f32)/2.);
                if image.in_bounds(mix_x,mix_y) {
                    image.put_pixel(mix_x, mix_y, into(c(mix_t)));
                }
            }

            prev = Some((x,y,t))
        }

        image.into()
    }

    fn name() -> &'static str {
        "Hilbert Curve"
    }
}