pub mod sphere;
pub mod shape;

use crate::geometry::shape::Shape;
use crate::geometry::vector::vector::Vector3;
use super::CollisionInfo;


pub trait CollidableShape {
    fn is_colliding(&self, with: &Shape) -> bool;
    fn get_collision_info(&self, with: &Shape) -> Option<CollisionInfo>;
}