use hue::colours::BLACK;
use crate::*;
use noise::{NoiseFn, Perlin, Seedable, Simplex};
use rand::Rng;
// use rayon::prelude::*;
use crate::position::*;

const SCALE_FACTOR: f64 = 300.;
const NOISE_FACTOR: f64 = 400.;

pub struct Gimel;

pub struct ShapeSet {
    pub(crate) objects: Vec<Box<dyn ShapeObject>>,
}

pub struct Particle {
    pos: Position,
    col: Rgba,
    radius: f64,
}


pub trait ShapeObject: Send + Sync {
    fn sdf(&self, position: &Position) -> f64;

    fn colour(&self) -> Rgba;

    fn position_mut(&mut self) -> &mut Position;
}

impl Generator for Gimel {
    fn create(&self, args: &Args) -> RgbaImage {

        let mut rng = rand::thread_rng();
        let seed = rng.gen_range(0..u32::MAX);

        let (width, height) = args.get_wh();

        let mut image = RgbaImage::new(width, height, BLACK);

        for y in 0..height {
            for x in 0..width {
                let mut pos = Position{x:x as f64,y:y as f64};
                let old_pos = pos.clone();

                domain_warp(&mut pos, seed, seed+1);
                let distance = pos.distance(&old_pos);

                let c = Rgba::new(pos.x / args.width_f64(), pos.y / args.height_f64(), 0.5, 1.0);
                image.set_pixel(x,y,c.to::<Rgba>());
            }
        };

        image

        // render_double_perlin(&args, seed)

        // let mut set =
        //     ShapeSet::new_uniform_of_particles(&args, 30,
        //                                        |p| { 25.0 },
        //                                        |p, args| { Hsva::new(p.x / args.width_f64(), p.y / args.height_f64() * 0.75 + 0.25, 1.0, 1.0).to::<Rgba>() });
        //
        // set.objects.iter_mut()
        //     .for_each(|object| domain_warp(object.position_mut(), seed, 0.5, 3));

        // set.render(&args)
    }
}


impl ShapeSet {
    pub fn new_uniform_of_particles(args: &Args, step_by: usize, radius_fn: fn(pos: &Position) -> f64, colour_fn: fn(pos: &Position, args: &Args) -> Rgba) -> ShapeSet {
        let (width, height) = args.get_wh();
        let mut points = vec![];
        for x in (0..width).step_by(step_by) {
            for y in (0..height).step_by(step_by) {
                let pos = Position { x: x as f64, y: y as f64 };
                let radius = radius_fn(&pos);
                let colour = colour_fn(&pos, &args);
                let p = Particle::new(pos, colour, radius);
                points.push(Box::new(p) as Box<dyn ShapeObject>);
            }
        }
        ShapeSet { objects: points }
    }

    pub fn render(&self, args: &Args) -> RgbaImage {
        let (width, height) = args.get_wh();
        let mut image = RgbaImage::blank(width, height);
        for y in 0..height {
            for x in 0..width {
                let pos = Position { x: x as f64, y: y as f64 };

                let colours = self.objects.iter().filter_map(|object| {
                    let d = object.sdf(&pos);
                    if d < 0.0 { // needs to be refactored into a const
                        return Some(object.colour());
                    };
                    return None;
                }).collect::<Vec<Rgba>>();

                if colours.len() > 0 {
                    image.set_pixel(x, y, Rgba::average(&colours));
                }
                // else: leave as is
            }
        }
        image
    }
}

fn fbm(seed: u32, h: f64, n: usize, x: Position) -> f64 {
    // let max_min = (2.0_f64.powf(h) - 2.0_f64.powf(h - h * n as f64)) / (2.0_f64.powf(h) - 1.0);
    // let max_min = (1. - 2.0_f64.powf(-h*n as f64)) / (h * 2.0_f64.ln());

    let x = x / NOISE_FACTOR;

    let g = f64::exp2(-h);
    let mut f = 1.0;
    let mut a = 1.0;
    let mut t = 0.0;

    // let noise = Simplex::new(seed);
    let noise = Perlin::new(seed);

    for _ in 0..n {
        t += a * noise.get(p(x * f));
        f *= 2.0;
        a *= g;
    }
    // (t + max_min) / (2.0 * max_min)
    t
}

