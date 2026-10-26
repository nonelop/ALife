pub mod shape;
pub mod body;

use crate::geometry::vector::vector::Vector3;

#[derive(Debug)]
pub struct CollisionInfo {
    pub depth: f64,
    pub normal: Vector3,
    pub contact_point: Vector3
}