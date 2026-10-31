use super::collide::body::CollidableBody;
use super::moves::body::MoveableBody;
use crate::body::Body;

pub trait PhysicsBody: Body + CollidableBody + MoveableBody {}