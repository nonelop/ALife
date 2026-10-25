use crate::errors::GeometryErrors;
use crate::geometry::point::point::Point3;
use crate::geometry::unit::unit_enum::GeometryUnits;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: GeometryUnits,
}

impl Sphere {
    pub fn new(position: Point3, radius: GeometryUnits) -> Result<Self, GeometryErrors> {
        if radius.get_value_in_meters() <= 0.0 {
            return Err(GeometryErrors::InvalidSizeValue(
                radius.get_value_in_meters(),
            ));
        }

        Ok(Sphere {
            center: position,
            radius: radius,
        })
    }
}
