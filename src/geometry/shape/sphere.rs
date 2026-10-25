use crate::errors::GeometryErrors;
use crate::geometry::point::point::Point3;
use crate::geometry::unit::unit_enum::GeometryUnits;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(position: Point3, radius: GeometryUnits) -> Result<Self, GeometryErrors> {
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
}
