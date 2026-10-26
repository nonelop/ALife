// This is just an attempt to make some object based on what already exists.
// it may won't be in the simulation.

use crate::geometry::shape::Shape;
use crate::physics::collide::{Collidable, CollisionInfo};

struct Atom {
    shape: Shape,
    velocity: f64
}

impl Collidable for Atom {
    // wrapper for quick access like atom.is_colliding()
    fn is_colliding(&self, with: &Shape) -> bool {
        self.shape.is_colliding(with)
    }

    fn get_collision_info(&self, with: &Shape) -> Option<CollisionInfo> {
        self.shape.get_collision_info(with)
    }

    fn collide(&mut self, with: &mut Shape) {
        // I'll write later
    }
}