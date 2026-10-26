mod config;
mod errors;
mod geometry;
mod physics;
mod time;
mod objects;

use crate::physics::collide::{Collidable, CollisionInfo};
use crate::time::tick::tick;
use std::time::Duration;
use std::{println, thread};

use geometry::vector::vector::Vector3;
use geometry::shape::sphere::Sphere;
use geometry::shape::Shape;
use geometry::unit::unit_enum::GeometryUnits::*;

fn main() {
    let mut sphere1 = Sphere::new(
        Vector3 {
            x: -20.0,
            y: 0.0,
            z: 0.0,
        },
        Nanometer(2.5),
    )
    .unwrap();

    let mut sphere2 = Sphere::new(
        Vector3 {
            x: 20.0,
            y: 0.0,
            z: 0.0,
        },
        Nanometer(2.5),
    )
    .unwrap();

    let mut counter = 0;

    loop {
        thread::sleep(Duration::from_millis(100));

        counter += 1;

        let dx = 0.1;

        sphere1.center.x += dx;
        sphere2.center.x -= dx;

        println!("[INFO] [T: {}] X1: {:.4} | X2: {:.4}", counter, sphere1.center.x, sphere2.center.x);

        if sphere1.is_colliding(&Shape::Sphere(sphere2)) {
            println!("[INFO] [T: {}] Sphere1 colliding Sphere2", counter);
            println!("[INFO] [T: {}] {:?}", counter, sphere1.get_collision_info(&Shape::Sphere(sphere2)));
        }

        tick();
    }
}
