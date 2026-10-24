use core::panic;

use crate::config::BASE_GEOMETRY_UNIT;

#[derive(Debug, Clone, Copy)]
pub enum GeometryUnits {
    Nanometer(f64),
    Micrometer(f64),
    Millimeter(f64),
    Centimeter(f64),
    Meter(f64),
    Kilometer(f64)
}

impl GeometryUnits {
    pub fn to_base_unit(&self) -> Self {

        let meters: f64 = match self.to_meters() {
            GeometryUnits::Meter(v) => v,
            _ => panic!()
        };

        match BASE_GEOMETRY_UNIT {
            GeometryUnits::Nanometer(_v) => {
                GeometryUnits::Nanometer(meters * 10_f64.powi(9))
            }
            GeometryUnits::Micrometer(_v) => {
                GeometryUnits::Micrometer(meters * 10_f64.powi(6))
            }
            GeometryUnits::Millimeter(_v) => {
                GeometryUnits::Millimeter(meters * 10_f64.powi(3))
            }
            GeometryUnits::Centimeter(_v) => {
                GeometryUnits::Centimeter(meters * 100.0)
            }
            GeometryUnits::Meter(_v) => {
                self.to_meters()
            }
            GeometryUnits::Kilometer(_v) => {
                GeometryUnits::Kilometer(meters / 1000.0)
            }
        }
    }

    pub fn to_meters(&self) -> Self {
        match self {
            GeometryUnits::Nanometer(v) => {
                GeometryUnits::Meter(v * 10_f64.powi(-9))
            }
            GeometryUnits::Micrometer(v) => {
                GeometryUnits::Meter(v * 10_f64.powi(-6))
            }
            GeometryUnits::Millimeter(v) => {
                GeometryUnits::Meter(v * 10_f64.powi(-3))
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