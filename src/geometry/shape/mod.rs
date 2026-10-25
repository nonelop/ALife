pub mod cube;
pub mod sphere;

use cube::Cube;
use sphere::Sphere;

#[derive(Clone, Copy)]
pub enum Shape {
    Cube(Cube),
    Sphere(Sphere)
}