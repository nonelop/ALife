mod cell;
mod view;
mod world;

use crate::cell::cell::Cell;
use crate::view::render_map;
use crate::world::map::World;
use std::thread;
use std::time::Duration;

fn main() {
    let mut cell0 = Cell::new((0, 0));

    let mut world = World::new(15);
    world.random_food();

    println!("\x1B[2J");

    loop {
        thread::sleep(Duration::from_millis(450));

        println!("\x1B[H");

        render_map(&world);
        world.update();

        cell0.movement(&mut world);
        println!(
            "CELL | X: {} Y: {} | ENERGY: {}",
            cell0.x, cell0.y, cell0.energy
        )
    }
}
