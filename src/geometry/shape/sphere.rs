use crate::errors::GeometryErrors;
use crate::geometry::vector::vector::Vector3;
use crate::geometry::unit::unit_enum::GeometryUnits;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(position: Vector3, radius: GeometryUnits) -> Result<Self, GeometryErrors> {
        let radius = radius.to_base_unit().get_value_in_base();
        
        if radius <= 0.0 {
            return Err(GeometryErrors::InvalidSizeValue(
                radius,
            ));
        }

        Ok(Sphere {
            center: position,
            radius: radius,
        })
    }
    pub fn get_volume(&self) -> f64 {
        (4.0/3.0) * PI * self.radius.powi(3)
    }
}
