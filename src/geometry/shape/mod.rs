pub mod cube;
pub mod sphere;

use cube::Cube;
use sphere::Sphere;

#[derive(Clone, Copy, PartialEq)]
pub enum Shape {
    Cube(Cube),
    Sphere(Sphere)
}