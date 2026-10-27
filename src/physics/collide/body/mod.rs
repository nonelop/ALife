use crate::body::Body;
use crate::physics::collide::CollisionInfo;

pub trait CollidableBody: Body {
    fn collide(&mut self, with: &mut dyn Body, info: CollisionInfo) {
        let push = info.normal * (info.depth / 2.0);

        self.set_position(self.get_position() - push);
        with.set_position(with.get_position() + push);

        let self_velocity = self.get_velocity();
        let with_velocity = with.get_velocity();

        self.set_velocity(with_velocity);
        with.set_velocity(self_velocity);
    }
}