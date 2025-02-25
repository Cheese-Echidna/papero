use crate::utils::colour_utils;
use crate::utils::colour_utils::ImageColour;
use crate::utils::noise::BetterFbm;
use crate::{Args, Generator};
use glam::{UVec2, Vec2, Vec4};
use image::{DynamicImage, GenericImageView, Rgba, Rgba32FImage};
use palette::named::BLACK;
use rand::Rng;
use std::f32::consts::TAU;

const NOISE_SCALE: f64 = 0.002;
pub(crate) const FORCE_SCALE: f32 = 1.;

const DENSITY: f32 = 0.002;

struct Particle {
    prev_pos: Vec2,
    pos: Vec2,
    dead: bool,
    col: Rgba<f32>,
    lifetime: usize,
    t: bool,
}

impl Particle {
    fn new(pos: Vec2, lifetime: usize, col: Rgba<f32>, t: bool) -> Self {
        Self {
            prev_pos: pos,
            pos,
            dead: false,
            col,
            lifetime,
            t,
        }
    }

    fn update(&mut self, size: Vec2, better_fbm: &BetterFbm) {
        let bound_dim = 1.2;
        let (x_min, x_max) = ((1.0 - bound_dim) * size.x, bound_dim * size.x);
        let (y_min, y_max) = ((1.0 - bound_dim) * size.y, bound_dim * size.y);
        if self.pos.x > x_max || self.pos.x < x_min || self.pos.y > y_max || self.pos.y < y_min {
            self.dead = true;
        }

        self.lifetime -= 1;
        if self.lifetime == 0 {
            self.dead = true;
        }

        if self.dead {
            return;
        }

        let noise = better_fbm.get(self.pos.as_dvec2()) as f32;
        let theta = noise * TAU;
        let force = Vec2::new(theta.cos(), theta.sin());

        self.prev_pos = self.pos;
        self.pos += force * FORCE_SCALE * self.mult();
    }

    fn mult(&self) -> f32 {
        if self.t {
            1.0
        } else {
            -1.0
        }
    }

    fn draw(&self, image: &mut Rgba32FImage) {
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
    fbm: BetterFbm,
    size: Vec2
}

impl ParticleSet {
    fn new(n: usize, lifetime: usize, args: &Args) -> Self {
        let size = args.wh();

        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..n {
            let size = Vec2::new(size.0 as f32, size.1 as f32);
            let p = (crate::utils::num_utils::random_unit_vec() * 1.4 - 0.2) * size;

            let col = colour_utils::sick_gradient(p.x / size.x, p.y / size.y).with_alpha_of(0.35);

            particles.push(Particle::new(Vec2::new(p.x, p.y), lifetime, col, false));
            particles.push(Particle::new(Vec2::new(p.x, p.y), lifetime, col, true));
        }

        let seed = rng.gen_range(0..u32::MAX);

        let fbm = BetterFbm::new(seed, 1, NOISE_SCALE);

        Self {
            particles,
            dead: false,
            fbm,
            size: Vec2::new(size.0 as f32, size.1 as f32),
        }
    }

    fn update(&mut self) {
        self.dead = true;
        for p in &mut self.particles {
            if !p.dead {
                self.dead = false;
                p.update(self.size, &self.fbm);
            }
        }
    }

    fn draw(&self, image: &mut Rgba32FImage) {
        for p in self.particles.iter() {
            if !p.dead && self.includes(p, 1.0) {
                p.draw(image);
            }
        }
    }

    fn alive(&self) -> usize {
        self.particles.iter().filter(|p| !p.dead).count()
    }

    fn includes(&self, particle: &Particle, scale: f32) -> bool {
        let (x_min, x_max) = ((1.0 - scale) * self.size.x, scale * self.size.x);
        let (y_min, y_max) = ((1.0 - scale) * self.size.y, scale * self.size.y);
        !(particle.pos.x > x_max || particle.pos.x < x_min || particle.pos.y > y_max || particle.pos.y < y_min)
    }

    // main loop
    fn run(&mut self, args: &Args) -> DynamicImage {
        let mut image = args.image_f32_alpha(Rgba::<f32>::from_const(BLACK));

        let mut counter = 0;
        while !self.dead {
            counter += 1;

            self.update();
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

        let mut particles =
            ParticleSet::new(((width * height) as f32 * DENSITY) as usize, lifetime, args);

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
