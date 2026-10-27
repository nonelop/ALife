use crate::geometry::shape::Shape;
use crate::body::Body;
use crate::physics::collide::CollisionInfo;

pub trait CollidableBody {
    fn collide(&mut self, with: &mut dyn Body, info: CollisionInfo);
}