mod config;
mod errors;
mod geometry;
mod physics;
mod time;
mod objects;
mod body;

use crate::body::Body;
use crate::physics::body::PhysicsBody;
use crate::time::tick;
use std::println;
use geometry::vector::Vector3;
use geometry::shape::sphere::Sphere;
use geometry::shape::Shape;
use geometry::unit::GeometryUnits::*;
use objects::atom::Atom;
use crate::config::DELTA_TIME;
use std::thread::sleep;
use std::time::Duration;


fn main() {
    let mut objects: Vec<Box<dyn PhysicsBody>> = Vec::new();

    let mut atom1 = Atom::new(
        Shape::Sphere(
            Sphere::new(Vector3::new(-5.0, 0.75, 5.0), Meter(1.0)).unwrap()
    ));

    let mut atom2 = Atom::new(
        Shape::Sphere(
            Sphere::new(Vector3::zero(), Meter(1.0)).unwrap()
    ));

    atom1.set_velocity(Vector3::new(0.5, 0.0, -0.5));

    objects.push(Box::new(atom1));
    objects.push(Box::new(atom2));
    
    let mut _counter = 0;

    loop {
        println!("Atom1: {} | Atom2: {}", objects[0].get_position(), objects[1].get_position());

        tick(DELTA_TIME, &mut objects);
        
        sleep(Duration::from_secs_f64(DELTA_TIME));
    }
}
