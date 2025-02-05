use std::f32::consts::{FRAC_PI_2, TAU};
use glam::{UVec2, UVec4, Vec2, Vec4};
use image::{DynamicImage, GenericImageView, Rgba, Rgba32FImage};
use rand::{random, Rng};
use palette::named::BLACK;
use crate::{Args, Generator};
use crate::utils::colour_utils;
use crate::utils::colour_utils::ImageColour;

const NOISE_SCALE: f64 = 0.002;
const FORCE_SCALE: f32 = 1.;

const DENSITY: f32 = 0.002;

struct Particle {
    prev_pos: Vec2,
    pos: Vec2,
    dead: bool,
    col: Rgba<f32>,
    lifetime: usize,
}

impl Particle {
    fn new(pos: Vec2, lifetime: usize, col: Rgba<f32>) -> Self {
        Self {
            prev_pos: pos,
            pos,
            dead: false,
            col,
            lifetime,
        }
    }

    fn update(&mut self, size: (u32, u32), seed: u32) {
        let (w, h) = (size.0 as f32, size.1 as f32);
        if self.pos.x >= w || self.pos.x < 0. || self.pos.y >= h || self.pos.y < 0. {
            self.dead = true;
        }

        self.lifetime -= 1;
        if self.lifetime == 0 {
            self.dead = true;
        }

        if self.dead {
            return;
        }

        let force = force_at(self.pos, seed);

        self.prev_pos = self.pos;
        self.pos += force * FORCE_SCALE;
    }

    fn step(&mut self, image: &mut Rgba32FImage) {
        // println!("{:?} prev", self.prev_pos);
        // println!("{:?} now", self.pos);
        // draw_xiaolin_wu(image, self.prev_pos, self.pos, self.col);
        // println!("drawn");

        // plot_line(image, self.prev_pos, self.pos, self.col);
        let [x, y] = self.pos.as_uvec2().to_array();
        if image.in_bounds(x, y) {
            let prev = image.get_pixel(x, y);
            let new = blend(*prev, self.col);
            image.put_pixel(x, y, new)
        }
    }
}

struct ParticleSet {
    particles: Vec<Particle>,
    dead: bool,
    seed: u32,
}

impl ParticleSet {
    fn new(n: usize, lifetime: usize, args: &Args) -> Self {
        let size = args.wh();

        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..n {
            let px = rng.gen_range(0.0..(size.0 as f32));
            let py = rng.gen_range(0.0..(size.1 as f32));

            // Gabe stuff
            let col = colour_utils::sick_gradient(px / size.0 as f32, py / size.1 as f32).with_alpha_of(0.35);

            particles.push(Particle::new(Vec2::new(px, py), lifetime, col));
        }

        let seed = rng.gen_range(0..u32::MAX);

        Self {
            particles,
            dead: false,
            seed,
        }
    }

    fn update(&mut self, size: (u32, u32)) {
        self.dead = true;
        for p in &mut self.particles {
            if !p.dead {
                self.dead = false;
                p.update(size, self.seed);
            }
        }
    }

    fn draw(&mut self, image: &mut Rgba32FImage) {
        for p in &mut self.particles {
            if !p.dead {
                p.step(image);
            }
        }
    }

    fn alive(&self) -> usize {
        self.particles.iter().filter(|p| !p.dead).count()
    }


    // main loop
    fn run(&mut self, args: &Args) -> DynamicImage {
        let mut image = args.image_f32_alpha(Rgba::<f32>::from_const(BLACK));

        let mut counter = 0;
        while !self.dead {
            counter += 1;

            self.update(args.wh());
            self.draw(&mut image);

            let alive_prop = self.alive() as f32 / self.particles.len() as f32;
            if counter % 100 == 0 {
                println!("i: {}, {:.2}% particles", counter, alive_prop * 100.);
            }
        }
        image.into()
    }
}

#[derive(Default)]
pub struct Flow;

impl Generator for Flow {
    fn generate(args: &Args) -> DynamicImage {
        let (width, height) = (args.width as usize, args.height as usize);

        let lifetime = width + height;

        let mut particles = ParticleSet::new(((width * height) as f32 * DENSITY) as usize, lifetime, &args);

        particles.run(args)
    }

    fn name() -> &'static str {
        "Flow"
    }
}

// todo use blend function
fn blend(bg: Rgba<f32>, fg: Rgba<f32>) -> Rgba<f32> {
    let bg = Vec4::from_array(bg.0);
    let fg = Vec4::from_array(fg.0);

    if fg.w == 0. {
        return Rgba(bg.to_array());
    }
    if fg.w == 1. {
        return Rgba(fg.to_array());
    }

    let alpha_final = bg.w + fg.w - bg.w * fg.w;
    if alpha_final == 0. {
        return Rgba(Vec4::ZERO.to_array());
    }

    let (bg_r_a, bg_g_a, bg_b_a) = (bg.x * bg.w, bg.y * bg.w, bg.z * bg.w);
    let (fg_r_a, fg_g_a, fg_b_a) = (fg.x * fg.w, fg.y * fg.w, fg.z * fg.w);

    let (out_r_a, out_g_a, out_b_a) = (
        fg_r_a + bg_r_a * (1.0 - fg.w),
        fg_g_a + bg_g_a * (1.0 - fg.w),
        fg_b_a + bg_b_a * (1.0 - fg.w),
    );

    let (out_r, out_g, out_b) = (
        out_r_a / alpha_final,
        out_g_a / alpha_final,
        out_b_a / alpha_final,
    );

    Rgba([out_r, out_g, out_b, alpha_final])
}

// // let seed = 4;
// // let perlin = Perlin::new(seed as u32);
// let perlin = |x: [f32; 2]| {
//     fbm(seed, 0.5, 5, x)
// };
//
// let mut noise: Vec<Vec<f32>> = Vec::new();
//
// let mut noise_image = RgbaImage::new(size.0, size.1, BLACK);
//
// for y in 0..size.1 {
//     let mut row: Vec<f32> = Vec::new();
//     for x in 0..size.0 {
//         // I changed the next line so that val element [0,1] instead of [-1,1]
//         let val = (perlin([x as f32 * NOISE_SCALE/args.multiplier as f32, y as f32 * NOISE_SCALE/args.multiplier as f32]) + 1.0) / 2.0;
//         noise_image.set_pixel(x, y, Hsva::new(val, 1.0, 1.0, 1.0).to::<Rgba>());
//         row.push(val);
//     }
//     noise.push(row);
// }
//
// noise_image.save("perlin.png");

fn force_at(pos: Vec2, seed: u32) -> Vec2 {
    let noise = crate::noise::fbm(seed, NOISE_SCALE, 3, pos.as_dvec2()) as f32;
    let theta = noise * TAU;
    force_from_angle(theta - FRAC_PI_2)
}

fn force_from_angle(theta: f32) -> Vec2 {
    Vec2::new(theta.cos() * FORCE_SCALE, theta.sin() * FORCE_SCALE)
}