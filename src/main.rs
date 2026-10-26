mod config;
mod errors;
mod geometry;
mod physics;
mod time;
mod objects;

use crate::physics::collide::{CollidableShape, CollisionInfo};
use crate::time::tick::tick;
use std::time::Duration;
use std::{println, thread};
use geometry::vector::vector::Vector3;
use geometry::shape::sphere::Sphere;
use geometry::shape::Shape;
use geometry::unit::unit_enum::GeometryUnits::*;
use objects::atom::Atom;

fn main() {

    let mut atom1 = Atom::new(
        Shape::Sphere(
            Sphere::new(Vector3::zero(), Nanometer(5.0)).unwrap()
    ));

    let mut atom2 = Atom::new(
        Shape::Sphere(
            Sphere::new(Vector3::zero(), Nanometer(5.0)).unwrap()
    ));
    
    let mut counter = 0;

    loop {
        thread::sleep(Duration::from_millis(100));

        counter += 1;

        tick();
    }
}
