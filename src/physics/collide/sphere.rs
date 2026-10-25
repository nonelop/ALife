use super::Collidable;
use crate::geometry::shape::sphere::Sphere;
use crate::geometry::shape::Shape;

impl Collidable for Sphere {
    fn is_colliding(&self, with: &Shape) -> bool {
        match with {
            Shape::Sphere(sphere) => {
                let d = (sphere.center.x - self.center.x).powi(2) + (sphere.center.y - self.center.y).powi(2) + (sphere.center.z - self.center.z).powi(2);
                if d <= (self.radius + sphere.radius).powi(2) {
                    return true;
                }
                else {
                    return false;
                }
            }
            Shape::Cube(cube) => {
                return false; // pass
            }
        }
    }
}