use crate::cell::ai::Network;
use crate::world::map::{Content, World};

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub energy: f32,
    pub neural_network: Network,
}

impl Cell {
    pub fn new(coordinates: (i32, i32)) -> Cell {
        Cell {
            x: coordinates.0,
            y: coordinates.1,
            energy: 100.0,
            neural_network: Network::new(&[9, 20, 8]),
        }
    }

    pub fn movement(&mut self, world: &mut World) {
        let inputs: &[f32] = &[
            self.energy / 100.00,
            world.get_point(self.x, self.y + 1).smell,
            world.get_point(self.x, self.y - 1).smell,
            world.get_point(self.x - 1, self.y).smell,
            world.get_point(self.x + 1, self.y).smell,
            world.get_point(self.x - 1, self.y + 1).smell,
            world.get_point(self.x + 1, self.y + 1).smell,
            world.get_point(self.x + 1, self.y - 1).smell,
            world.get_point(self.x - 1, self.y - 1).smell,
        ];

        let action = self.neural_network.decide(inputs);
        let mut dx = 0;
        let mut dy = 0;

        match action {
            0 => dy = 1,
            1 => dy = -1,
            2 => dx = -1,
            3 => dx = 1,
            4 => {
                dx = -1;
                dy = 1;
            }
            5 => {
                dx = 1;
                dy = 1;
            }
            6 => {
                dx = 1;
                dy = -1;
            }
            7 => {
                dx = -1;
                dy = -1;
            }
            _ => println!("ERROR"),
        }

        self.x += dx;
        self.y += dy;
        self.energy -= 0.5;

        if world.get_point(self.x, self.y).content == Content::Food {
            self.energy += 5.0;
            world.get_point(self.x, self.y).content = Content::Empty;
        }

        world.get_point(self.x, self.y).content = Content::Cell;

    }
}
