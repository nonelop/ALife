// This is just an attempt to make some object based on what already exists.
// it may won't be in the simulation.

use crate::geometry::shape::Shape;
use crate::geometry::vector::vector::Vector3;
use crate::physics::collide::shape::CollidableShape;
use crate::physics::collide::CollisionInfo;

pub struct Atom {
    shape: Shape,
    pub velocity: Vector3
}

impl Atom {
    pub fn new(shape: Shape) -> Self {
        Atom { shape, velocity: Vector3::zero() }
    }

    pub fn set_position(&mut self, position: Vector3) {
        match &mut self.shape {
            Shape::Sphere(sphere) => {
                sphere.center = position
            }
            Shape::Cube(cube) => {}
        }
    }

    pub fn get_position(&self) -> Vector3 {
        match &self.shape {
            Shape::Sphere(sphere) => sphere.center,
            Shape::Cube(cube) => Vector3::zero()
        }
    }
}

// wrapper for quick access to methods of atom's shape
impl CollidableShape for Atom {
    fn is_colliding(&self, with: &Shape) -> bool {
        self.shape.is_colliding(with)
    }

    fn get_collision_info(&self, with: &Shape) -> Option<CollisionInfo> {
        self.shape.get_collision_info(with)
    }
}