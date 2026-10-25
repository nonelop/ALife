pub mod sphere;
pub mod shape;

use crate::geometry::shape::Shape;
use crate::geometry::vector::vector::Vector3;

pub struct CollisionInfo {
    pub depth: f64,
    pub normal: Vector3,
    pub contact_point: Vector3
}

pub trait Collidable {
    fn is_colliding(&self, with: &Shape) -> bool;

    fn collide(&self, with: &Shape) {}
}