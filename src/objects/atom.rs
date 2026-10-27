// This is just an attempt to make some object based on what already exists.
// it may won't be in the simulation.

use crate::geometry::shape::Shape;
use crate::geometry::vector::vector::Vector3;
use crate::physics::collide::body::CollidableBody;
use crate::physics::moves::body::MoveableBody;
use crate::physics::collide::shape::CollidableShape;
use crate::physics::collide::CollisionInfo;
use crate::body::Body;
use crate::config::DELTA_TIME;

pub struct Atom {
    shape: Shape,
    pub velocity: Vector3
}

impl Atom {
    pub fn new(shape: Shape) -> Self {
        Atom { shape, velocity: Vector3::zero() }
    }
}

impl Body for Atom {
    fn get_shape(&self) -> Shape {self.shape}
    fn set_shape(&mut self, shape: Shape) {self.shape = shape}

    fn get_velocity(&self) -> Vector3 {self.velocity}
    fn set_velocity(&mut self, velocity: Vector3) {self.velocity = velocity}
}

impl CollidableBody for Atom {}

impl MoveableBody for Atom {
    fn move_with_velocity(&mut self) {
        self.set_position(self.get_position() + (DELTA_TIME * self.get_velocity()));
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