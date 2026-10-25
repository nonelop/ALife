// This is just an attempt to make some object based on what already exists.
// it may won't be in the simulation.

use crate::geometry::shape::Shape;
use crate::physics::collide::Collidable;

struct Atom {
    shape: Shape,
    velocity: f64
}

impl Collidable for Atom {
    // wrapper for quick access like atom.is_colliding()
    fn is_colliding(&self, with: &Shape) -> bool {
        self.shape.is_colliding(with)
    }

    fn collide(&self, with: &Shape) {
        // I'll write later
    }
}