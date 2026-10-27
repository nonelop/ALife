mod config;
mod errors;
mod geometry;
mod physics;
mod time;
mod objects;
mod body;

use crate::physics::collide::shape::CollidableShape;
use crate::physics::collide::CollisionInfo;
use crate::body::Body;
use crate::physics::moves::body::MoveableBody;
use crate::time::tick;
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
        let velocity = Vector3::new(1.0, 0.25, 0.0);

        atom1.set_velocity(velocity);
        atom1.move_with_velocity();

        println!("Atom position: {}", atom1.get_position());

        tick();
    }
}
