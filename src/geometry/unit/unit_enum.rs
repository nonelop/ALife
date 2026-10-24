use crate::{config::BASE_GEOMETRY_UNIT, geometry::unit::unit_enum::GeometryUnits::Nanometer};

#[derive(Debug, Clone, Copy)]
pub enum GeometryUnits {
    Nanometer(f32),
    Micrometer(f32),
    Millimeter(f32),
    Centimeter(f32),
    Meter(f32),
    Kilometer(f32)
}

impl GeometryUnits {
    pub fn to_meters(&self) -> Self {
        match self {
            GeometryUnits::Nanometer(v) => {
                GeometryUnits::Meter(v.powi(-9))
            }
            GeometryUnits::Micrometer(v) => {
                GeometryUnits::Meter(v.powi(-6))
            }
            GeometryUnits::Millimeter(v) => {
                GeometryUnits::Meter(v.powi(-3))
            }
            GeometryUnits::Centimeter(v) => {
                GeometryUnits::Meter(v / 100.0)
            }
            GeometryUnits::Meter(_v) => {
                return *self;
            }
            GeometryUnits::Kilometer(v) => {
                GeometryUnits::Meter(v * 1000.0)
            }
        }
    }
}