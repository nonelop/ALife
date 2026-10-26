use crate::geometry::shape::Shape;

pub trait CollidableBody {
    fn collide(&mut self, with: &mut Shape);
}