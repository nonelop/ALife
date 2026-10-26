use crate::geometry::unit::unit_enum::{GeometryUnits, GeometryUnits::*};

pub const DELTA_TIME: f64 = 0.01; // Tick speed in secs

pub const BASE_GEOMETRY_UNIT: GeometryUnits = Nanometer(0.0); // Scale of simulation. 

pub const POINT_SIZE: GeometryUnits = Nanometer(1.0); // Size of minimal point

pub const CHUNK_SIZE: (GeometryUnits, GeometryUnits, GeometryUnits) =
    (Nanometer(64.0), Nanometer(64.0), Nanometer(64.0)); // Size of 1 chunk (width, height, depth)
