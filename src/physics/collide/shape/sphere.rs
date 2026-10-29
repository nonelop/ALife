use super::CollidableShape;
use crate::geometry::shape::sphere::Sphere;
use crate::geometry::shape::Shape;
use super::CollisionInfo;

impl CollidableShape for Sphere {
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
    fn get_collision_info(&self, with: &Shape) -> Option<CollisionInfo> {
        if self.is_colliding(with) {
            match with {
                Shape::Sphere(sphere) => {
                    let depth = (self.radius + sphere.radius) - (self.center - sphere.center).length();
                    let normal = (self.center - sphere.center).normalize();
                    let contact_point = self.center + (self.radius * normal);

                    Some(CollisionInfo {
                        depth,
                        normal,
                        contact_point
                    })
                }
                Shape::Cube(cube) => {
                    None
                }
            }
        }
        else {
            None
        }
    }
}