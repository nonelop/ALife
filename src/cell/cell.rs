use crate::cell::ai::Network;
use crate::world::map::{Content, World};

const DIRS: [(i32, i32); 8] = [
    (0, 1), (0, -1), (-1, 0), (1, 0),
    (-1, 1), (1, 1), (1, -1), (-1, -1),
];

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub energy: f32,
    pub neural_network: Network,
}

impl Cell {
    pub fn new((x, y): (i32, i32)) -> Cell {
        Cell { x, y, energy: 100.0, neural_network: Network::new(&[9, 20, 8]) }
    }

    pub fn movement(&mut self, world: &mut World) {
        let mut inputs = vec![self.energy / 100.0];
        for (dx, dy) in DIRS {
            inputs.push(world.get_point(self.x + dx, self.y + dy).smell);
        }

        let (dx, dy) = DIRS[self.neural_network.decide(&inputs)];
        self.x += dx;
        self.y += dy;
        self.energy -= 0.5;

        let point = world.get_point(self.x, self.y);
        if point.content == Content::Food {
            self.energy += 5.0;
        }
        point.content = Content::Cell;
    }
}
