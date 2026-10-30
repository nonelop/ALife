use crate::geometry::shape::CollidableShape;
use crate::physics::body::PhysicsBody;


pub fn tick(dt: f64, objects: &mut Vec<Box<dyn PhysicsBody>>) {
    let len = objects.len();

    for obj in &mut *objects {
        obj.move_with_velocity(dt);
    }

    for i in 0..len {
        for j in (i + 1)..len {
            let (left, right) = objects.split_at_mut(j);

            let obj = &mut left[i];
            let with_obj = &mut right[0];

            let shape = obj.get_shape();
            let with_shape = with_obj.get_shape();

            if shape.is_colliding_with(&with_shape) {
                obj.collide( &mut **with_obj, shape.get_collision_info_with(&with_shape).unwrap());
            }
        }
    }
}
