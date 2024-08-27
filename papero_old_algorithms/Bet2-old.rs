use std::f64::consts::{FRAC_PI_2, TAU};
use hue::*;
use hue::colours::*;
use rand::{random, Rng};
use noise::{NoiseFn, Perlin, Seedable};
use crate::{Args, Generator};

const NOISE_SCALE: f64 = 0.002;
const FORCE_SCALE: f64 = 1.;

const DENSITY: f64 = 0.01;

const BG_COLOUR: Rgba = BLACK;

type Position = (f64, f64);
type PVector = (f64, f64);

struct Particle {
    prev_pos: Position,
    pos: Position,
    dead: bool,
    col: Rgba,
    lifetime: usize,
}

impl Particle {
    fn new(pos: Position, lifetime: usize, col: Rgba) -> Self {
        Self {
            prev_pos: pos,
            pos,
            dead: false,
            col,
            lifetime,
        }
    }

    fn update(&mut self, size: (usize, usize), noise: &Vec<Vec<f64>>) {
        let (w, h) = (size.0 as f64, size.1 as f64);
        if self.pos.0 >= w || self.pos.0 < 0. || self.pos.1 >= h || self.pos.1 < 0. {
            self.dead = true;
        }

        self.lifetime -= 1;
        if self.lifetime <= 0 {
            self.dead = true;
        }

        if self.dead {
            return;
        }

        // let force = force_at(self.pos, noise);

        self.prev_pos = self.pos;

        // let mut max = 0.0;
        // let mut max_pos = None;

        let mut dx_total = 0.;
        let mut dy_total = 0.;

        for (dx, dy) in [(-1., -1.), (-1., 0.), (-1., 1.), (0., -1.), (0., 0.), (0., 1.), (1., -1.), (1., 0.), (1., 1.)] {
            let (x, y) = ((self.pos.0 + dx) as usize, (self.pos.1 + dy) as usize);
            if noise.len() > y {
                if noise[0].len() > x {
                    let val = noise[y][x];
                    // if val > max {
                    //     max = val;
                    //     max_pos = Some((x,y));
                    // }
                    dx_total += dx * val;
                    dy_total += dy * val;
                }
            }
        }

        self.pos.0 += dx_total;
        self.pos.1 += dy_total;

        // if let Some((x,y)) = max_pos {
        //     self.pos = (x as f64, y as f64);
        // }


        // self.pos.0 += force.0 * FORCE_SCALE;
        // self.pos.1 += force.1 * FORCE_SCALE;

        // let mut hsva = self.col.to::<Hsva>();
        // hsva.h = (hsva.h + 0.001) % 1.;
        // let rgba = hsva.to::<Rgba>();
        // self.col = rgba;
    }

    fn step(&mut self, image: &mut RgbaImage) {
        // println!("{:?} prev", self.prev_pos);
        // println!("{:?} now", self.pos);
        // draw_xiaolin_wu(image, self.prev_pos, self.pos, self.col);
        // println!("drawn");

        // plot_line(image, self.prev_pos, self.pos, self.col);
        if let Some(bg) = image.get_pixel(self.pos.0 as usize, self.pos.1 as usize) {
            let col = draw_blend(bg, self.col);
            image.set_pixel(self.pos.0 as usize, self.pos.1 as usize, col);
        }
    }
}

struct ParticleSet {
    particles: Vec<Particle>,
    dead: bool,
    noise: Vec<Vec<f64>>,
}

