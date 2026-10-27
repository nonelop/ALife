use crate::config::DELTA_TIME;
use std::thread::sleep;
use std::time::Duration;

pub fn tick() {
    sleep(Duration::from_secs_f64(DELTA_TIME));
}
