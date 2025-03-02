use glam::Vec2;
use crate::algorithms::shapes::types::shape_object::ShapeObject;
use crate::Rgb;

pub struct Triangle {
    points: [Vec2; 3],
    colour: Rgb<f32>,
}

impl Triangle {
    pub fn new(points: [Vec2; 3], colour: Rgb<f32>) -> Self {
        Self { points, colour }
    }
}

impl ShapeObject for Triangle {
    fn sdf(&self, position: &Vec2) -> f32 {
        let p = *position;
        let [p0, p1, p2] = self.points;

        // Closure to compute the distance from a point to a segment [a, b].
        let edge_distance = |a: Vec2, b: Vec2| {
            let pa = p - a;
            let ab = b - a;
            let t = (pa.dot(ab)) / ab.dot(ab);
            let t = t.clamp(0.0, 1.0);
            (pa - ab * t).length()
        };

        // Compute the distance from p to each edge of the triangle.
        let d0 = edge_distance(p0, p1);
        let d1 = edge_distance(p1, p2);
        let d2 = edge_distance(p2, p0);
        let min_dist = d0.min(d1).min(d2);

        // Compute signed areas (via 2D cross products) for each edge.
        let s0 = (p1.x - p0.x) * (p.y - p0.y) - (p1.y - p0.y) * (p.x - p0.x);
        let s1 = (p2.x - p1.x) * (p.y - p1.y) - (p2.y - p1.y) * (p.x - p1.x);
        let s2 = (p0.x - p2.x) * (p.y - p2.y) - (p0.y - p2.y) * (p.x - p2.x);

        // Determine if the point is inside the triangle:
        // All cross products must have the same sign.
        let inside = (s0 >= 0.0 && s1 >= 0.0 && s2 >= 0.0) ||
            (s0 <= 0.0 && s1 <= 0.0 && s2 <= 0.0);

        if inside {
            -min_dist
        } else {
            min_dist
        }
    }

    fn colour(&self) -> Rgb<f32> {
        self.colour
    }

    fn position_mut(&mut self) -> &mut Vec2 {
        &mut self.points[0]
    }
}