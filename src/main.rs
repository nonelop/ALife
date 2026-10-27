mod config;
mod errors;
mod geometry;
mod physics;
mod time;
mod objects;
mod body;

use crate::physics::collide::body::CollidableBody;
use crate::physics::collide::shape::CollidableShape;
use crate::body::{Body, PhysicsBody};
use crate::physics::moves::body::MoveableBody;
use crate::time::tick;
use std::println;
use geometry::vector::vector::Vector3;
use geometry::shape::sphere::Sphere;
use geometry::shape::Shape;
use geometry::unit::unit_enum::GeometryUnits::*;
use objects::atom::Atom;

fn main() {
    let mut objects: Vec<Box<dyn PhysicsBody>> = Vec::new();

    let mut atom1 = Atom::new(
        Shape::Sphere(
            Sphere::new(Vector3::new(-5.0, 0.0, 0.0), Nanometer(1.0)).unwrap()
    ));

    let mut atom2 = Atom::new(
        Shape::Sphere(
            Sphere::new(Vector3::new(5.0, 0.0, 0.0), Nanometer(1.0)).unwrap()
    ));

    atom1.set_velocity(Vector3::new(0.5, 0.0, 0.0));
    atom2.set_velocity(Vector3::new(-0.5, 0.0, 0.0));


    objects.push(Box::new(atom1));
    objects.push(Box::new(atom2));
    
    let mut counter = 0;

    loop {
        println!("Atom1 position: {}", objects[0].get_position());

        tick(&mut objects);
    }
}
