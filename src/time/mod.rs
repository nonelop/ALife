use crate::body::{Body, PhysicsBody};
use crate::physics::collide::shape::CollidableShape;
use crate::physics::moves::body::MoveableBody;
use crate::config::DELTA_TIME;
use std::thread::sleep;
use std::time::Duration;

pub fn tick(objects: &mut Vec<Box<dyn PhysicsBody>>) {
    let len = objects.len();

    for obj in &mut *objects {
        obj.move_with_velocity();
    }

    for i in 0..len {
        for j in (i + 1)..len {
            let (left, right) = objects.split_at_mut(j);

            let obj = &mut left[i];
            let with_obj = &mut right[0];

            let shape = obj.get_shape();
            let with_shape = with_obj.get_shape();

            if with_shape != shape {
                if shape.is_colliding(&with_shape) {
                    obj.collide( &mut **with_obj, shape.get_collision_info(&with_shape).unwrap());
                }
            }
        }
    }
    sleep(Duration::from_secs_f64(DELTA_TIME));
}
