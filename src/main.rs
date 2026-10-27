mod config;
mod errors;
mod geometry;
mod physics;
mod time;
mod objects;
mod body;

use crate::physics::collide::body::CollidableBody;
use crate::physics::collide::shape::CollidableShape;
use crate::physics::collide::CollisionInfo;
use crate::body::Body;
use crate::physics::moves::body::MoveableBody;
use crate::time::tick;
use std::println;
use geometry::vector::vector::Vector3;
use geometry::shape::sphere::Sphere;
use geometry::shape::Shape;
use geometry::unit::unit_enum::GeometryUnits::*;
use objects::atom::Atom;

fn main() {

    let mut atom1 = Atom::new(
        Shape::Sphere(
            Sphere::new(Vector3::new(-5.0, 0.0, 0.0), Nanometer(1.0)).unwrap()
    ));

    let mut atom2 = Atom::new(
        Shape::Sphere(
            Sphere::new(Vector3::new(5.0, 0.0, 0.0), Nanometer(1.0)).unwrap()
    ));
    
    let mut counter = 0;

    let velocity1 = Vector3::new(0.5, 0.0, 0.0);
    let velocity2 = Vector3::new(-0.5, 0.0, 0.0);
    atom1.set_velocity(velocity1);
    atom2.set_velocity(velocity2);


    loop {
        atom1.move_with_velocity();
        atom2.move_with_velocity();

        println!("Atom1 position: {}", atom1.get_position());

        let atom2_shape = atom2.get_shape();

        if atom1.is_colliding(&atom2_shape) {
            atom1.collide(
                &mut atom2, 
                atom1.get_collision_info(&atom2_shape).unwrap()
            );
            println!("Atom1 colliding: {}", atom1.get_position());
        }

        tick();
    }
}
