use crate::body::Body;

pub trait MoveableBody: Body {
    fn move_with_velocity(&mut self, dt: f64) {
        self.set_position(self.get_position() + (dt * self.get_velocity()));
    }
}