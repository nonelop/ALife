mod config;
mod errors;
mod geometry;
mod time;

use crate::time::tick::tick;
use std::time::Duration;
use std::{println, thread};

use geometry::point::point::Point3;
use geometry::vector::vector::Vector3;
use geometry::shape::sphere::Sphere;
use geometry::unit::unit_enum::{GeometryUnits, GeometryUnits::*};

fn main() {
    let mut sphere = Sphere::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Nanometer(5.0),
    )
    .unwrap();

    loop {
        thread::sleep(Duration::from_millis(10));

        let dx = 0.5;

        sphere.center = Point3 {
            x: sphere.center.x + dx,
            y: sphere.center.y,
            z: sphere.center.z,
        };

        println!("{:?}", sphere.center);

        tick();
    }
}