impl ParticleSet {
    fn new(n: usize, lifetime: usize, args: &Args) -> Self {
        let size = args.get_wh();

        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..n {
            let px = rng.gen_range(0.0..(size.0 as f64));
            let py = rng.gen_range(0.0..(size.1 as f64));

            let hue = px / (size.0 as f64) + rng.gen_range(-0.2..0.2);

            // Gabe stuff
            // let col = Rgba::new(px / (size.0 as f64), py / (size.1 as f64), 0.5, 0.333);
            let col = Hsva::new(hue % 1., 0.811, 0.8, 0.3).to::<Rgba>();

            particles.push(Particle::new((px, py), lifetime, col));
        }


        let max = 5;

        let mut noise: Vec<Vec<f64>> = vec![vec![0.; size.0]; size.1];
        let mut noise_images = vec![RgbaImage::new(size.0, size.1, BLACK); max];


        for i in 1..=max {
            let seed = rng.gen_range(usize::MIN..usize::MAX);
            let perlin = Perlin::new(seed as u32);

            let noise_image = &mut noise_images[i - 1];

            for y in 0..size.1 {
                let row = &mut noise[y];
                for x in 0..size.0 {
                    let (rx, ry) = (x as f64 / size.0 as f64 - 0.5, y as f64 / size.1 as f64 - 0.5);
                    let w = (rx.powi(2) + ry.powi(2)).sqrt();


                    // I changed the next line so that val element [0,1] instead of [-1,1]
                    let val = (perlin.get([
                        x as f64 * NOISE_SCALE / args.multiplier as f64 * i as f64,
                        y as f64 * NOISE_SCALE / args.multiplier as f64 * i as f64
                    ]) + 1.0) / 2.0;
                    row[x] += (1.0 / i as f64 * val + w) / 3.0;
                    // row[x] += 1.0 / i as f64 * val;
                    // noise_image.set_pixel(x, y, Hsva::new(val, 1.0, 1.0/i as f64, 1.0).to::<Rgba>());
                }
            }
            // noise_image.save(&format!("/out/perlins/perlin{}.png", i));;
        }

        let mut noise_image = RgbaImage::new(size.0, size.1, BLACK);
        for y in 0..size.1 {
            for x in 0..size.0 {
                let val = noise[y][x];
                noise_image.set_pixel(x, y, Hsva::new(val * 0.9, 1.0, 1.0, 1.0).to::<Rgba>());
            }
        }
        noise_image.save("perlin.png");

        Self {
            particles,
            dead: false,
            noise,
        }
    }

    fn update(&mut self, size: (usize, usize)) {
        self.dead = true;
        for p in &mut self.particles {
            if !p.dead {
                self.dead = false;
                p.update(size, &self.noise);
            }
        }
    }

    fn draw(&mut self, image: &mut RgbaImage) {
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
    fn run(&mut self, size: (usize, usize)) -> RgbaImage {
        let mut image = RgbaImage::new(size.0, size.1, BG_COLOUR);
        // let mut frames = vec![];

        // let mut displacement = 0.;

        let mut counter = 0;
        while !self.dead {
            counter += 1;

            self.update(size);
            self.draw(&mut image);

            if counter % 50 == 0 {
                println!("i: {}, {}/{} particles", counter, self.alive(), self.particles.len());
            }
            // if counter % 10 == 0 {
            //     frames.push(image.clone())
            // }
        }
        // crate::util::gif_gen(frames, 20);
        image
    }
}

pub struct Bet2;

impl Generator for Bet2 {
    fn create(&self, args: &Args) -> RgbaImage {
        let (width, height) = args.get_wh();
        let mut image = RgbaImage::new(width, height, BLACK);
        (&mut image, (0., 0.), (width as f64, height as f64), RED);

        let lifetime = (width + height) * 2;

        let mut particles = ParticleSet::new(((width * height) as f64 * DENSITY) as usize, lifetime, &args);

        particles.run((width, height))
    }
}

fn draw_blend(bg: Rgba, fg: Rgba) -> Rgba {
    if fg.a == 0. {
        return bg;
    }
    if fg.a == 1. {
        return fg;
    }

    let alpha_final = bg.a + fg.a - bg.a * fg.a;
    if alpha_final == 0. {
        return Rgba::new(0., 0., 0., 0.);
    }

    let (bg_r_a, bg_g_a, bg_b_a) = (bg.r * bg.a, bg.g * bg.a, bg.b * bg.a);
    let (fg_r_a, fg_g_a, fg_b_a) = (fg.r * fg.a, fg.g * fg.a, fg.b * fg.a);

    let (out_r_a, out_g_a, out_b_a) = (
        fg_r_a + bg_r_a * (1.0 - fg.a),
        fg_g_a + bg_g_a * (1.0 - fg.a),
        fg_b_a + bg_b_a * (1.0 - fg.a),
    );

    let (out_r, out_g, out_b) = (
        out_r_a / alpha_final,
        out_g_a / alpha_final,
        out_b_a / alpha_final,
    );

    Rgba::new(out_r, out_g, out_b, alpha_final)
}

fn force_at(pos: Position, noise: &Vec<Vec<f64>>) -> PVector {
    let theta = noise[pos.1 as usize][pos.0 as usize] * TAU;
    force_from_angle(theta - FRAC_PI_2)
}

fn force_from_angle(theta: f64) -> PVector {
    (theta.cos() * FORCE_SCALE, theta.sin() * FORCE_SCALE)
}