fn fbm_2d(seed1: u32, seed2: u32, h: f64, n: usize, x: Position) -> Position {
    Position {
        x: fbm(seed1, h, n, x),
        y: fbm(seed2, h, n, x),
    }
}

fn p(pos: Position) -> [f64; 2] {
    return [pos.x, pos.y];
}

impl Particle {
    pub fn new(position: Position, colour: Rgba, radius: f64) -> Particle {
        Particle { pos: position, col: colour, radius }
    }
}

impl ShapeObject for Particle {
    fn sdf(&self, point: &Position) -> f64 {
        self.pos.distance(point) - self.radius
    }

    fn colour(&self) -> Rgba {
        self.col.clone()
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.pos
    }
}

fn domain_warp(position: &mut Position, seed_1: u32, seed_2:u32) {
    let f = |x: Position| -> f64 {
        let perlin = Perlin::new(seed_1);
        perlin.get(p(x / NOISE_FACTOR)) * SCALE_FACTOR
    };
    let g = |x: Position| -> f64 {
        let perlin = Perlin::new(seed_2);
        perlin.get(p(x / NOISE_FACTOR)) * SCALE_FACTOR
    };

    let (x, y) = (position.x + f(position.clone()), position.y + g(position.clone()));
    let new_position = Position { x, y };
    *position = new_position;
}

fn domain_warp_2(position: &mut Position, seed: u32, h: f64, n: usize) {
    let f = |x: Position| -> f64 { fbm(seed, h, n, x) * SCALE_FACTOR};
    let g = |x: Position| -> f64 { fbm(seed+1, h, n, x) * SCALE_FACTOR };

    let (x, y) = (position.x + f(position.clone()), position.y + g(position.clone()));
    let new_position = Position { x, y };
    *position = new_position;
}

fn render_fbm(args: &Args, seed: u32, h: f64, n: usize) -> RgbaImage {
    let (width, height) = args.get_wh();

    let mut image = RgbaImage::blank(width, height);

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    for y in 0..height {
        for x in 0..width {
            let p = Position { x: x as f64, y: y as f64 } / 400.;
            let q = Position { x: fbm(seed, h, n, p), y: fbm(seed + 1, h, n, p) };
            // let r = Position{x: fbm(seed+2, h, n, p + (q*4.0)), y: fbm(seed+3, h, n, p + (q*4.0))};

            let v = fbm(seed + 4, h, n, p + (q * 4.0)) / 4.0;
            let v = v/5.0 + 0.6;
            // ang element [0.5, 0.7]

            let c = Hsva::new(v + 1.0, 1.0, 1.0, 1.0).to::<Rgba>();
            image.set_pixel(x, y, c);
        }
    }

    // n = 1 => 1.0
    // n = 2 => 1.7
    // n = 3 => 2
    // n = 5 => 2.3
    // n = 10 => 2.5
    // n = 20 => 2.5
    dbg!(min);
    dbg!(max);

    image
}

// http://thingonitsown.blogspot.com/2019/01/general-domain-warping.html
// height(x,y,strength,size) = noise(x,y,strength*noise(size*x,size*y))
fn render_double_perlin(args: &Args, seed: u32) -> RgbaImage {
    let (width, height) = args.get_wh();

    let noise = Perlin::new(seed);
    let noise2 = Perlin::new(seed + 1);

    let mut image = RgbaImage::blank(width, height);

    for frame in 0..30 {
        for y in 0..height {
            for x in 0..width {
                // let strength = (1.33_f64).powi(4);
                let size = 1. / 400.;
                let z = noise.get([x as f64 * size, y as f64 * size]);
                let height = noise2.get([x as f64 * size, y as f64 * size, z + (frame as f64 / 10.)]);

                // dbg!(z);
                // dbg!(height);

                let colour = Hsva::new(height, 0.8, 0.8, 1.).to::<Rgba>();
                image.set_pixel(x, y, colour);
            }
        }
        image.save(&format!("out/gif/{}.png", frame));
        println!("Finished with frame {}", frame);
    }

    image
}
