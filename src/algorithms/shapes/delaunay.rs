use crate::algorithms::particle::voronoi::points;
use crate::algorithms::shapes::types::shape_set::ShapeSet;
use crate::algorithms::shapes::types::tri::Triangle;
use crate::utils::colour_utils::Colour3;
use crate::*;
use glam::{DVec2, Vec3};
use rayon::prelude::*;
use voronator::delaunator::Coord;
use voronator::delaunator::Point as DelaunayPoint;

const BLACK: Rgb<f32> = Rgb([0.0, 0.0, 0.0]);

#[derive(Default)]
pub struct DelaunayTri;

type Point = (DVec2, Rgb<f32>);

impl Generator for DelaunayTri {
    fn generate(args: &Args) -> DynamicImage {
        let points = points(args, 1.4);
        let triangles = delaunay_triangulation(&points);

        let triangles = triangles
            .into_iter()
            .map(|t| {
                Triangle::new(
                    t.map(|x| x.0.as_vec2()),
                    Rgb::<f32>::from_vec3(t.map(|x| x.1.to_vec3()).iter().sum::<Vec3>() / 3.),
                )
            })
            .collect::<Vec<Triangle>>();

        let set = ShapeSet::new(triangles);

        set.generate(args, None)
    }

    fn name() -> &'static str {
        "Delaunay Triangulation"
    }
}

pub fn delaunay_triangulation(points: &Vec<Point>) -> Vec<[&Point; 3]> {
    let points_xy = points
        .iter()
        .map(|p| DelaunayPoint::from_xy(p.0.x, p.0.y))
        .collect::<Vec<_>>();
    
    let triangulation = voronator::delaunator::triangulate(&points_xy).unwrap();
    
    triangulation
        .triangles
        .windows(3)
        .step_by(3)
        .map(|slice| {

            slice
                .iter()
                .map(|i| &points[*i])
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
}
