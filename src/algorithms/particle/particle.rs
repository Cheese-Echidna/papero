use rand::Rng;

pub(crate) struct Particle {
    point: Point,
    colour: image::Rgba<f32>
}

pub(crate) struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub(crate) fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Point {
    pub(crate) fn new(x:f64, y:f64) -> Self {
        Self{x, y}
    }
    pub(crate) fn random_particle_zero_one() -> Self {
        let mut rng = rand::thread_rng();
        Point::new(
            rng.gen_range(0.0..1.),
            rng.gen_range(0.0..1.)
        )
    }
}