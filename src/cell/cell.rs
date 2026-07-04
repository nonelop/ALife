use crate::cell::ai::Network;
use crate::world::map::{Content, Point, World};

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub energy: f32,
    brain: Network,
}

impl Cell {
    pub fn new(coordinates: (i32, i32)) -> Cell {
        Cell {
            x: coordinates.0,
            y: coordinates.1,
            energy: 100.0,
            brain: Network::new(&[5, 10, 4]),
        }
    }

    fn nearest_food(&self, world: &World, x: i32, y: i32) -> f32 {
        let point = world.get_imut_point(x, y);

        match point.content {
            Content::Empty => return 0.0,
            Content::Food => return 1.0,
            Content::Border => return -1.0,
        }
    }
    pub fn movement(&mut self, world: &mut World) {
        let inputs: &[f32] = &[
            self.energy / 100.0,
            self.nearest_food(world, self.x, self.y + 1),
            self.nearest_food(world, self.x, self.y - 1),
            self.nearest_food(world, self.x - 1, self.y),
            self.nearest_food(world, self.x + 1, self.y),
        ];

        let action = self.brain.decide(inputs);
        let mut step_point: Point = Point {
            x: self.x,
            y: self.y,
            content: Content::Empty,
        };

        match action {
            0 => {
                step_point.y += 1;
                self.energy -= 0.5
            }
            1 => {
                step_point.y -= 1;
                self.energy -= 0.5
            }
            2 => {
                step_point.x -= 1;
                self.energy -= 0.5
            }
            3 => {
                step_point.x += 1;
                self.energy -= 0.5
            }
            _ => println!("ERROR"),
        }

        self.x = step_point.x;
        self.y = step_point.y;

        if world.get_point(self.x, self.y).content == Content::Food {
            self.energy += 5.0;
            world.get_point(self.x, self.y).content = Content::Empty;
        }

    }
}
