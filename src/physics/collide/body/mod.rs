use crate::body::Body;
use crate::physics::collide::CollisionInfo;

pub trait CollidableBody: Body {
    fn collide(&mut self, with: &mut dyn Body, info: CollisionInfo) {
        let push = info.normal * (info.depth / 2.0);

        self.set_position(self.get_position() - push);
        with.set_position(with.get_position() + push);

        let self_mass = self.get_mass_f64();
        let with_mass = with.get_mass_f64();
        let inv_self_mass =  if self_mass > 0.0 { 1.0 / self_mass } else { 0.0 };
        let inv_with_mass = if with_mass > 0.0 { 1.0 / with_mass} else { 0.0 };
        let inv_sum = inv_self_mass + inv_with_mass;

        if inv_sum <= 0.0 {
            return;
        }

        let self_velocity = self.get_velocity();
        let with_velocity = with.get_velocity();
        let velocity_diff = self_velocity - with_velocity;
        let velocity_along_normal = velocity_diff.x * info.normal.x + velocity_diff.y * info.normal.y + velocity_diff.z * info.normal.z;

        if velocity_along_normal <= 0.0 {
            return;
        }

        let restitution = 1.0;

        let j = -(1.0 + restitution) * velocity_along_normal / inv_sum;
        let impulse = info.normal * j;

        self.set_velocity(self_velocity + impulse * inv_self_mass);
        with.set_velocity(with_velocity - impulse * inv_with_mass);
    }
}