// This is just an attempt to make some object based on what already exists.
// it may won't be in the simulation.

use crate::geometry::shape::{GeometryShape, Shape};
use crate::geometry::vector::vector::Vector3;
use crate::geometry::shape::CollidableShape;
use crate::physics::collide::body::CollidableBody;
use crate::physics::moves::body::MoveableBody;
use crate::physics::collide::CollisionInfo;
use crate::body::{Body, PhysicsBody};
use crate::config::DELTA_TIME;

pub struct Atom {
    shape: Shape,
    pub velocity: Vector3,
    density: f64
}

impl Atom {
    pub fn new(shape: Shape) -> Self {
        Atom { shape, velocity: Vector3::zero(), density: 1.0}
    }
}

impl Body for Atom {
    fn get_shape(&self) -> Shape {self.shape}
    fn set_shape(&mut self, shape: Shape) {self.shape = shape}

    fn get_velocity(&self) -> Vector3 {self.velocity}
    fn set_velocity(&mut self, velocity: Vector3) {self.velocity = velocity}

    fn get_mass(&self) -> f64 {
        self.density * self.get_shape().get_volume()
    }
}

impl CollidableBody for Atom {}

impl MoveableBody for Atom {}

impl PhysicsBody for Atom {}

// wrapper for quick access to methods of atom's shape
impl CollidableShape for Atom {
    fn is_colliding(&self, with: &Shape) -> bool {
        self.shape.is_colliding(with)
    }

    fn get_collision_info(&self, with: &Shape) -> Option<CollisionInfo> {
        self.shape.get_collision_info(with)
    }
}