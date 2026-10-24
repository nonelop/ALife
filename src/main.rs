mod config;
mod errors;
mod geometry;
mod time;

use crate::time::tick::tick;
use std::time::Duration;
use std::{println, thread};

use geometry::point::point::Point;
use geometry::shape::sphere::Sphere;
use geometry::unit::unit_enum::{GeometryUnits, GeometryUnits::*};

fn main() {
    let mut sphere = Sphere::new(
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Nanometer(5.0),
    )
    .unwrap();

    loop {
        thread::sleep(Duration::from_millis(10));

        let dx = 0.1;

        sphere.center = Point {
            x: sphere.center.x + dx,
            y: sphere.center.y,
            z: sphere.center.z,
        };

        println!("{:?}", sphere.center);

        tick();
    }
}
