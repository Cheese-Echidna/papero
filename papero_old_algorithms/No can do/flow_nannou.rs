use nannou::prelude::*;
use nannou::noise::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Particle {
    positions: Vec<Vec2>,
}
impl Particle {
    pub fn new(p:Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        Particle {
            positions,
        }
    }
}

struct Model {
    things: Vec<Particle>,
    noise: OpenSimplex,
}

const N_THINGS: usize = 20000;

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024,1024).view(view).build().unwrap();
    let mut things = Vec::new();

    let size = app.window_rect().wh();

    for _i in 0..N_THINGS{
        let thing = Particle::new(size * Vec2::new(random::<f32>() - 0.5,random::<f32>() - 0.5));
        things.push(thing);
    }

    let noise = OpenSimplex::new();
    Model {
        things,
        noise,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let dt = app.duration.since_prev_update.as_secs_f32();

    for particle in model.things.iter_mut() {
        if let Some(current_pos) = particle.positions.last() {
            let ang = generate(&model.noise, 0.003, current_pos) * TAU;
            let p = Vec2::new(ang.cos(), ang.sin());
            let d = p.length() / 7000.;
            let out_v = current_pos.normalize() * d;

            let new_pos = *current_pos + (p + out_v) * dt;
            particle.positions.push(new_pos);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for thing in model.things.iter() {
        draw.polyline().points(thing.positions.iter().cloned()).color(WHITE);
    }


    // let d = 36;
    //
    // for x in (-512/d)..(512/d) {
    //     for y in -20..20 {
    //         let p = Vec2::new(x as f32, y as f32) * d as f32;
    //         let n = generate(&model.noise, 0.003, &p) * TAU;
    //         draw.arrow()
    //             .color(WHITE)
    //             .points(p, p + Vec2::new(n.cos(), n.sin()) * d as f32)
    //             .weight(3.0);
    //     }
    // }

    draw.to_frame(app, &frame).unwrap();
}

fn generate(noise: &OpenSimplex, scale: f32, at: &Vec2) -> f32 {
    noise.get([(at.x * scale) as f64 + std::f64::consts::PI, (at.y * scale) as f64 + std::f64::consts::E]) as f32 + 0.5
}