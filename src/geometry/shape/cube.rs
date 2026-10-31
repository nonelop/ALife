use crate::{errors::GeometryErrors, geometry::{shape::GeometryShape, unit::GeometryUnits, vector::Vector3}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    center: Vector3,
    edge: f64
}

impl Cube {
    pub fn new(position: Vector3, edge: GeometryUnits) -> Result<Self, GeometryErrors> {
        if edge.get_value_in_base() < 0.0 {
            return Err(GeometryErrors::InvalidSizeValue(edge.get_value_in_base()));
        }
        Ok(Cube { center: position, edge: edge.get_value_in_base() })
    }

    pub fn get_edge(&self) -> f64 { self.edge }
}

impl GeometryShape for Cube {
    fn get_center(&self) -> Vector3 {
        self.center
    }
    fn set_center(&mut self, center: Vector3) {
        self.center = center
    }
    fn get_volume(&self) -> f64 {
        self.edge.powi(3)
    }
}