use crate::geometry::vector::vector::Vector3;
use crate::geometry::unit::unit_enum::GeometryUnits;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    center: Vector3,
    width: GeometryUnits,
    height: GeometryUnits,
    depth: GeometryUnits,
}
