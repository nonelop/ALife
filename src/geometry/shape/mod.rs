pub mod cube;
pub mod sphere;

use cube::Cube;
use sphere::Sphere;

use crate::geometry::vector::vector::Vector3;

#[derive(Clone, Copy, PartialEq)]
pub enum Shape {
    Cube(Cube),
    Sphere(Sphere)
}

impl GeometryShape for Shape {
    fn get_center(&self) -> Vector3 {
        match self {
            Shape::Sphere(sphere) => sphere.center,
            _ => Vector3::zero()
        }
    }
    fn get_volume_f64(&self) -> f64 {
        match self {
            Shape::Sphere(sphere) => sphere.get_volume(),
            _ => 0.0
        }
    }
}

pub trait GeometryShape {
    fn get_center(&self) -> Vector3;
    fn get_volume_f64(&self) -> f64;
}