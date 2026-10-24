mod time;
mod config;
mod geometry;

use crate::time::tick::tick;
use std::{println, thread};
use std::time::Duration;

use geometry::unit::unit_enum::GeometryUnits;

fn main() {

    let meter = GeometryUnits::Meter(100.0);
    println!("meter: {:?} base: {:?}", meter, meter.to_base_unit());

    loop {
        thread::sleep(Duration::from_millis(10));

        tick();
    }
}
