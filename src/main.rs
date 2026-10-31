mod geometry;
mod physics;
mod body;
mod objects;
mod time;
mod errors;
mod config;
mod visual;
mod world;

use kiss3d::prelude::*;

use crate::geometry::unit::GeometryUnits::Meter;
use crate::geometry::vector::Vector3;
use crate::geometry::shape::{Shape, sphere::Sphere};
use crate::objects::atom::Atom;
use crate::config::DELTA_TIME;
use crate::world::World;
use crate::world::BodyId;

#[kiss3d::main]
async fn main() {
    let mut world = World::new();

    let mut window = Window::new("Simulation: AUS").await;
    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty();

    let mut objects: Vec<BodyId> = Vec::new();

    let atom = world.spawn(Box::new(Atom::new(
        Shape::Sphere(Sphere::new(Vector3::new(0.0, 0.25, 0.15), Meter(1.0)).unwrap())
    )));

    let mut atom2 = world.spawn(Box::new(Atom::new(
        Shape::Sphere(Sphere::new(Vector3::new(20.0, 0.0, 0.0), Meter(1.0)).unwrap())
    )));

    objects.push(atom);
    objects.push(atom2);

    world.get_mut_body(atom).unwrap().set_velocity(Vector3::new(4.0, 0.0, 0.0));

    while window.render_3d(&mut scene, &mut camera).await {
        time::tick(DELTA_TIME, &mut world, &mut objects);

        let atom_pos = world.get_mut_body(atom).unwrap().get_position();
        let atom2_pos = world.get_mut_body(atom).unwrap().get_position();

        world.get_mut_node(atom).unwrap().set_position(
            vector3_to_vec3(&atom_pos)
        );
        world.get_mut_node(atom2).unwrap().set_position(
            vector3_to_vec3(&atom2_pos)
        );
    }
}

fn vector3_to_vec3(v: &Vector3) -> Vec3 {
    Vec3 {
        x: v.x as f32,
        y: v.y as f32,
        z: v.z as f32,
    }
}