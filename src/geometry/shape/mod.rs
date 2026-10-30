pub mod cube;
pub mod sphere;

use cube::Cube;
use sphere::Sphere;
use crate::geometry::vector::Vector3;
use crate::physics::collide::CollisionInfo;

// enum for group all shapes to one object
#[derive(Clone, Copy, PartialEq)]
pub enum Shape {
    Cube(Cube),
    Sphere(Sphere)
}

// basic shape methods
pub trait GeometryShape {
    fn get_center(&self) -> Vector3;
    fn get_radius(&self) -> f64;
    fn get_volume(&self) -> f64;
}

// methods for shape which can collide with other shapes
pub trait CollidableShape {
    fn is_colliding_with(&self, with: &Shape) -> bool;
    fn get_collision_info_with(&self, with: &Shape) -> Option<CollisionInfo>;
}

// wrapper which links to real implements
impl GeometryShape for Shape {
    fn get_center(&self) -> Vector3 {
        match self {
            Shape::Sphere(sphere) => sphere.get_center(),
            _ => Vector3::zero()
        }
    }
    fn get_radius(&self) -> f64 {
        match self {
            Self::Sphere(sphere) => sphere.get_radius(),
            _ => 0.0
        }
    }
    fn get_volume(&self) -> f64 {
        match self {
            Shape::Sphere(sphere) => sphere.get_volume(),
            _ => 0.0
        }
    }
}

// wrapper which links to real implements
impl CollidableShape for Shape {
    fn is_colliding_with(&self, with: &Shape) -> bool {
        match self {
            Self::Sphere(sphere) => sphere.is_colliding_with(with),
            Self::Cube(_cube) => false
        }
    }
    fn get_collision_info_with(&self, with: &Shape) -> Option<CollisionInfo> {
        match self {
            Self::Sphere(sphere) => sphere.get_collision_info_with(with),
            Self::Cube(_cube) => None
        }
    }
}