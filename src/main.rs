mod cell;
mod world;
mod time;
mod config;

use crate::cell::cell::Cell;
use crate::world::map::World;
use crate::time::tick::tick;
use std::thread;
use std::time::Duration;

fn main() {
    let cell0 = Cell::new((0, 0));

    let mut world = World::new(15);
    world.random_food();

    println!("\x1B[2J");

    let mut cells: Vec<Cell> = Vec::new();
    cells.push(cell0);

    loop {
        thread::sleep(Duration::from_millis(450));

        tick(&mut world, &mut cells);
    }
}
