mod geometry;
mod physics;
mod body;
mod objects;
mod time;
mod errors;
mod config;

use kiss3d::prelude::*;

use crate::geometry::unit::GeometryUnits::Meter;
use crate::geometry::vector::Vector3;
use crate::geometry::shape::{Shape, sphere::Sphere};
use crate::geometry::shape::GeometryShape;
use crate::objects::atom::Atom;
use crate::config::DELTA_TIME;
use crate::physics::body::PhysicsBody;

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Simulation: AUS").await;
    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty();

    let mut objects: Vec<Box<dyn PhysicsBody>> = Vec::new();

    let mut atom = Atom::new(
        Shape::Sphere(Sphere::new(Vector3::new(0.0, 0.25, 0.15), Meter(1.0)).unwrap())
    );

    let mut atom2 = Atom::new(
        Shape::Sphere(Sphere::new(Vector3::new(20.0, 0.0, 0.0), Meter(1.0)).unwrap())
    );

    objects.push(Box::new(atom));
    objects.push(Box::new(atom2));

    let mut atom_sphere = scene.add_sphere(
        objects[0].get_shape().get_radius() as f32
    );
    let mut atom2_sphere = scene.add_sphere(
        objects[1].get_shape().get_radius() as f32
    );


    objects[0].set_velocity(Vector3::new(5.0, 0.0, 0.0));

    while window.render_3d(&mut scene, &mut camera).await {
        time::tick(DELTA_TIME, &mut objects);

        atom_sphere.set_position(vector3_to_vec3(&objects[0].get_position()));
        atom2_sphere.set_position(vector3_to_vec3(&objects[1].get_position()));
    }
}

fn vector3_to_vec3(v: &Vector3) -> Vec3 {
    Vec3 {
        x: v.x as f32,
        y: v.y as f32,
        z: v.z as f32,
    }
}