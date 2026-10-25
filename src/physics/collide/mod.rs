pub mod sphere;

use crate::geometry::shape::Shape;

pub trait Collidable {
    fn is_colliding(&self, with: &Shape) -> bool;

    fn collide(&self, with: &Shape);
}