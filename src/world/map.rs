use rand::{self, Rng};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Content {
    Empty,
    Food,
    Border,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub content: Content,
    pub smell: f32,
}

impl Point {
    fn get_points(radius: i32) -> Vec<Point> {
        let mut total_points: Vec<Point> = Vec::new();

        for y in (-radius..=radius).rev() {
            for x in -radius..=radius {
                total_points.push(Point {
                    x,
                    y,
                    content: Content::Empty,
                    smell: 0.0
                });
            }
        }

        total_points
    }
}

pub struct World {
    pub radius: i32,
    pub points: Vec<Point>,
}

impl World {
    pub fn new(radius: i32) -> World {
        let mut world = World {
            radius,
            points: Point::get_points(radius),
        };

        for i in &mut world.points {
            if i.x == -radius || i.x == radius || i.y == -radius || i.y == radius {
                i.content = Content::Border;
            }
        }

        world
    }
    pub fn get_point(&mut self, x: i32, y: i32) -> &mut Point {
        let index = ((self.radius - y) * (self.radius * 2 + 1)).abs() + (x + self.radius);
        &mut self.points[index as usize]
    }
    pub fn get_imut_point(&self, x: i32, y: i32) -> &Point {
        let index = ((self.radius - y) * (self.radius * 2 + 1)).abs() + (x + self.radius);
        &self.points[index as usize]
    }
    pub fn random_food(&mut self) {
        let mut rng = rand::thread_rng();

        let usable_points_quantity = self.points.len() as f32 - self.points.len() as f32 * 0.60;
        let food_quantity = rng.gen_range(1.0..usable_points_quantity) as usize;
        let mut indexes: Vec<usize> = vec![0; food_quantity];

        for i in 0..food_quantity {
            indexes[i] = rng.gen_range(0..self.points.len());
        }

        for i in &indexes {
            if self.points[*i].content == Content::Empty {
                self.points[*i].content = Content::Food;
            }
        }
    }
}
