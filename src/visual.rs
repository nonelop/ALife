use kiss3d::prelude::*;

use crate::geometry::shape::{GeometryShape, Shape};
use crate::geometry::vector::Vector3;
use crate::physics::body::PhysicsBody;

pub fn new_scene_node(scene: &mut SceneNode3d, shape: &Shape) {
    match shape {
        Shape::Sphere(sphere) => {
            let mut node = scene.add_sphere(sphere.get_radius() as f32);
            node.set_position(vector3_to_vec3(&shape.get_center()));
        }
        Shape::Cube(cube) => {}
    }
}

pub fn update_scene_nodes(scene: &mut SceneNode3d, objects: &[Box<dyn PhysicsBody>]) {
    let scene_data = scene.data();

    for node in scene_data.children() {

    }
}

fn vector3_to_vec3(v: &Vector3) -> Vec3 {
    Vec3 {
        x: v.x as f32,
        y: v.y as f32,
        z: v.z as f32,
    }
}

pub fn shape_to_node(shape: &Shape) -> Option<SceneNode3d> {
    match shape {
        Shape::Sphere(sphere) => {
            Some(SceneNode3d::sphere(shape.get_radius() as f32))
        }
        _ => None
    }
}