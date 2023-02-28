// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PER_HOUR: u8 = 221;
const MINUTES_PER_HOUR: u32 = 60;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let i = match speed {
        1 | 2 | 3 | 4 => speed as f64,
        5 | 6 | 7 | 8 => 0.9 * speed as f64,
        9 | 10 => 0.77 * speed as f64,
        _ => 0.0,
    };
    i * CARS_PER_HOUR as f64
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / MINUTES_PER_HOUR
}
