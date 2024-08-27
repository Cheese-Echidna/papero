use rand_distr::num_traits::clamp;
use crate::{Args, Generator};
use crate::*;
use crate::Gimel::*;
use crate::position::*;

pub struct Dalet;

impl Generator for Dalet {
    fn create(&self, args: &Args) -> RgbaImage {
        let (width, height) = args.get_wh();

        let mut points = vec![];
        for x in (0..width).step_by(120) {
            for y in (0..height).step_by(120) {
                let pos = Position { x: x as f64, y: y as f64 };
                let size = (x as f64).sqrt();
                let colour = Hsva::new(x as f64 / width as f64, y as f64 / height as f64 * 0.5 + 0.5, 1.0, 1.0).to::<Rgba>();

                let p = Hexagon {
                    pos,
                    col: colour,
                    size,
                    radius: 0.0,
                };
                points.push(Box::new(p) as Box<dyn ShapeObject>);
            }
        }
        let shapes = ShapeSet { objects: points };


        shapes.render(&args)

    }
}


// Note for now we are only doing pointy top hexagons
// size is the distance from the point to the centre
pub struct Hexagon {
    pos: Position,
    col: Rgba,
    size: f64,
    radius: f64,
}

impl ShapeObject for Hexagon {
    fn sdf(&self, position: &Position) -> f64 {
        let k2 = Position{x:-0.866025404,y:0.5};
        let z = 0.577350269;
        let s = self.size;
        let r = self.radius;

        // confusing line
        let mut p = (*position - self.pos).abs_each();
        p -= k2 * (2.0 * f64_min(k2.dot(&p), 0.0));
        p -= Position{x:clamp(p.x, -z*s, z*s), y:s};
        return (p.mag() * sign(p.y) - r);
    }


    fn colour(&self) -> Rgba {
        self.col.clone()
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.pos
    }
}

fn f64_min(x:f64, y:f64) -> f64 {
    if x > y {
        return y
    }
    return x
}