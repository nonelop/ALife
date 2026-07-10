mod cell;
mod view;
mod world;
mod time;

use crate::cell::cell::Cell;
use crate::view::render_map;
use crate::world::map::World;
use crate::time::tick::tick;
use std::thread;
use std::time::Duration;

fn main() {
    let mut cell0 = Cell::new((0, 0));

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
