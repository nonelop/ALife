use crate::geometry::unit::{GeometryUnits, GeometryUnits::*};

pub const DELTA_TIME: f64 = 0.01; // Tick speed in secs

// Now base value is meter

pub const CHUNK_SIZE: (GeometryUnits, GeometryUnits, GeometryUnits) =
    (Nanometer(64.0), Nanometer(64.0), Nanometer(64.0)); // Size of 1 chunk (width, height, depth)
