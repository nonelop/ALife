use super::Collidable;
use crate::geometry::shape::Shape;

// wrapper for Shape enum which just links to real shapes

impl Collidable for Shape {
    fn is_colliding(&self, with: &Shape) -> bool {
        match self {
            Self::Sphere(sphere) => sphere.is_colliding(with),
            Self::Cube(cube) => false
        }
    }
}