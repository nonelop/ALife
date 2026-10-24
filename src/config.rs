use crate::geometry::unit::unit_enum::GeometryUnits;

pub const BASE_GEOMETRY_UNIT: GeometryUnits = GeometryUnits::Nanometer(0.0); // Scale of simulation. 

pub const MIN_POINT_SIZE: GeometryUnits = GeometryUnits::Nanometer(1.0); // Size of minimal point

pub const CHUNK_SIZE: (u32, u32, u32) = (32, 32, 32); // Size of 1 chunk (width, height, depth)