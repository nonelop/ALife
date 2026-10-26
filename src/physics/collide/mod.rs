pub mod sphere;
pub mod shape;

use crate::geometry::shape::Shape;
use crate::geometry::vector::vector::Vector3;

#[derive(Debug)]
pub struct CollisionInfo {
    pub depth: f64,
    pub normal: Vector3,
    pub contact_point: Vector3
}

pub trait Collidable {
    fn is_colliding(&self, with: &Shape) -> bool;
    fn get_collision_info(&self, with: &Shape) -> Option<CollisionInfo>;
    fn collide(&mut self, with: &mut Shape) {}
}