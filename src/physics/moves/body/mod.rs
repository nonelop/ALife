use crate::body::Body;
use crate::config::DELTA_TIME;

pub trait MoveableBody: Body {
    fn move_with_velocity(&mut self) {
        self.set_position(self.get_position() + (DELTA_TIME * self.get_velocity()));
    }
}