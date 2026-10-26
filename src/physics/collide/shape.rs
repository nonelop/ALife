use super::{Collidable, CollisionInfo};
use crate::geometry::shape::Shape;

// wrapper for Shape enum which just links to real shapes

impl Collidable for Shape {
    fn is_colliding(&self, with: &Shape) -> bool {
        match self {
            Self::Sphere(sphere) => sphere.is_colliding(with),
            Self::Cube(cube) => false
        }
    }
    fn get_collision_info(&self, with: &Shape) -> Option<CollisionInfo> {
        match self {
            Self::Sphere(sphere) => sphere.get_collision_info(with),
            Self::Cube(cube) => None
        }
    }
}