// This is just an attempt to make some object based on what already exists.
// it may won't be in the simulation.

use crate::geometry::shape::Shape;
use crate::geometry::vector::vector::Vector3;
use crate::physics::collide::{CollidableShape, CollisionInfo};

pub struct Atom {
    shape: Shape,
    pub velocity: Vector3
}

impl Atom {
    pub fn new(shape: Shape) -> Self {
        Atom { shape, velocity: Vector3::zero() }
    }
}

impl CollidableShape for Atom {
    // wrapper for quick access like atom.is_colliding()
    fn is_colliding(&self, with: &Shape) -> bool {
        self.shape.is_colliding(with)
    }

    // wrapper for quick access like atom.get_collision_info()
    fn get_collision_info(&self, with: &Shape) -> Option<CollisionInfo> {
        self.shape.get_collision_info(with)
    }
}