mod world;
mod time;
mod config;
mod geometry;

use crate::world::map::World;
use crate::time::tick::tick;
use std::{println, thread};
use std::time::Duration;

use geometry::unit::unit_enum::GeometryUnits;

fn main() {

    let meter = GeometryUnits::Meter(100.0);
    println!("meter: {:?} base: {:?}", meter, meter.to_base_unit());

    let mut world = World::new(15);
    world.random_food();

    loop {
        thread::sleep(Duration::from_millis(450));

        tick(&mut world);
    }
}
