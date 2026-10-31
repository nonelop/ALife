use crate::geometry::vector::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    center: Vector3,
    width: f64,
    height: f64,
    depth: f64,
}
