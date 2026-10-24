use crate::errors::GeometryErrors;
use crate::geometry::point::point::Point;
use crate::geometry::unit::unit_enum::GeometryUnits;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: GeometryUnits,
}

impl Sphere {
    pub fn new(position: Point, radius: GeometryUnits) -> Result<Self, GeometryErrors> {
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
